pub enum Action {
    Fold,
    Check,
    Call(u32),
    Raise(u32),
}

pub struct Round {
    is_over: bool,
}

impl Round {
    pub fn new() -> Self {
        Self { is_over: false }
    }

    pub fn end_round(&mut self) {
        self.is_over = true;
    }
}

pub struct Pot {
    total: u32,
}

impl Pot {
    pub fn new() -> Self {
        Self { total: 0 }
    }

    pub fn add(&mut self, amount: u32) {
        self.total += amount;
    }
}
