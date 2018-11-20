#[macro_use]
extern crate drone_stm32_map_svd;

fn main() {
  drone_stm32_map_svd::generate_reg_map(svd_feature!(), 3, 12);
}
