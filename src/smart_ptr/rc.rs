struct MyRcRef<T> {
    value: T,
    count: usize,
}

pub struct MyRc<T> {
    inner: *mut MyRcRef<T>,
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

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = MyRcRef { value, count: 1 };
        Self {
            inner: Box::into_raw(Box::new(inner)),
        }
    }

    pub fn get_mut(value: &mut Self) -> Option<&mut T> {
        if unsafe { (*value.inner).count } <= 1 {
            Some(unsafe { &mut (*value.inner).value })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod rc_tests {
    use super::MyRc;

    #[test]
    fn rc_test_1() {
        let val = MyRc::new(5);
        assert_eq!(*val, 5);
    }

    #[test]
    fn rc_test_2() {
        let val_1 = MyRc::new(5);
        let val_2 = val_1.clone();
        assert_eq!(*val_2, 5);
    }

    #[test]
    fn rc_test_3() {
        let mut val_1 = MyRc::new(5);
        assert_eq!(*val_1, 5);
        let val_ref = MyRc::get_mut(&mut val_1);
        *val_ref.unwrap() = 2;
        assert_eq!(*val_1, 2);
    }

    #[test]
    #[should_panic]
    fn rc_test_4() {
        let mut val_1 = MyRc::new(5);
        let val_2 = val_1.clone();
        MyRc::get_mut(&mut val_1).unwrap();
        assert_eq!(*val_2, 5);
    }
}
