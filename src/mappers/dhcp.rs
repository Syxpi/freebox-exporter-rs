use async_trait::async_trait;
use log::debug;
use models::{DhcpLease, DynamicDhcpLease, StaticDhcpLease};
use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};
use reqwest::Client;
use std::error::Error;

use crate::core::common::http_client_factory::{AuthenticatedHttpClientFactory, ManagedHttpClient};
use crate::core::common::transport::FreeboxResponse;
use crate::diagnostics::{DryRunOutputWriter, DryRunnable};
use crate::mappers::MetricMap;

pub mod models;
pub mod unittests;

pub struct DhcpMetricMap<'a> {
    factory: &'a AuthenticatedHttpClientFactory<'a>,
    managed_client: Option<ManagedHttpClient>,
    lease_remaining_gauge: IntGaugeVec,
    refresh_time_gauge: IntGaugeVec,
    assign_time_gauge: IntGaugeVec,
}

impl<'a> DhcpMetricMap<'a> {
    pub fn new(factory: &'a AuthenticatedHttpClientFactory<'a>, prefix: String) -> Self {
        let prfx: String = format!("{prefix}_dhcp");

        Self {
            factory,
            managed_client: None,
            lease_remaining_gauge: register_int_gauge_vec!(
                format!("{prfx}_lease_remaining",),
                "Lease remaining time in milliseconds".to_string(),
                &["id", "hostname", "ip", "mac", "is_static"],
            )
            .expect(&format!(
                "Failed to create gauge for {prfx}_lease_remaining"
            )),
            refresh_time_gauge: register_int_gauge_vec!(
                format!("{prfx}_refresh_time"),
                "Refresh time in milliseconds".to_string(),
                &["id", "hostname", "ip", "mac", "is_static"],
            )
            .expect(&format!("Failed to create gauge for {prfx}_refresh_time")),
            assign_time_gauge: register_int_gauge_vec!(
                format!("{prfx}_assign_time"),
                "Assign time in milliseconds".to_string(),
                &["id", "hostname", "ip", "mac", "is_static"],
            )
            .expect(&format!("Failed to create gauge for {prfx}_assign_time")),
        }
    }

    async fn get_managed_client(
        &mut self,
    ) -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
        if self.managed_client.as_ref().is_none() {
            debug!("creating managed client");

            let res = self.factory.create_managed_client().await;

            if res.is_err() {
                debug!("cannot create managed client");

                return Err(res.err().unwrap());
            }

            self.managed_client = Some(res.unwrap());
        }

        let client = self.managed_client.as_ref().clone().unwrap();
        let res = client.get();

        if res.is_ok() {
            return Ok(res.unwrap());
        } else {
            debug!("renewing managed client");

            let client = self.factory.create_managed_client().await;
            self.managed_client = Some(client.unwrap());

            return self.managed_client.as_ref().unwrap().get();
        }
    }

    async fn fetch_dhcp_static_leases(
        &mut self,
    ) -> Result<Vec<StaticDhcpLease>, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.get_managed_client().await?;

        let res = client
            .get(format!("{}v4/dhcp/static_lease/", self.factory.api_url))
            .send()
            .await;

        if let Err(e) = res {
            return Err(Box::new(e));
        }

        let res = res
            .unwrap()
            .json::<FreeboxResponse<Vec<StaticDhcpLease>>>()
            .await;

        if let Err(e) = res {
            return Err(Box::new(e));
        }

        let res = res.unwrap();

        if res.success.unwrap_or_default() {
            Ok(res.result.unwrap_or(vec![]))
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                res.msg.unwrap_or("Unknown error".to_string()),
            )))
        }
    }

    async fn fetch_dhcp_dynamic_leases(
        &mut self,
    ) -> Result<Vec<DynamicDhcpLease>, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.get_managed_client().await?;

        let res = client
            .get(format!("{}v4/dhcp/dynamic_lease/", self.factory.api_url))
            .send()
            .await;

        if let Err(e) = res {
            return Err(Box::new(e));
        }

        let res = res
            .unwrap()
            .json::<FreeboxResponse<Vec<DynamicDhcpLease>>>()
            .await;

        if let Err(e) = res {
            return Err(Box::new(e));
        }

        let res = res.unwrap();

        if res.success.unwrap_or_default() {
            Ok(res.result.unwrap_or(vec![]))
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                res.msg.unwrap_or("Unknown error".to_string()),
            )))
        }
    }

    async fn fetch_dhcp_leases(
        &mut self,
    ) -> Result<Vec<Box<dyn DhcpLease>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut leases: Vec<Box<dyn DhcpLease>> = vec![];

        let dyn_leases = self.fetch_dhcp_dynamic_leases().await;

        if let Err(e) = dyn_leases {
            return Err(e);
        }

        let dynamics = dyn_leases.unwrap();

        for lease in dynamics {
            leases.push(Box::new(lease));
        }

        let sta_leases = self.fetch_dhcp_static_leases().await;

        if let Err(e) = sta_leases {
            return Err(e);
        }

        let sta_leases = sta_leases.unwrap();

        for lease in sta_leases {
            leases.push(Box::new(lease));
        }

        Ok(leases)
    }

    async fn set_all(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let leases = self.fetch_dhcp_leases().await;

        if let Err(e) = leases {
            return Err(e);
        }

        let leases = leases.unwrap();

        for lease in leases {
            self.lease_remaining_gauge
                .with_label_values(&[
                    &lease.get_id().unwrap_or_default(),
                    &lease.get_hostname().unwrap_or_default(),
                    &lease.get_ip().unwrap_or_default(),
                    &lease.get_mac().unwrap_or_default(),
                    &lease.get_is_static().unwrap_or_default().to_string(),
                ])
                .set(lease.get_lease_remaining().unwrap_or_default());

            self.refresh_time_gauge
                .with_label_values(&[
                    &lease.get_id().unwrap_or_default(),
                    &lease.get_hostname().unwrap_or_default(),
                    &lease.get_ip().unwrap_or_default(),
                    &lease.get_mac().unwrap_or_default(),
                    &lease.get_is_static().unwrap_or_default().to_string(),
                ])
                .set(lease.get_refresh_time().unwrap_or_default());

            self.assign_time_gauge
                .with_label_values(&[
                    &lease.get_id().unwrap_or_default(),
                    &lease.get_hostname().unwrap_or_default(),
                    &lease.get_ip().unwrap_or_default(),
                    &lease.get_mac().unwrap_or_default(),
                    &lease.get_is_static().unwrap_or_default().to_string(),
                ])
                .set(lease.get_assign_time().unwrap_or_default() as i64);
        }

        Ok(())
    }

    fn reset_all(&self) {
        self.lease_remaining_gauge.reset();
        self.refresh_time_gauge.reset();
        self.assign_time_gauge.reset();
    }
}

#[async_trait]
impl<'a> MetricMap<'a> for DhcpMetricMap<'a> {
    async fn set(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        self.reset_all();

        if let Err(e) = self.set_all().await {
            return Err(e);
        }

        Ok(())
    }

    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Initialize any necessary state or metrics
        Ok(())
    }
}

#[async_trait]
impl DryRunnable for DhcpMetricMap<'_> {
    fn get_name(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok("dhcp".to_string())
    }

    async fn dry_run(
        &mut self,
        _writer: &mut dyn DryRunOutputWriter,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn as_dry_runnable(&mut self) -> &mut dyn DryRunnable {
        self
    }
}
