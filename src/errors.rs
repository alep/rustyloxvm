pub enum Error {
    UnknownOpCode,
}

#[derive(Debug)]
pub struct ScannerError {
    pub message: String,
    pub line: usize,
}
