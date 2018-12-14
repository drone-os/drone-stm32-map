#[macro_use]
extern crate drone_stm32_map_svd;

fn main() {
  drone_stm32_map_svd::generate_regs(svd_feature!(), 12, 12);
}
