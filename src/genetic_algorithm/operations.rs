pub trait Breedable {
    /// Breed two parents to produce two children.
    fn breed(&self, other: &Self) -> (Self, Self)
    where
        Self: Sized;
}

pub trait Crossable {
    /// Cross two crossable objects to create a child
    fn cross(&self, other: &Self) -> Self;
}

pub trait Mutable {
    /// Mutate the object
    fn mutate(&self, mutation_rate: f32) -> Self;
}
