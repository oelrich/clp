#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

//use clp;

#[quickcheck]
fn prop(_xs: Vec<u32>) -> bool {
  true
}
