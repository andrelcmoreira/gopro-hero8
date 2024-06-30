
#[derive(Debug)]
pub struct WifiInfo {
    pub wifi_ssid: String,
    pub wifi_password: String,
}

impl WifiInfo {
    pub fn new(wifi_ssid: String, wifi_password: String) -> Self {
        WifiInfo {
            wifi_ssid,
            wifi_password
        }
    }
}
