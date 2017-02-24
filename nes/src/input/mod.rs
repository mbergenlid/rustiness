pub mod standard_controller;

trait Input {
    fn read(&self) -> u8;
    fn write(&mut self, u8);
}

trait Source<T: Input> {
    fn load(&self) -> T;
}

