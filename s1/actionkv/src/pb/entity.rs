pub trait Namer {
    fn name(&self) -> &'static str;
}
