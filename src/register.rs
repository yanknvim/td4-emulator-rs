pub struct Register {
    pub register_a: u8,
    pub register_b: u8,
    pub carry: bool,
    pub pc: u8,
}

impl Register {
    pub fn new() -> Self {
        Register {
            register_a: 0,
            register_b: 0,
            carry: false,
            pc: 0,
        }
    }
}
