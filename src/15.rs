use std::ops::{Add, Sub, Mul};

trait MyTrait {
    fn my_method(&self) -> &Self;
}

impl<T> MyTrait for T where T: Add + Sub + Mul {
    fn my_method(&self) -> &Self {
        self + self - self * self
    }
}
