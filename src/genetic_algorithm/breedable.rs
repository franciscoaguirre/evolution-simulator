pub trait Breedable {
    /// Breed two parents to produce two children.
    fn breed(&self, other: &Self) -> (Self, Self)
    where
        Self: Sized;
}
