struct Cpu {
    memory: Vec<i16>,
    pc: u16,
    d: i16,
    a: i16,
}

impl Cpu {
    fn m(&self) -> i16 {
        self.memory[self.a as usize]
    }

    fn m_as_mut(&mut self) -> &mut i16 {
        &mut self.memory[self.a as usize]
    }
}
