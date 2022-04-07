#[cfg(test)]
use quickcheck_macros::quickcheck;

//use clp;

#[quickcheck]
fn prop(_xs: Vec<u32>) -> bool {
    true
}
