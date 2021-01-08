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
