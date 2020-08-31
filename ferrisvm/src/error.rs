use err_derive::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum FerrisError{
    #[error(display = "StackOverflow")]
    StackOverflow,
    #[error(display = "StackUnderflow")]
    StackUnderflow,
    #[error(display = "PageFault")]
    PageFault,
    #[error(display = "SegFault")]
    SegFault,
    #[error(display = "DoubleFault")]
    DoubleFault,
    #[error(display = "TripleFault")]
    TripleFault,
}