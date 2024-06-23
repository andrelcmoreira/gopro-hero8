use std::io::Error;

use tokio::runtime::Runtime;

use crate::data::factory_info::FactoryInfo;
use crate::data::status_info::StatusInfo;
use crate::data::wifi_info::WifiInfo;
use crate::command::asynchronous as _async;

pub fn get_factory_info() -> Result<FactoryInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let info = _async::get_factory_info()
                    .await?;

                Ok(info)
            }
        )
}

pub fn get_wifi_info() -> Result<WifiInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let info = _async::get_wifi_info()
                    .await?;

                Ok(info)
            }
        )
}

pub fn get_status_info() -> Result<StatusInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let info = _async::get_status_info()
                    .await?;

                Ok(info)
            }
        )
}
