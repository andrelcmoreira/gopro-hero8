use tokio::runtime::Runtime;

use crate::api::asynchronous as _async;
use crate::data::factory_info::FactoryInfo;
use crate::data::status_info::StatusInfo;
use crate::data::wifi_info::WifiInfo;
use crate::error::AppError;

pub fn get_factory_info() -> Result<FactoryInfo, AppError> {
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

pub fn get_wifi_info() -> Result<WifiInfo, AppError> {
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

pub fn get_status_info() -> Result<StatusInfo, AppError> {
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
