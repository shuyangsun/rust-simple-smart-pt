use crate::smart_ptr::cell::MyCell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
pub enum ReferenceState {
    Exclusive,
    Unshared,
    Shared(usize),
}

/// Simple implementation mimicking std::cell::RefCell.
/// ```rust
/// use rust_simple_smart_pt::smart_ptr::refcell::MyRefCell;
///
/// let v = MyRefCell::new(vec![0, 1, 2]);
/// {
///     let v_ref = v.borrow();
///     println!("v.len() = {}", v_ref.len());
/// }
/// v.borrow_mut().push(3);
/// let v_ref_2 = v.borrow();
/// println!("v.len() = {}", v_ref_2.len());
/// ```
pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    rc: MyCell<ReferenceState>,
}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            rc: MyCell::new(ReferenceState::Unshared),
        }
    }

    pub fn borrow(&self) -> MyRef<'_, T> {
        match self.rc.get() {
            ReferenceState::Exclusive => {
                // Alternative solutions to panic would be returning Result or Option, however an
                // incorrect borrow is not something the programmer will like be able to recover
                // from. The only solution to "recover" from incorrect borrow behavior would be
                // fixing it in the code, which means making it correct during compile-time. So
                // panic is the best solution here.
                panic!("Cannot borrow variable owned exclusively elsewhere.")
            }
            ReferenceState::Unshared => {
                self.rc.set(ReferenceState::Shared(1));
                MyRef::new(&self)
            }
            ReferenceState::Shared(count) => {
                self.rc.set(ReferenceState::Shared(count + 1));
                MyRef::new(&self)
            }
        }
    }

    pub fn borrow_mut(&self) -> MyRefMut<'_, T> {
        match self.rc.get() {
            ReferenceState::Unshared => {
                self.rc.set(ReferenceState::Exclusive);
                MyRefMut::new(&self)
            }
            _ => panic!("Cannot borrow variable shared elsewhere as mutable reference."),
        }
    }
}

pub struct MyRef<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

pub struct MyRefMut<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

impl<T> Drop for MyRef<'_, T> {
    fn drop(&mut self) {
        println!("Dropping MyRef");
        match self.refcell.rc.get() {
            ReferenceState::Exclusive => {
                panic!("Cannot drop reference when there is an exclusive reference.")
            }
            ReferenceState::Unshared => panic!("Cannot drop unshared reference."),
            ReferenceState::Shared(count) => {
                let new_count = count - 1;
                self.refcell.rc.set(if new_count <= 0 {
                    ReferenceState::Unshared
                } else {
                    ReferenceState::Shared(new_count)
                })
            }
        }
    }
}

impl<T> Drop for MyRefMut<'_, T> {
    fn drop(&mut self) {
        println!("Dropping MyRefMut");
        match self.refcell.rc.get() {
            ReferenceState::Exclusive => self.refcell.rc.set(ReferenceState::Unshared),
            ReferenceState::Unshared => panic!("Cannot drop unshared reference."),
            ReferenceState::Shared(_) => panic!("Cannot drop reference when shared."),
        }
    }
}

impl<T> std::ops::Deref for MyRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> std::ops::Deref for MyRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for MyRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_ref_mut()
    }
}

impl<'refcell, T> MyRef<'refcell, T> {
    pub fn new(cell: &'refcell MyRefCell<T>) -> Self {
        println!("Initializing MyRef");
        Self { refcell: cell }
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'refcell, T> MyRefMut<'refcell, T> {
    pub fn new(cell: &'refcell MyRefCell<T>) -> Self {
        println!("Initializing MyRefMut");
        Self { refcell: cell }
    }

    pub fn as_ref_mut(&self) -> &mut T {
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod refcell_test {
    use super::MyRefCell;

    #[test]
    fn test_refcell_1() {
        let string = MyRefCell::new(String::from("hello"));
        {
            assert_eq!("hello", string.borrow().as_ref().as_str());
        }
        string.borrow_mut().as_ref_mut().remove(0);
        assert_eq!("ello", string.borrow().as_ref().as_str());
    }

    #[test]
    fn test_refcell_2() {
        let mut raw_string = String::from("hello");
        let string = MyRefCell::new(&mut raw_string);
        {
            assert_eq!("hello", string.borrow().as_str());
        }
        string.borrow_mut().remove(0);
        assert_eq!("ello", string.borrow().as_str());
    }
}
