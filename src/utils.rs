

#[derive(Debug, Clone)]
pub enum Or<T, U> {
    This(T),
    That(U),
}
impl<T, U> Or<T, U> {
    pub fn unwrap(self) -> (Option<T>, Option<U>) {
        match self {
            Or::This(x) => return (Some(x), None),
            Or::That(x) => return (None, Some(x)),
        }
    }
}

