
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal DVH data is not sorted.")]
    DvhUnsorted,
    #[error("DVH data is empty.")]
    DvhNoData,
    #[error("DVH data should have at least 2 data points.")]
    DvhInsufficientData,
    #[error("Error in DVH Dx logic.")]
    DvhDxLogic,
    #[error("Error in DVH Vx logic.")]
    DvhVxLogic,
    #[error("A negative volume value is not valid.")]
    NegativeVolume,
    #[error("A negative dose value is not valid.")]
    NegativeDose,
}

pub type Result<T> = std::result::Result<T, Error>;