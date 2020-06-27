use drone_core::token::Token;
use drone_stm32_map::stm32_reg_tokens;

stm32_reg_tokens! {
    struct Regs;
}

#[test]
#[allow(unused_variables)]
fn periph_macros1() {
    let reg = unsafe { Regs::take() };
    #[cfg(all(
        feature = "adc",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
        )
    ))]
    {
        let adc_com = drone_stm32_map::periph::adc::periph_adc_com!(reg);
    }
    #[cfg(all(
        feature = "adc",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
        )
    ))]
    {
        let adc1 = drone_stm32_map::periph::adc::periph_adc1!(reg);
    }
    #[cfg(all(
        feature = "adc",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let adc2 = drone_stm32_map::periph::adc::periph_adc2!(reg);
        let adc3 = drone_stm32_map::periph::adc::periph_adc3!(reg);
    }
    #[cfg(all(
        feature = "adc",
        any(
            stm32_mcu = "stm32f303",
        )
    ))]
    {
        let adc4 = drone_stm32_map::periph::adc::periph_adc4!(reg);
    }
    #[cfg(all(
        feature = "dma",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let dma1 = drone_stm32_map::periph::dma::periph_dma1!(reg);
        let dma2 = drone_stm32_map::periph::dma::periph_dma2!(reg);
    }
    #[cfg(all(
        feature = "dma",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let dma1_ch0 = drone_stm32_map::periph::dma::periph_dma1_ch0!(reg);
        let dma2_ch0 = drone_stm32_map::periph::dma::periph_dma2_ch0!(reg);
    }
    #[cfg(all(
        feature = "dma",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let dma1_ch1 = drone_stm32_map::periph::dma::periph_dma1_ch1!(reg);
        let dma1_ch2 = drone_stm32_map::periph::dma::periph_dma1_ch2!(reg);
        let dma1_ch3 = drone_stm32_map::periph::dma::periph_dma1_ch3!(reg);
        let dma1_ch4 = drone_stm32_map::periph::dma::periph_dma1_ch4!(reg);
        let dma1_ch5 = drone_stm32_map::periph::dma::periph_dma1_ch5!(reg);
        let dma1_ch6 = drone_stm32_map::periph::dma::periph_dma1_ch6!(reg);
        let dma1_ch7 = drone_stm32_map::periph::dma::periph_dma1_ch7!(reg);
        let dma2_ch1 = drone_stm32_map::periph::dma::periph_dma2_ch1!(reg);
        let dma2_ch2 = drone_stm32_map::periph::dma::periph_dma2_ch2!(reg);
        let dma2_ch3 = drone_stm32_map::periph::dma::periph_dma2_ch3!(reg);
        let dma2_ch4 = drone_stm32_map::periph::dma::periph_dma2_ch4!(reg);
        let dma2_ch5 = drone_stm32_map::periph::dma::periph_dma2_ch5!(reg);
    }
    #[cfg(all(
        feature = "dma",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let dma2_ch6 = drone_stm32_map::periph::dma::periph_dma2_ch6!(reg);
        let dma2_ch7 = drone_stm32_map::periph::dma::periph_dma2_ch7!(reg);
    }
    #[cfg(all(
        feature = "dma",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
        )
    ))]
    {
        let dmamux1 = drone_stm32_map::periph::dma::periph_dmamux1!(reg);
        let dmamux1_ch0 = drone_stm32_map::periph::dma::periph_dmamux1_ch0!(reg);
        let dmamux1_ch1 = drone_stm32_map::periph::dma::periph_dmamux1_ch1!(reg);
        let dmamux1_ch2 = drone_stm32_map::periph::dma::periph_dmamux1_ch2!(reg);
        let dmamux1_ch3 = drone_stm32_map::periph::dma::periph_dmamux1_ch3!(reg);
        let dmamux1_ch4 = drone_stm32_map::periph::dma::periph_dmamux1_ch4!(reg);
        let dmamux1_ch5 = drone_stm32_map::periph::dma::periph_dmamux1_ch5!(reg);
        let dmamux1_ch6 = drone_stm32_map::periph::dma::periph_dmamux1_ch6!(reg);
        let dmamux1_ch7 = drone_stm32_map::periph::dma::periph_dmamux1_ch7!(reg);
        let dmamux1_ch8 = drone_stm32_map::periph::dma::periph_dmamux1_ch8!(reg);
        let dmamux1_ch9 = drone_stm32_map::periph::dma::periph_dmamux1_ch9!(reg);
        let dmamux1_ch10 = drone_stm32_map::periph::dma::periph_dmamux1_ch10!(reg);
        let dmamux1_ch11 = drone_stm32_map::periph::dma::periph_dmamux1_ch11!(reg);
        let dmamux1_ch12 = drone_stm32_map::periph::dma::periph_dmamux1_ch12!(reg);
        let dmamux1_ch13 = drone_stm32_map::periph::dma::periph_dmamux1_ch13!(reg);
        let dmamux1_rg0 = drone_stm32_map::periph::dma::periph_dmamux1_rg0!(reg);
        let dmamux1_rg1 = drone_stm32_map::periph::dma::periph_dmamux1_rg1!(reg);
        let dmamux1_rg2 = drone_stm32_map::periph::dma::periph_dmamux1_rg2!(reg);
        let dmamux1_rg3 = drone_stm32_map::periph::dma::periph_dmamux1_rg3!(reg);
    }
    #[cfg(all(
        feature = "exti",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let exti0 = drone_stm32_map::periph::exti::periph_exti0!(reg);
        let exti1 = drone_stm32_map::periph::exti::periph_exti1!(reg);
        let exti2 = drone_stm32_map::periph::exti::periph_exti2!(reg);
        let exti3 = drone_stm32_map::periph::exti::periph_exti3!(reg);
        let exti4 = drone_stm32_map::periph::exti::periph_exti4!(reg);
        let exti5 = drone_stm32_map::periph::exti::periph_exti5!(reg);
        let exti6 = drone_stm32_map::periph::exti::periph_exti6!(reg);
        let exti7 = drone_stm32_map::periph::exti::periph_exti7!(reg);
        let exti8 = drone_stm32_map::periph::exti::periph_exti8!(reg);
        let exti9 = drone_stm32_map::periph::exti::periph_exti9!(reg);
        let exti10 = drone_stm32_map::periph::exti::periph_exti10!(reg);
        let exti11 = drone_stm32_map::periph::exti::periph_exti11!(reg);
        let exti12 = drone_stm32_map::periph::exti::periph_exti12!(reg);
        let exti13 = drone_stm32_map::periph::exti::periph_exti13!(reg);
        let exti14 = drone_stm32_map::periph::exti::periph_exti14!(reg);
        let exti15 = drone_stm32_map::periph::exti::periph_exti15!(reg);
        let exti16 = drone_stm32_map::periph::exti::periph_exti16!(reg);
        let exti17 = drone_stm32_map::periph::exti::periph_exti17!(reg);
        let exti18 = drone_stm32_map::periph::exti::periph_exti18!(reg);
    }
    #[cfg(all(
        feature = "exti",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let exti19 = drone_stm32_map::periph::exti::periph_exti19!(reg);
        let exti20 = drone_stm32_map::periph::exti::periph_exti20!(reg);
        let exti21 = drone_stm32_map::periph::exti::periph_exti21!(reg);
        let exti22 = drone_stm32_map::periph::exti::periph_exti22!(reg);
    }
    #[cfg(all(
        feature = "exti",
        any(
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let exti23 = drone_stm32_map::periph::exti::periph_exti23!(reg);
    }
    #[cfg(all(
        feature = "exti",
        any(
            stm32_mcu = "stm32f303",
        )
    ))]
    {
        let exti29 = drone_stm32_map::periph::exti::periph_exti29!(reg);
        let exti30 = drone_stm32_map::periph::exti::periph_exti30!(reg);
        let exti31 = drone_stm32_map::periph::exti::periph_exti31!(reg);
        let exti32 = drone_stm32_map::periph::exti::periph_exti32!(reg);
        let exti33 = drone_stm32_map::periph::exti::periph_exti33!(reg);
    }
    #[cfg(all(
        feature = "exti",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let exti24 = drone_stm32_map::periph::exti::periph_exti24!(reg);
        let exti25 = drone_stm32_map::periph::exti::periph_exti25!(reg);
        let exti26 = drone_stm32_map::periph::exti::periph_exti26!(reg);
        let exti27 = drone_stm32_map::periph::exti::periph_exti27!(reg);
        let exti28 = drone_stm32_map::periph::exti::periph_exti28!(reg);
        let exti29 = drone_stm32_map::periph::exti::periph_exti29!(reg);
        let exti30 = drone_stm32_map::periph::exti::periph_exti30!(reg);
        let exti31 = drone_stm32_map::periph::exti::periph_exti31!(reg);
        let exti32 = drone_stm32_map::periph::exti::periph_exti32!(reg);
        let exti33 = drone_stm32_map::periph::exti::periph_exti33!(reg);
        let exti34 = drone_stm32_map::periph::exti::periph_exti34!(reg);
        let exti35 = drone_stm32_map::periph::exti::periph_exti35!(reg);
        let exti36 = drone_stm32_map::periph::exti::periph_exti36!(reg);
        let exti37 = drone_stm32_map::periph::exti::periph_exti37!(reg);
        let exti38 = drone_stm32_map::periph::exti::periph_exti38!(reg);
        let exti39 = drone_stm32_map::periph::exti::periph_exti39!(reg);
    }
    #[cfg(all(
        feature = "exti",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let exti40 = drone_stm32_map::periph::exti::periph_exti40!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_a = drone_stm32_map::periph::gpio::periph_gpio_a!(reg);
        let gpio_b = drone_stm32_map::periph::gpio::periph_gpio_b!(reg);
        let gpio_c = drone_stm32_map::periph::gpio::periph_gpio_c!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_d = drone_stm32_map::periph::gpio::periph_gpio_d!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_e = drone_stm32_map::periph::gpio::periph_gpio_e!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_f = drone_stm32_map::periph::gpio::periph_gpio_f!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_g = drone_stm32_map::periph::gpio::periph_gpio_g!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_h = drone_stm32_map::periph::gpio::periph_gpio_h!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_i = drone_stm32_map::periph::gpio::periph_gpio_i!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let gpio_j = drone_stm32_map::periph::gpio::periph_gpio_j!(reg);
        let gpio_k = drone_stm32_map::periph::gpio::periph_gpio_k!(reg);
    }
    #[cfg(all(
        feature = "i2c",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let i2c1 = drone_stm32_map::periph::i2c::periph_i2c1!(reg);
        let i2c2 = drone_stm32_map::periph::i2c::periph_i2c2!(reg);
    }
    #[cfg(all(
        feature = "i2c",
        any(
            stm32_mcu = "stm32f303",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let i2c3 = drone_stm32_map::periph::i2c::periph_i2c3!(reg);
    }
    #[cfg(all(
        feature = "i2c",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let i2c4 = drone_stm32_map::periph::i2c::periph_i2c4!(reg);
    }
    #[cfg(all(
        feature = "rtc",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let rtc = drone_stm32_map::periph::rtc::periph_rtc!(reg);
    }
    #[cfg(all(
        feature = "spi",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let spi1 = drone_stm32_map::periph::spi::periph_spi1!(reg);
        let spi2 = drone_stm32_map::periph::spi::periph_spi2!(reg);
    }
    #[cfg(all(
        feature = "spi",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let spi3 = drone_stm32_map::periph::spi::periph_spi3!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim1 = drone_stm32_map::periph::tim::periph_tim1!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim2 = drone_stm32_map::periph::tim::periph_tim2!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim3 = drone_stm32_map::periph::tim::periph_tim3!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim4 = drone_stm32_map::periph::tim::periph_tim4!(reg);
        let tim5 = drone_stm32_map::periph::tim::periph_tim5!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim6 = drone_stm32_map::periph::tim::periph_tim6!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim7 = drone_stm32_map::periph::tim::periph_tim7!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim8 = drone_stm32_map::periph::tim::periph_tim8!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let tim9 = drone_stm32_map::periph::tim::periph_tim9!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let tim10 = drone_stm32_map::periph::tim::periph_tim10!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let tim11 = drone_stm32_map::periph::tim::periph_tim11!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let tim12 = drone_stm32_map::periph::tim::periph_tim12!(reg);
        let tim13 = drone_stm32_map::periph::tim::periph_tim13!(reg);
        let tim14 = drone_stm32_map::periph::tim::periph_tim14!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim15 = drone_stm32_map::periph::tim::periph_tim15!(reg);
        let tim16 = drone_stm32_map::periph::tim::periph_tim16!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let tim17 = drone_stm32_map::periph::tim::periph_tim17!(reg);
    }
    #[cfg(all(
        feature = "tim",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let lptim1 = drone_stm32_map::periph::tim::periph_lptim1!(reg);
        let lptim2 = drone_stm32_map::periph::tim::periph_lptim2!(reg);
    }
    #[cfg(all(
        feature = "uart",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let usart1 = drone_stm32_map::periph::uart::periph_usart1!(reg);
        let usart2 = drone_stm32_map::periph::uart::periph_usart2!(reg);
        let usart3 = drone_stm32_map::periph::uart::periph_usart3!(reg);
    }
    #[cfg(all(
        feature = "uart",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let uart4 = drone_stm32_map::periph::uart::periph_uart4!(reg);
    }
    #[cfg(all(
        feature = "uart",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let uart5 = drone_stm32_map::periph::uart::periph_uart5!(reg);
    }
    #[cfg(all(
        feature = "uart",
        any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let lpuart1 = drone_stm32_map::periph::uart::periph_lpuart1!(reg);
    }
}

