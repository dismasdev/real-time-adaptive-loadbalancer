#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendState {
    Healthy = 0,
    Unhealthy = 1,
    Draining = 2,
}

impl From<u8> for BackendState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Healthy,
            1 => Self::Unhealthy,
            _ => Self::Draining, // safe default
        }
    }
}
