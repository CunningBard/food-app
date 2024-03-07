#[derive(Deserialize)] // `Deserialize` is required for `Query`.
pub struct RangeParameters {
    pub start: usize,
    pub end: usize,
}