use thiserror::Error;

#[derive(Error, Debug)]
pub enum HelperError {
    #[error("Uppercase in bucket name")]
    BucketNameError,
}
