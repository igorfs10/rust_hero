use std::ops::RangeInclusive;

use rand::{prelude::*, Rng};
use rand_pcg::Pcg64Mcg;

pub struct RandomValue<T: Random>(T);

pub trait RandomTrait<T> {
    type Input;
    fn get_random_value(seed: &u64, input: Self::Input) -> T;

    // Implementar quando lançarem traits constantes
    // fn get_id(&self) -> usize;
}

pub trait Random {
    type Input;

    fn get_random(seed: &u64, input: Self::Input) -> Self;
}

impl<T> RandomTrait<T> for RandomValue<T>
where
    T: Random,
{
    type Input = T::Input;

    fn get_random_value(seed: &u64, input: Self::Input) -> T {
        T::get_random(seed, input)
    }
}

impl Random for bool {
    type Input = f64;
    fn get_random(seed: &u64, input: Self::Input) -> Self {
        let mut rng = Pcg64Mcg::seed_from_u64(*seed);
        let mut input = input;
        if input > 100.0 {
            input = 100.0;
        } else if input < 0.0 {
            input = 0.0;
        }
        rng.gen_bool(input)
    }
}

impl Random for u8 {
    type Input = RangeInclusive<u8>;

    fn get_random(seed: &u64, input: Self::Input) -> Self {
        let mut rng = Pcg64Mcg::seed_from_u64(*seed);
        rng.gen_range(input)
    }
}

impl Random for u32 {
    type Input = RangeInclusive<u32>;

    fn get_random(seed: &u64, input: Self::Input) -> Self {
        let mut rng = Pcg64Mcg::seed_from_u64(*seed);
        rng.gen_range(input)
    }
}