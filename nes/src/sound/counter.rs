extern crate rand;
pub struct ClockTester<T> {
    pub cycles: u64,
    period: u32,
    clocked_object: T,
}

impl<T> ClockTester<T> {
    pub fn new(object: T, period: u32) -> ClockTester<T> {
        ClockTester {
            cycles: 0,
            period: period,
            clocked_object: object,
        }
    }

    pub fn count_down<CFn, F1, F2>(&mut self, mut clock_fn: CFn, step: &F1, finished: &F2)
    where
        CFn: FnMut(&mut T, u8),
        F1: Fn(&T, u64),
        F2: Fn(&T, u64),
    {
        let period = self.period;
        let clocks_required = self.cycles + (period as u64 - (self.cycles % period as u64));
        while self.cycles < clocks_required {
            let tick = rand::random::<u8>() % 10;
            clock_fn(&mut self.clocked_object, tick);
            self.cycles += tick as u64;
            if self.cycles >= clocks_required {
                finished(&self.clocked_object, self.cycles);
            } else {
                step(&self.clocked_object, self.cycles);
            }
        }
    }
}
