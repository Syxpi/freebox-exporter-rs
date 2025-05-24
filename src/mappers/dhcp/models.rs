use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StaticDhcpLease {
    pub id: Option<String>,
    pub hostname: Option<String>,
    pub ip: Option<String>,
    pub mac: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DynamicDhcpLease {
    pub id: Option<String>,
    pub hostname: Option<String>,
    pub ip: Option<String>,
    pub mac: Option<String>,
    pub assign_time: Option<u64>,
    pub lease_remaining: Option<u64>,
    pub refresh_time: Option<u64>,
}

pub trait DhcpLease: std::fmt::Debug + Send {
    fn get_id(&self) -> Option<String>;
    fn get_hostname(&self) -> Option<String>;
    fn get_ip(&self) -> Option<String>;
    fn get_mac(&self) -> Option<String>;
    fn get_is_static(&self) -> Option<bool>;
    fn get_lease_remaining(&self) -> Option<i64>;
    fn get_assign_time(&self) -> Option<u64>;
    fn get_refresh_time(&self) -> Option<i64>;
}

impl DhcpLease for StaticDhcpLease {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn get_hostname(&self) -> Option<String> {
        self.hostname.clone()
    }

    fn get_ip(&self) -> Option<String> {
        self.ip.clone()
    }

    fn get_mac(&self) -> Option<String> {
        self.mac.clone()
    }

    fn get_is_static(&self) -> Option<bool> {
        Some(true)
    }

    fn get_lease_remaining(&self) -> Option<i64> {
        Some(-1)
    }

    fn get_assign_time(&self) -> Option<u64> {
        Some(0)
    }

    fn get_refresh_time(&self) -> Option<i64> {
        Some(-1)
    }
}

impl DhcpLease for DynamicDhcpLease {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn get_hostname(&self) -> Option<String> {
        self.hostname.clone()
    }

    fn get_ip(&self) -> Option<String> {
        self.ip.clone()
    }

    fn get_mac(&self) -> Option<String> {
        self.mac.clone()
    }

    fn get_is_static(&self) -> Option<bool> {
        Some(false)
    }

    fn get_lease_remaining(&self) -> Option<i64> {
        self.lease_remaining.clone().map(|v| v as i64)
    }

    fn get_assign_time(&self) -> Option<u64> {
        self.assign_time.clone()
    }

    fn get_refresh_time(&self) -> Option<i64> {
        self.refresh_time.clone().map(|v| v as i64)
    }
}
