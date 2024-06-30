
#[derive(Debug)]
pub struct FactoryInfo {
    pub hw_revision: String,
    pub fw_revision: String,
    pub sw_revision: String,
    pub serial_number: String,
    pub model_number: String,
    pub manufacturer_name: String,
}

impl FactoryInfo {
    pub fn new(hw_revision: String, fw_revision: String,
               sw_revision: String, serial_number: String,
               model_number: String, manufacturer_name: String) -> Self {
        FactoryInfo {
            hw_revision,
            fw_revision,
            sw_revision,
            serial_number,
            model_number,
            manufacturer_name,
        }
    }
}
