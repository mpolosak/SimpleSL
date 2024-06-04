pub trait ToResult<T, E> {
    fn to_result(self) -> Result<T, E>;
}

impl<T, E> ToResult<T, E> for T {
    fn to_result(self) -> Result<T, E> {
        Ok(self)
    }
}

impl<T, E0, E1: From<E0>> ToResult<T, E1> for Result<T, E0> {
    fn to_result(self) -> Result<T, E1> {
        self.map_err(E1::from)
    }
}
