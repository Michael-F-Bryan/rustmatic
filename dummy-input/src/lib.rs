use rustmatic_core::{
    Device, DeviceError, DeviceManager, InputNumber, OutputNumber,
};
use std::{
    fmt::{self, Display, Formatter},
    sync::Arc,
    time::Instant,
};

#[derive(Clone)]
pub struct DummyInput {
    input_number: InputNumber,
    value: bool,
    created: Instant,
}

impl DummyInput {
    pub fn new(input_number: InputNumber) -> Self {
        DummyInput {
            input_number,
            value: false,
            created: Instant::now(),
        }
    }
}

impl Device<bool> for DummyInput {
    fn read(&self) -> Result<bool, DeviceError> {
        let elapsed = self.created.elapsed().as_secs();

        if elapsed % 2 != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn write(&self, new_state: bool) -> Result<(), DeviceError> {
        unimplemented! {}
    }
}

impl Display for DummyInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DummyInput value:s {}", self.value)
    }
}
