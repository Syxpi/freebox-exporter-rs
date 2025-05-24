#[cfg(test)]
mod non_reg_tests {
    use regex::Regex;

    use crate::{
        core::common::transport::FreeboxResponse,
        mappers::switch::{models::SwitchPortStatus, SwitchMetricMap},
    };

    // https://github.com/shackerd/freebox-exporter-rs/issues/90
    #[test]
    fn poc_malformed_mac_list() {
        // The output error described in the issue shows that their is a panic when trying to deserialize a sequence, the only sequence in the payload is the mac_list field

        // this is a payload with a malformed mac_list field, it contains an empty object {} instead of an array [] as it should be in the response
        // c.f. https://dev.freebox.fr/sdk/os/switch/#SwitchPortStatus
        let payload = r#"{"success":true,"result":[{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 1","link":"up","id":1,"mode":"100BaseTX-FD","speed":"100","rrd_id":"1"},{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 2","link":"up","id":2,"mode":"100BaseTX-FD","speed":"100","rrd_id":"2"},{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 3","link":"up","id":3,"mode":"1000BaseT-FD","speed":"1000","rrd_id":"3"},{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 4","link":"up","id":4,"mode":"100BaseTX-FD","speed":"100","rrd_id":"4"},{"duplex":"half","name":"Freeplug","link":"down","id":5,"mode":"10BaseT-HD","speed":"10","rrd_id":"freeplug"},{"duplex":"auto","mac_list":{},"name":"Sfp lan","link":"down","id":6,"mode":"1000BaseT-FD","speed":"1000","rrd_id":"sfp_lan"}]}"#;

        let regex = Regex::new(r#""mac_list"[^\[]+\{\s{0,}}"#).unwrap();
        let fixed_results = regex.replace_all(payload, r#""mac_list":[]"#).to_string();

        let res =
            match serde_json::from_str::<FreeboxResponse<Vec<SwitchPortStatus>>>(&fixed_results) {
                Err(e) => {
                    println!("{:?}", e);
                    panic!()
                }
                Ok(r) => r,
            };

        if !res.success.unwrap_or(false) {
            panic!()
        }

        match res.result {
            None => panic!(),
            Some(r) => {
                assert!(!r
                    .last()
                    .unwrap()
                    .to_owned()
                    .mac_list
                    .unwrap()
                    .iter()
                    .any(|x| x.mac.is_some()));
            }
        }
    }

    #[test]
    fn should_handle_malformed_mac_list_test() {
        let payload = r#"{"success":true,"result":[{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 1","link":"up","id":1,"mode":"100BaseTX-FD","speed":"100","rrd_id":"1"},{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 2","link":"up","id":2,"mode":"100BaseTX-FD","speed":"100","rrd_id":"2"},{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"},{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 3","link":"up","id":3,"mode":"1000BaseT-FD","speed":"1000","rrd_id":"3"},{"duplex":"full","mac_list":[{"mac":"xx:xx:xx:xx:xx:xx","hostname":"some device :)"}],"name":"Ethernet 4","link":"up","id":4,"mode":"100BaseTX-FD","speed":"100","rrd_id":"4"},{"duplex":"half","name":"Freeplug","link":"down","id":5,"mode":"10BaseT-HD","speed":"10","rrd_id":"freeplug"},{"duplex":"auto","mac_list":{},"name":"Sfp lan","link":"down","id":6,"mode":"1000BaseT-FD","speed":"1000","rrd_id":"sfp_lan"}]}"#;
        let res = SwitchMetricMap::handle_malformed_mac_list(payload);
        assert!(res.is_ok());

        // check is the replacement is done correctly
        let reg = Regex::new(r#""mac_list".+\[\s{0,}\]"#).unwrap();
        assert!(reg.is_match(&res.unwrap()));
    }
}
