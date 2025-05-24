use async_trait::async_trait;

use crate::{
    core::common::transport::FreeboxResponseError,
    diagnostics::{DryRunOutputWriter, DryRunnable},
};

use super::SwitchMetricMap;

#[async_trait]
impl DryRunnable for SwitchMetricMap<'_> {
    fn get_name(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("switch".to_string())
    }

    async fn dry_run(
        &mut self,
        writer: &mut dyn DryRunOutputWriter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let statuses = self.get_ports_status_json().await;

        if statuses.is_err() {
            return Err(Box::new(FreeboxResponseError::new(
                "v4/switch/status/ failed".to_string(),
            )));
        }

        let statuses = statuses.unwrap();

        let _ = writer.push("switch", "status", &statuses);

        let port_statuses = match self.get_ports_status(&statuses).await {
            Err(e) => return Err(e),
            Ok(r) => r,
        };

        let mut i = 0;
        let len = port_statuses.len();

        let _ = writer.push("switch", "stats", "[");

        for port_status in port_statuses {
            let body_stats = self.get_port_stats_json(&port_status).await;

            if body_stats.is_err() {
                return Err(Box::new(FreeboxResponseError::new(
                    "v4/switch/port/{}/stats failed".to_string(),
                )));
            }

            let body_stats = body_stats.unwrap();

            if body_stats == "" {
                continue;
            }

            let _ = writer.push("switch", "stats", &body_stats);

            i += 1;

            if i < len {
                let _ = writer.push("switch", "stats", ",");
            }
        }

        let _ = writer.push("switch", "stats", "]");

        Ok(())
    }

    fn as_dry_runnable(&mut self) -> &mut dyn DryRunnable {
        self
    }
}
