pub struct Detail<T>(pub T);

impl<T> Detail<T> {
    pub fn as_ref(&self) -> Detail<&T> {
        Detail(&self.0)
    }

    pub fn map<R>(self, f: impl Fn(T) -> R) -> Detail<R> {
        Detail(f(self.0))
    }
}