#[test]
#[allow(unused_variables)]
fn periph_macros2() {
    let reg = unsafe { Regs::take() };
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_a_head = drone_stm32_map::periph::gpio::periph_gpio_a_head!(reg);
        let gpio_a0 = drone_stm32_map::periph::gpio::periph_gpio_a0!(reg);
        let gpio_a1 = drone_stm32_map::periph::gpio::periph_gpio_a1!(reg);
        let gpio_a2 = drone_stm32_map::periph::gpio::periph_gpio_a2!(reg);
        let gpio_a3 = drone_stm32_map::periph::gpio::periph_gpio_a3!(reg);
        let gpio_a4 = drone_stm32_map::periph::gpio::periph_gpio_a4!(reg);
        let gpio_a5 = drone_stm32_map::periph::gpio::periph_gpio_a5!(reg);
        let gpio_a6 = drone_stm32_map::periph::gpio::periph_gpio_a6!(reg);
        let gpio_a7 = drone_stm32_map::periph::gpio::periph_gpio_a7!(reg);
        let gpio_a8 = drone_stm32_map::periph::gpio::periph_gpio_a8!(reg);
        let gpio_a9 = drone_stm32_map::periph::gpio::periph_gpio_a9!(reg);
        let gpio_a10 = drone_stm32_map::periph::gpio::periph_gpio_a10!(reg);
        let gpio_a11 = drone_stm32_map::periph::gpio::periph_gpio_a11!(reg);
        let gpio_a12 = drone_stm32_map::periph::gpio::periph_gpio_a12!(reg);
        let gpio_a13 = drone_stm32_map::periph::gpio::periph_gpio_a13!(reg);
        let gpio_a14 = drone_stm32_map::periph::gpio::periph_gpio_a14!(reg);
        let gpio_a15 = drone_stm32_map::periph::gpio::periph_gpio_a15!(reg);
        let gpio_b_head = drone_stm32_map::periph::gpio::periph_gpio_b_head!(reg);
        let gpio_b0 = drone_stm32_map::periph::gpio::periph_gpio_b0!(reg);
        let gpio_b1 = drone_stm32_map::periph::gpio::periph_gpio_b1!(reg);
        let gpio_b2 = drone_stm32_map::periph::gpio::periph_gpio_b2!(reg);
        let gpio_b3 = drone_stm32_map::periph::gpio::periph_gpio_b3!(reg);
        let gpio_b4 = drone_stm32_map::periph::gpio::periph_gpio_b4!(reg);
        let gpio_b5 = drone_stm32_map::periph::gpio::periph_gpio_b5!(reg);
        let gpio_b6 = drone_stm32_map::periph::gpio::periph_gpio_b6!(reg);
        let gpio_b7 = drone_stm32_map::periph::gpio::periph_gpio_b7!(reg);
        let gpio_b8 = drone_stm32_map::periph::gpio::periph_gpio_b8!(reg);
        let gpio_b9 = drone_stm32_map::periph::gpio::periph_gpio_b9!(reg);
        let gpio_b10 = drone_stm32_map::periph::gpio::periph_gpio_b10!(reg);
        let gpio_b11 = drone_stm32_map::periph::gpio::periph_gpio_b11!(reg);
        let gpio_b12 = drone_stm32_map::periph::gpio::periph_gpio_b12!(reg);
        let gpio_b13 = drone_stm32_map::periph::gpio::periph_gpio_b13!(reg);
        let gpio_b14 = drone_stm32_map::periph::gpio::periph_gpio_b14!(reg);
        let gpio_b15 = drone_stm32_map::periph::gpio::periph_gpio_b15!(reg);
        let gpio_c_head = drone_stm32_map::periph::gpio::periph_gpio_c_head!(reg);
        let gpio_c0 = drone_stm32_map::periph::gpio::periph_gpio_c0!(reg);
        let gpio_c1 = drone_stm32_map::periph::gpio::periph_gpio_c1!(reg);
        let gpio_c2 = drone_stm32_map::periph::gpio::periph_gpio_c2!(reg);
        let gpio_c3 = drone_stm32_map::periph::gpio::periph_gpio_c3!(reg);
        let gpio_c4 = drone_stm32_map::periph::gpio::periph_gpio_c4!(reg);
        let gpio_c5 = drone_stm32_map::periph::gpio::periph_gpio_c5!(reg);
        let gpio_c6 = drone_stm32_map::periph::gpio::periph_gpio_c6!(reg);
        let gpio_c7 = drone_stm32_map::periph::gpio::periph_gpio_c7!(reg);
        let gpio_c8 = drone_stm32_map::periph::gpio::periph_gpio_c8!(reg);
        let gpio_c9 = drone_stm32_map::periph::gpio::periph_gpio_c9!(reg);
        let gpio_c10 = drone_stm32_map::periph::gpio::periph_gpio_c10!(reg);
        let gpio_c11 = drone_stm32_map::periph::gpio::periph_gpio_c11!(reg);
        let gpio_c12 = drone_stm32_map::periph::gpio::periph_gpio_c12!(reg);
        let gpio_c13 = drone_stm32_map::periph::gpio::periph_gpio_c13!(reg);
        let gpio_c14 = drone_stm32_map::periph::gpio::periph_gpio_c14!(reg);
        let gpio_c15 = drone_stm32_map::periph::gpio::periph_gpio_c15!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f102",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_d_head = drone_stm32_map::periph::gpio::periph_gpio_d_head!(reg);
        let gpio_d0 = drone_stm32_map::periph::gpio::periph_gpio_d0!(reg);
        let gpio_d1 = drone_stm32_map::periph::gpio::periph_gpio_d1!(reg);
        let gpio_d2 = drone_stm32_map::periph::gpio::periph_gpio_d2!(reg);
        let gpio_d3 = drone_stm32_map::periph::gpio::periph_gpio_d3!(reg);
        let gpio_d4 = drone_stm32_map::periph::gpio::periph_gpio_d4!(reg);
        let gpio_d5 = drone_stm32_map::periph::gpio::periph_gpio_d5!(reg);
        let gpio_d6 = drone_stm32_map::periph::gpio::periph_gpio_d6!(reg);
        let gpio_d7 = drone_stm32_map::periph::gpio::periph_gpio_d7!(reg);
        let gpio_d8 = drone_stm32_map::periph::gpio::periph_gpio_d8!(reg);
        let gpio_d9 = drone_stm32_map::periph::gpio::periph_gpio_d9!(reg);
        let gpio_d10 = drone_stm32_map::periph::gpio::periph_gpio_d10!(reg);
        let gpio_d11 = drone_stm32_map::periph::gpio::periph_gpio_d11!(reg);
        let gpio_d12 = drone_stm32_map::periph::gpio::periph_gpio_d12!(reg);
        let gpio_d13 = drone_stm32_map::periph::gpio::periph_gpio_d13!(reg);
        let gpio_d14 = drone_stm32_map::periph::gpio::periph_gpio_d14!(reg);
        let gpio_d15 = drone_stm32_map::periph::gpio::periph_gpio_d15!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f107",
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_e_head = drone_stm32_map::periph::gpio::periph_gpio_e_head!(reg);
        let gpio_e0 = drone_stm32_map::periph::gpio::periph_gpio_e0!(reg);
        let gpio_e1 = drone_stm32_map::periph::gpio::periph_gpio_e1!(reg);
        let gpio_e2 = drone_stm32_map::periph::gpio::periph_gpio_e2!(reg);
        let gpio_e3 = drone_stm32_map::periph::gpio::periph_gpio_e3!(reg);
        let gpio_e4 = drone_stm32_map::periph::gpio::periph_gpio_e4!(reg);
        let gpio_e5 = drone_stm32_map::periph::gpio::periph_gpio_e5!(reg);
        let gpio_e6 = drone_stm32_map::periph::gpio::periph_gpio_e6!(reg);
        let gpio_e7 = drone_stm32_map::periph::gpio::periph_gpio_e7!(reg);
        let gpio_e8 = drone_stm32_map::periph::gpio::periph_gpio_e8!(reg);
        let gpio_e9 = drone_stm32_map::periph::gpio::periph_gpio_e9!(reg);
        let gpio_e10 = drone_stm32_map::periph::gpio::periph_gpio_e10!(reg);
        let gpio_e11 = drone_stm32_map::periph::gpio::periph_gpio_e11!(reg);
        let gpio_e12 = drone_stm32_map::periph::gpio::periph_gpio_e12!(reg);
        let gpio_e13 = drone_stm32_map::periph::gpio::periph_gpio_e13!(reg);
        let gpio_e14 = drone_stm32_map::periph::gpio::periph_gpio_e14!(reg);
        let gpio_e15 = drone_stm32_map::periph::gpio::periph_gpio_e15!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f100",
            stm32_mcu = "stm32f101",
            stm32_mcu = "stm32f103",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_f_head = drone_stm32_map::periph::gpio::periph_gpio_f_head!(reg);
        let gpio_f0 = drone_stm32_map::periph::gpio::periph_gpio_f0!(reg);
        let gpio_f1 = drone_stm32_map::periph::gpio::periph_gpio_f1!(reg);
        let gpio_f2 = drone_stm32_map::periph::gpio::periph_gpio_f2!(reg);
        let gpio_f3 = drone_stm32_map::periph::gpio::periph_gpio_f3!(reg);
        let gpio_f4 = drone_stm32_map::periph::gpio::periph_gpio_f4!(reg);
        let gpio_f5 = drone_stm32_map::periph::gpio::periph_gpio_f5!(reg);
        let gpio_f6 = drone_stm32_map::periph::gpio::periph_gpio_f6!(reg);
        let gpio_f7 = drone_stm32_map::periph::gpio::periph_gpio_f7!(reg);
        let gpio_f8 = drone_stm32_map::periph::gpio::periph_gpio_f8!(reg);
        let gpio_f9 = drone_stm32_map::periph::gpio::periph_gpio_f9!(reg);
        let gpio_f10 = drone_stm32_map::periph::gpio::periph_gpio_f10!(reg);
        let gpio_f11 = drone_stm32_map::periph::gpio::periph_gpio_f11!(reg);
        let gpio_f12 = drone_stm32_map::periph::gpio::periph_gpio_f12!(reg);
        let gpio_f13 = drone_stm32_map::periph::gpio::periph_gpio_f13!(reg);
        let gpio_f14 = drone_stm32_map::periph::gpio::periph_gpio_f14!(reg);
        let gpio_f15 = drone_stm32_map::periph::gpio::periph_gpio_f15!(reg);
        let gpio_g_head = drone_stm32_map::periph::gpio::periph_gpio_g_head!(reg);
        let gpio_g0 = drone_stm32_map::periph::gpio::periph_gpio_g0!(reg);
        let gpio_g1 = drone_stm32_map::periph::gpio::periph_gpio_g1!(reg);
        let gpio_g2 = drone_stm32_map::periph::gpio::periph_gpio_g2!(reg);
        let gpio_g3 = drone_stm32_map::periph::gpio::periph_gpio_g3!(reg);
        let gpio_g4 = drone_stm32_map::periph::gpio::periph_gpio_g4!(reg);
        let gpio_g5 = drone_stm32_map::periph::gpio::periph_gpio_g5!(reg);
        let gpio_g6 = drone_stm32_map::periph::gpio::periph_gpio_g6!(reg);
        let gpio_g7 = drone_stm32_map::periph::gpio::periph_gpio_g7!(reg);
        let gpio_g8 = drone_stm32_map::periph::gpio::periph_gpio_g8!(reg);
        let gpio_g9 = drone_stm32_map::periph::gpio::periph_gpio_g9!(reg);
        let gpio_g10 = drone_stm32_map::periph::gpio::periph_gpio_g10!(reg);
        let gpio_g11 = drone_stm32_map::periph::gpio::periph_gpio_g11!(reg);
        let gpio_g12 = drone_stm32_map::periph::gpio::periph_gpio_g12!(reg);
        let gpio_g13 = drone_stm32_map::periph::gpio::periph_gpio_g13!(reg);
        let gpio_g14 = drone_stm32_map::periph::gpio::periph_gpio_g14!(reg);
        let gpio_g15 = drone_stm32_map::periph::gpio::periph_gpio_g15!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_h_head = drone_stm32_map::periph::gpio::periph_gpio_h_head!(reg);
        let gpio_h0 = drone_stm32_map::periph::gpio::periph_gpio_h0!(reg);
        let gpio_h1 = drone_stm32_map::periph::gpio::periph_gpio_h1!(reg);
        let gpio_h2 = drone_stm32_map::periph::gpio::periph_gpio_h2!(reg);
        let gpio_h3 = drone_stm32_map::periph::gpio::periph_gpio_h3!(reg);
        let gpio_h4 = drone_stm32_map::periph::gpio::periph_gpio_h4!(reg);
        let gpio_h5 = drone_stm32_map::periph::gpio::periph_gpio_h5!(reg);
        let gpio_h6 = drone_stm32_map::periph::gpio::periph_gpio_h6!(reg);
        let gpio_h7 = drone_stm32_map::periph::gpio::periph_gpio_h7!(reg);
        let gpio_h8 = drone_stm32_map::periph::gpio::periph_gpio_h8!(reg);
        let gpio_h9 = drone_stm32_map::periph::gpio::periph_gpio_h9!(reg);
        let gpio_h10 = drone_stm32_map::periph::gpio::periph_gpio_h10!(reg);
        let gpio_h11 = drone_stm32_map::periph::gpio::periph_gpio_h11!(reg);
        let gpio_h12 = drone_stm32_map::periph::gpio::periph_gpio_h12!(reg);
        let gpio_h13 = drone_stm32_map::periph::gpio::periph_gpio_h13!(reg);
        let gpio_h14 = drone_stm32_map::periph::gpio::periph_gpio_h14!(reg);
        let gpio_h15 = drone_stm32_map::periph::gpio::periph_gpio_h15!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9",
            stm32_mcu = "stm32l4x6",
        )
    ))]
    {
        let gpio_i_head = drone_stm32_map::periph::gpio::periph_gpio_i_head!(reg);
        let gpio_i0 = drone_stm32_map::periph::gpio::periph_gpio_i0!(reg);
        let gpio_i1 = drone_stm32_map::periph::gpio::periph_gpio_i1!(reg);
        let gpio_i2 = drone_stm32_map::periph::gpio::periph_gpio_i2!(reg);
        let gpio_i3 = drone_stm32_map::periph::gpio::periph_gpio_i3!(reg);
        let gpio_i4 = drone_stm32_map::periph::gpio::periph_gpio_i4!(reg);
        let gpio_i5 = drone_stm32_map::periph::gpio::periph_gpio_i5!(reg);
        let gpio_i6 = drone_stm32_map::periph::gpio::periph_gpio_i6!(reg);
        let gpio_i7 = drone_stm32_map::periph::gpio::periph_gpio_i7!(reg);
        let gpio_i8 = drone_stm32_map::periph::gpio::periph_gpio_i8!(reg);
        let gpio_i9 = drone_stm32_map::periph::gpio::periph_gpio_i9!(reg);
        let gpio_i10 = drone_stm32_map::periph::gpio::periph_gpio_i10!(reg);
        let gpio_i11 = drone_stm32_map::periph::gpio::periph_gpio_i11!(reg);
        let gpio_i12 = drone_stm32_map::periph::gpio::periph_gpio_i12!(reg);
        let gpio_i13 = drone_stm32_map::periph::gpio::periph_gpio_i13!(reg);
        let gpio_i14 = drone_stm32_map::periph::gpio::periph_gpio_i14!(reg);
        let gpio_i15 = drone_stm32_map::periph::gpio::periph_gpio_i15!(reg);
    }
    #[cfg(all(
        feature = "gpio",
        any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f469",
        )
    ))]
    {
        let gpio_j_head = drone_stm32_map::periph::gpio::periph_gpio_j_head!(reg);
        let gpio_j0 = drone_stm32_map::periph::gpio::periph_gpio_j0!(reg);
        let gpio_j1 = drone_stm32_map::periph::gpio::periph_gpio_j1!(reg);
        let gpio_j2 = drone_stm32_map::periph::gpio::periph_gpio_j2!(reg);
        let gpio_j3 = drone_stm32_map::periph::gpio::periph_gpio_j3!(reg);
        let gpio_j4 = drone_stm32_map::periph::gpio::periph_gpio_j4!(reg);
        let gpio_j5 = drone_stm32_map::periph::gpio::periph_gpio_j5!(reg);
        let gpio_j6 = drone_stm32_map::periph::gpio::periph_gpio_j6!(reg);
        let gpio_j7 = drone_stm32_map::periph::gpio::periph_gpio_j7!(reg);
        let gpio_j8 = drone_stm32_map::periph::gpio::periph_gpio_j8!(reg);
        let gpio_j9 = drone_stm32_map::periph::gpio::periph_gpio_j9!(reg);
        let gpio_j10 = drone_stm32_map::periph::gpio::periph_gpio_j10!(reg);
        let gpio_j11 = drone_stm32_map::periph::gpio::periph_gpio_j11!(reg);
        let gpio_j12 = drone_stm32_map::periph::gpio::periph_gpio_j12!(reg);
        let gpio_j13 = drone_stm32_map::periph::gpio::periph_gpio_j13!(reg);
        let gpio_j14 = drone_stm32_map::periph::gpio::periph_gpio_j14!(reg);
        let gpio_j15 = drone_stm32_map::periph::gpio::periph_gpio_j15!(reg);
        let gpio_k_head = drone_stm32_map::periph::gpio::periph_gpio_k_head!(reg);
        let gpio_k0 = drone_stm32_map::periph::gpio::periph_gpio_k0!(reg);
        let gpio_k1 = drone_stm32_map::periph::gpio::periph_gpio_k1!(reg);
        let gpio_k2 = drone_stm32_map::periph::gpio::periph_gpio_k2!(reg);
        let gpio_k3 = drone_stm32_map::periph::gpio::periph_gpio_k3!(reg);
        let gpio_k4 = drone_stm32_map::periph::gpio::periph_gpio_k4!(reg);
        let gpio_k5 = drone_stm32_map::periph::gpio::periph_gpio_k5!(reg);
        let gpio_k6 = drone_stm32_map::periph::gpio::periph_gpio_k6!(reg);
        let gpio_k7 = drone_stm32_map::periph::gpio::periph_gpio_k7!(reg);
        let gpio_k8 = drone_stm32_map::periph::gpio::periph_gpio_k8!(reg);
        let gpio_k9 = drone_stm32_map::periph::gpio::periph_gpio_k9!(reg);
        let gpio_k10 = drone_stm32_map::periph::gpio::periph_gpio_k10!(reg);
        let gpio_k11 = drone_stm32_map::periph::gpio::periph_gpio_k11!(reg);
        let gpio_k12 = drone_stm32_map::periph::gpio::periph_gpio_k12!(reg);
        let gpio_k13 = drone_stm32_map::periph::gpio::periph_gpio_k13!(reg);
        let gpio_k14 = drone_stm32_map::periph::gpio::periph_gpio_k14!(reg);
        let gpio_k15 = drone_stm32_map::periph::gpio::periph_gpio_k15!(reg);
    }
}
