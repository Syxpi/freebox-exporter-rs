use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct SwitchPortStatus {
    pub id: Option<i16>,
    pub link: Option<String>,
    pub speed: Option<String>,
    pub mac_list: Option<Vec<SwitchPortHost>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SwitchPortStats {
    pub rx_packets_rate: Option<i64>,
    pub rx_good_bytes: Option<i64>,
    pub rx_oversize_packets: Option<i64>,
    pub rx_unicast_packets: Option<i64>,
    pub tx_bytes_rate: Option<i64>,
    pub tx_unicast_packets: Option<i64>,
    pub rx_bytes_rate: Option<i64>,
    pub tx_packets: Option<i64>,
    pub tx_collisions: Option<i64>,
    pub tx_packets_rate: Option<i64>,
    pub tx_fcs: Option<i64>,
    pub tx_bytes: Option<i64>,
    pub rx_jabber_packets: Option<i64>,
    pub tx_single: Option<i64>,
    pub tx_excessive: Option<i64>,
    pub rx_pause: Option<i64>,
    pub rx_multicast_packets: Option<i64>,
    pub tx_pause: Option<i64>,
    pub rx_good_packets: Option<i64>,
    pub rx_broadcast_packets: Option<i64>,
    pub tx_multiple: Option<i64>,
    pub tx_deferred: Option<i64>,
    pub tx_late: Option<i64>,
    pub tx_multicast_packets: Option<i64>,
    pub rx_fcs_packets: Option<i64>,
    pub tx_broadcast_packets: Option<i64>,
    pub rx_err_packets: Option<i64>,
    pub rx_fragments_packets: Option<i64>,
    pub rx_bad_bytes: Option<i64>,
    pub rx_undersize_packets: Option<i64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SwitchPortHost {
    pub mac: Option<String>,
    pub hostname: Option<String>,
}
