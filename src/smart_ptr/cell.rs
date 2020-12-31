use std::cell::UnsafeCell;

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

/// Simple implementation for mimicking std::cell::Cell.
/// ```rust
/// use rust_simple_smart_pt::smart_ptr::cell::MyCell;
///
/// let x = MyCell::new(0);
/// println!("x = {}", x.get());
/// x.set(x.get() + 1);
/// println!("x = {}", x.get());
/// ```
impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // Since no operation exposes any type of reference of "value", it is safe to directly
        // return a copy.
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        // "value" field is of type UnsafeCell<T>, which does not implement Sync, so instances of
        // MyCell cannot be shared across multiple threads.
        unsafe { *self.value.get() = value };
    }
}

#[cfg(test)]
mod cell_test {
    use super::MyCell;

    #[test]
    fn cell_test_1() {
        let x = MyCell::new(0);
        x.set(x.get() + 1);
        let y = x.get();
        assert_eq!(y, 1);
    }
}
