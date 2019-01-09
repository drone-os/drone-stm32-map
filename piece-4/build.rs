fn main() {
  drone_stm32_map_svd::generate_regs(
    drone_stm32_map_svd::svd_feature!(),
    4,
    12,
  );
}
