pub struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod box_test {
    use super::MyBox;

    #[test]
    fn test_box_1() {
        let x = MyBox::new(5);
        assert_eq!(*x, 5);
    }

    #[test]
    fn test_box_2() {
        let x = MyBox::new(String::from("Hello, world!"));
        assert_eq!(x.as_str(), "Hello, world!");
    }

    #[test]
    fn test_box_3() {
        fn hello(str: &str) {
            println!("{}", str);
        }
        let x = MyBox::new(String::from("Hello, world!"));
        hello(&x); // x can be used like a normal reference.
    }
}
