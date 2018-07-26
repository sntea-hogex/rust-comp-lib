#[derive()]
struct NewType<T>(T);

impl<T> std::ops::Deref for NewType<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for NewType<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}