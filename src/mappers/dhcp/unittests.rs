#[cfg(test)]
mod tests {
    use crate::{
        core::common::transport::FreeboxResponse,
        mappers::{
            api_specs_provider::get_specs_data,
            dhcp::models::{DynamicDhcpLease, StaticDhcpLease},
        },
    };
    use serde_json::from_str;

    #[tokio::test]
    async fn test_deserialize_dhcp_static_leases() {
        let json_data = get_specs_data("dhcp", "api_v2_dhcp_static_lease-get")
            .await
            .unwrap();

        let data: Result<FreeboxResponse<Vec<StaticDhcpLease>>, _> = from_str(&json_data);

        if let Ok(e) = &data {
            println!("{:?}", e);
        }

        assert!(data.is_ok());
    }

    #[tokio::test]
    async fn test_deserialize_dhcp_dynamic_leases() {
        let json_data = get_specs_data("dhcp", "api_v2_dhcp_dynamic_lease-get")
            .await
            .unwrap();

        let data: Result<FreeboxResponse<Vec<DynamicDhcpLease>>, _> = from_str(&json_data);

        if let Ok(e) = &data {
            println!("{:?}", e);
        }

        assert!(data.is_ok());
    }
}
