use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The bluetooth adapter is not available")]
    BluetoothNotAvailable,
    #[error("The camera was not found")]
    CameraNotFound,
    #[error("Fail to connect to the camera")]
    ConnectionFailure,
    #[error("Fail to scan for devices")]
    ScanError
}
