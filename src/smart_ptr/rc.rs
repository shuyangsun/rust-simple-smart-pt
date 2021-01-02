struct MyRcRef<T> {
    value: T,
    count: usize,
}

pub struct MyRc<T> {
    inner: *const MyRcRef<T>,
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        let mut inner = self.inner;
        unsafe {
            (*inner).count += 1;
        }
        MyRc { inner: self.inner }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { &(*self.inner) }.value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.inner).count -= 1;
        }
    }
}

#[cfg(test)]
mod rc_tests {
    use super::MyRc;

    #[test]
    fn rc_test_1() {}
}
