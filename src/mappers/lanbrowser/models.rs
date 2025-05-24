use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct LanBrowserInterface {
    pub name: Option<String>,
    pub host_count: Option<i32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LanHost {
    pub id: Option<String>,
    pub primary_name: Option<String>,
    pub host_type: Option<String>,
    pub primary_name_manual: Option<bool>,
    pub l2ident: Option<LanHostL2Ident>,
    pub vendor_name: Option<String>,
    pub active: Option<bool>,
    pub last_activity: Option<i64>,
    pub names: Option<Vec<LanHostName>>,
    pub l3connectivities: Option<Vec<LanHostL3Connectivity>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LanHostName {
    pub name: Option<String>,
    pub source: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LanHostL2Ident {
    pub id: Option<String>,
    #[serde(alias = "type")]
    pub _type: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LanHostL3Connectivity {
    pub addr: Option<String>,
    pub af: Option<String>,
    pub active: Option<bool>,
}
