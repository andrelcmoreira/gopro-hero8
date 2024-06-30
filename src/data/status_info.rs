
#[derive(Debug)]
pub struct StatusInfo {
    pub battery_level: u8,
    pub tx_power_level: u8,
//    characteristic_cfg: String,
//    unknown_field: String // TODO: rename it
}

impl StatusInfo {
    pub fn new(battery_level: u8, tx_power_level: u8) -> Self {
        StatusInfo {
            battery_level,
            tx_power_level
        }
    }
}
