#[macro_use]
extern crate drone_stm32_map_svd;

fn main() {
  drone_stm32_map_svd::generate_rest(svd_feature!());
}
