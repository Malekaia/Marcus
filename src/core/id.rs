use rand::{Rng, rngs::ThreadRng};

// 10 digit random number
pub fn random_10_digit() -> i32 {
  let mut range: ThreadRng = rand::thread_rng();
  range.gen_range(1000000000..2000000000)
}
