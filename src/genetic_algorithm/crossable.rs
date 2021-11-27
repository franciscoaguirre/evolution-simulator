pub trait Crossable {
    /// Cross two crossable objects to create a child
    fn cross(&self, other: &Self) -> Self;
}
