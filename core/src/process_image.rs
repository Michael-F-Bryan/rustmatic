use crate::{InputNumber, OutputNumber};
use anymap::AnyMap;
use slotmap::DenseSlotMap;
use std::{cell::RefCell, marker::PhantomData};

// TODO: InputChannels and Outputchannels are very similar. Should "Channels" be
// generic?
pub struct InputChannels<T>(DenseSlotMap<InputNumber, T>);

impl<T> Default for InputChannels<T> {
    fn default() -> InputChannels<T> { InputChannels(DenseSlotMap::with_key()) }
}

pub struct OutputChannels<T>(DenseSlotMap<OutputNumber, T>);

impl<T> Default for OutputChannels<T> {
    fn default() -> OutputChannels<T> {
        OutputChannels(DenseSlotMap::with_key())
    }
}

pub struct ProcessImage {
    input_channels: RefCell<AnyMap>,
    output_channels: RefCell<AnyMap>,
}

/// Handle to an input in the [ProcessImage]
#[derive(Copy, Clone)]
pub struct Input<T> {
    input_number: InputNumber,
    of_type: PhantomData<T>,
}

impl <T>Input<T> {
    pub fn get_number(self) -> InputNumber {
        self.input_number
    }
}

/// Handle to an output in the [ProcessImage]
pub struct Output<T> {
    input_number: OutputNumber,
    of_type: PhantomData<T>,
}

impl ProcessImage {
    pub fn new() -> Self {
        ProcessImage {
            input_channels: RefCell::new(AnyMap::new()),
            output_channels: RefCell::new(AnyMap::new()),
        }
    }

    pub fn register_input<T: 'static>(&self, input: T) -> Input<T> {
        let mut channels = self.input_channels.borrow_mut();

        let id = channels
            .entry::<InputChannels<T>>()
            .or_insert_with(InputChannels::<T>::default)
            .0
            .insert(input);

        Input {
            input_number: id,
            of_type: PhantomData,
        }
    }

    pub fn read<T: 'static + Copy>(&self, input: Input<T>) -> T {
        // Get handle to slotmap of correct type
        let channels = self.input_channels.borrow_mut();

        let value =
            channels
                .get::<InputChannels<T>>()
                .and_then(|input_channels| {
                    input_channels.0.get(input.input_number)
                });

        *value.unwrap()
    }

    pub fn write<T: 'static + Copy>(&self, input: Input<T>, state: T) {
        let mut channels = self.input_channels.borrow_mut();

        let value =
            channels
                .get_mut::<InputChannels<T>>()
                .and_then(|input_channels| {
                    input_channels.0.get_mut(input.input_number)
                });

        *value.unwrap() = state;
    }
}
