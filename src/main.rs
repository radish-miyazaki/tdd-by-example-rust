struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Self {
        Self { amount }
    }

    fn times(&mut self, multiplier: i32) {
        self.amount *= multiplier;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_test() {
        let mut five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
