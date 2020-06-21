pub struct OrderLog<T> {
    orders: Vec<Option<T>>,
    current: usize,
}

impl<T: Clone> OrderLog<T> {
    pub fn new(capacity: usize) -> Self {
        OrderLog {
            orders: vec![None; capacity],
            current: 0,
        }
    }

    pub fn record(&mut self, order: T) {
        self.orders[self.current] = Some(order);
        self.current = (self.current + 1) % self.orders.len();
    }

    pub fn get_last(&self, idx: usize) -> Option<T> {
        let idx = (self.current + self.orders.len() - idx - 1) % self.orders.len();
        self.orders[idx].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut log = OrderLog::new(4);
        assert_eq!(None, log.get_last(0));
        log.record(1);
        assert_eq!(Some(1), log.get_last(0));
        assert_eq!(None, log.get_last(1));
        log.record(2);
        log.record(3);
        log.record(4);
        assert_eq!(Some(1), log.get_last(3));
        log.record(5);
        assert_eq!(Some(2), log.get_last(3));
    }
}
