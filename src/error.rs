pub trait ResultExt<T, E> {
    fn handle(self) -> T;
}
impl<T, E: std::fmt::Debug> ResultExt<T, E> for Result<T, E> {
    fn handle(self) -> T {
        match self {
            Ok(value) => value,
            Err(err) => panic!("{:?}", err),
        }
    }
}