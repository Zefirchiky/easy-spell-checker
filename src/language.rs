pub trait Language: Default + Sync {
    fn name() -> &'static str;
    fn full_name() -> &'static str;
}
