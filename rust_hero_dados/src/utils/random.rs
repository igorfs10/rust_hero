use nanorand::{WyRand, RNG};

pub struct RandomValue<T: Random>(T);

pub trait RandomTrait<T> {
    fn get_random_value(seed: &u64, percentage: u64) -> T;

    // Implementar quando lançarem traits constantes
    // fn get_id(&self) -> usize;
}

pub trait Random {
    fn get_random(seed: &u64, percentage: u64) -> Self;
}

impl<T> RandomTrait<T> for RandomValue<T>
where
    T: Random,
{
    fn get_random_value(seed: &u64, percentage: u64) -> T {
        T::get_random(seed, percentage)
    }
}

impl Random for bool {
    fn get_random(seed: &u64, percentage: u64) -> Self {
        let mut rng = WyRand::new_seed(*seed);
        if percentage > 100 {
            percentage = 100;
        }
        rng.generate_range::<u64>(1, 101) <= percentage
    }
}
