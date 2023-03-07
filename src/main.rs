#[derive(Eq, PartialEq)]
struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Self {
        Self { amount }
    }

    fn times(&self, multiplier: i32) -> Self {
        Self {
            amount: self.amount * multiplier,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dollar_multiple_test() {
        let five = Dollar::new(5);
        let product = five.times(2);
        assert_eq!(10, product.amount);
        let product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn dollar_equality_test() {
        assert!(Dollar::new(5) == Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6))
    }
}
