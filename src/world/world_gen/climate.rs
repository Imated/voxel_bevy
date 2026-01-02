use std::ops::Range;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Climate {
    pub temperature: Range<f32>,
    pub humidity: Range<f32>,
}
