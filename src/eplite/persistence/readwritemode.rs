/// Read and write modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadWriteMode {
    Legacy,
    WriteAheadLog,
}
