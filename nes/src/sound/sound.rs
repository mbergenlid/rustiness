pub trait Pulse {
    fn get(&self) -> &[i16];
}

pub trait AudioDevice {
    fn play(&self, &Pulse);
}
