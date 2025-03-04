use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The bluetooth adapter is not available")]
    BluetoothNotAvailable,
    #[error("The camera was not found")]
    CameraNotFound,
    #[error("Failed to connect to the camera")]
    ConnectionFailure,
    #[error("Failed to lookup for devices")]
    DeviceLookupError
}
