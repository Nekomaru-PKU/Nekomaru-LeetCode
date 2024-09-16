use core::ops::Rem;

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Default + Copy + Ord + Rem<Output = T> {
    let zero = T::default();
    debug_assert!(a > zero);
    debug_assert!(b > zero);
    if    a < b    { (a, b) = (b, a); }
    while b > zero { (a, b) = (b, a % b); }
    a
}
