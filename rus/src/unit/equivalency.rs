use std::marker::PhantomData;

pub struct Equivalency<Kind1, Kind2, F1: Fn(f32) -> f32, F2: Fn(f32) -> f32> {
    forward: F1,
    backward: F2,
    _kind_marker: PhantomData<(Kind1, Kind2)>,
}

impl<Kind1, Kind2, F1: Fn(f32) -> f32, F2: Fn(f32) -> f32> Equivalency<Kind1, Kind2, F1, F2> {
    pub fn forward(&self, input: f32) -> f32 {
        self.forward(input)
    }

    pub fn backward(&self, input: f32) -> f32 {
        self.backward(input)
    }
}

trait DualEndedEquivalency<Kind1, Kind2> {
    fn forward(&self, input: f32) -> f32;
    fn backward(&self, input: f32) -> f32;
}

impl<Kind1, Kind2, F1: Fn(f32) -> f32, F2: Fn(f32) -> f32> DualEndedEquivalency<Kind1, Kind2>
    for Equivalency<Kind1, Kind2, F1, F2>
{
    fn forward(&self, input: f32) -> f32 {
        self.forward(input)
    }

    fn backward(&self, input: f32) -> f32 {
        self.backward(input)
    }
}

impl<Kind1, Kind2, F1: Fn(f32) -> f32, F2: Fn(f32) -> f32> DualEndedEquivalency<Kind2, Kind1>
    for Equivalency<Kind1, Kind2, F1, F2>
{
    fn forward(&self, input: f32) -> f32 {
        self.backward(input)
    }

    fn backward(&self, input: f32) -> f32 {
        self.forward(input)
    }
}
