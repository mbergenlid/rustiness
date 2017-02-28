pub mod standard_controller;
use memory::MemoryMappedIO;

trait Source<T: MemoryMappedIO> {
    fn load(&self) -> T;
}

