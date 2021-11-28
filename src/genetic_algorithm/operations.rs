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
    /// mutation_rate is the amount of mutation to apply
    fn mutate(&self, mutation_rate: f32) -> Self;
}

pub trait Correctable {
    /// Correct the object
    /// This is used to correct the object after a mutation
    /// to avoid the object to be out of bounds
    fn correct(&mut self);

    /// Checks if the object is correct
    fn is_correct(&self) -> bool;
}

pub trait RandomCreatable {
    /// Create a random object
    fn random() -> Self;
}

pub trait Individual: Breedable + Mutable + Correctable + RandomCreatable + Clone {}