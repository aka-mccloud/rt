#![no_std]

use core::{panic::PanicInfo, slice};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

#[derive(Clone, Copy)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

impl Vector {
    #[inline]
    const fn reserved() -> Self {
        Vector { reserved: 0 }
    }

    #[inline]
    const fn handler(func: unsafe extern "C" fn()) -> Self {
        Vector { handler: func }
    }
}

#[no_mangle]
extern "C" fn __default_handler() {
    loop {
    }
}

#[no_mangle]
extern "C" fn __reset_handler() -> ! {
    init_bss_memory();

    extern "C" {
        fn __main() -> !;
    }

    unsafe { __main() }
}

#[inline]
pub fn heap_start() -> *mut u32 {
    unsafe { &mut __heep_start }
}

fn init_bss_memory() {
    extern "C" {
        static mut __start_bss: u32;
        static mut __end_bss: u32;

        static mut __start_data: u32;
        static mut __end_data: u32;

        static __sidata: u32;
    }

    /* Init .bss memory */
    unsafe {
        let start_bss = &mut __start_bss as *mut u32;
        let end_bss = &__end_bss as *const u32;
        let count = end_bss.offset_from(start_bss) as usize;
        slice::from_raw_parts_mut(start_bss, count).fill(0);

        let start_data = &mut __start_data as *mut u32;
        let end_data = &__end_data as *const u32;
        let count = end_data.offset_from(start_data) as usize;

        let start_idata = &__sidata as *const u32;

        slice
            ::from_raw_parts_mut(start_data, count)
            .copy_from_slice(slice::from_raw_parts(start_idata, count));
    }
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = __reset_handler;

#[no_mangle]
#[link_section = ".vector_table.exceptions"]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector::handler(__non_maskable_interrupt),
    // Exception 3: Hard Fault Interrupt.
    Vector::handler(__hard_fault),
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Vector::handler(__memory_management_fault),
    #[cfg(armv6m)] Vector::reserved(),
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Vector::handler(__bus_fault),
    #[cfg(armv6m)] Vector::reserved(),
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Vector::handler(__usage_fault),
    #[cfg(armv6m)] Vector::reserved(),
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    #[cfg(armv8m)] Vector::handler(__secure_fault),
    #[cfg(not(armv8m))] Vector::reserved(),
    // 8-10: Reserved
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    // Exception 11: SV Call Interrupt.
    Vector::handler(__sv_call),
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Vector::handler(__debug_monitor),
    #[cfg(armv6m)] Vector::reserved(),
    // 13: Reserved
    Vector::reserved(),
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector::handler(__pend_sv),
    // Exception 15: System Tick Interrupt.
    Vector::handler(__sys_tick),
];

#[no_mangle]
#[link_section = ".vector_table.interrupts"]
pub static __INTERRUPTS: [Vector; 240] = [
    Vector::handler(__irq0_handler),
    Vector::handler(__irq1_handler),
    Vector::handler(__irq2_handler),
    Vector::handler(__irq3_handler),
    Vector::handler(__irq4_handler),
    Vector::handler(__irq5_handler),
    Vector::handler(__irq6_handler),
    Vector::handler(__irq7_handler),
    Vector::handler(__irq8_handler),
    Vector::handler(__irq9_handler),
    Vector::handler(__irq10_handler),
    Vector::handler(__irq11_handler),
    Vector::handler(__irq12_handler),
    Vector::handler(__irq13_handler),
    Vector::handler(__irq14_handler),
    Vector::handler(__irq15_handler),
    Vector::handler(__irq16_handler),
    Vector::handler(__irq17_handler),
    Vector::handler(__irq18_handler),
    Vector::handler(__irq19_handler),
    Vector::handler(__irq20_handler),
    Vector::handler(__irq21_handler),
    Vector::handler(__irq22_handler),
    Vector::handler(__irq23_handler),
    Vector::handler(__irq24_handler),
    Vector::handler(__irq25_handler),
    Vector::handler(__irq26_handler),
    Vector::handler(__irq27_handler),
    Vector::handler(__irq28_handler),
    Vector::handler(__irq29_handler),
    Vector::handler(__irq30_handler),
    Vector::handler(__irq31_handler),
    Vector::handler(__irq32_handler),
    Vector::handler(__irq33_handler),
    Vector::handler(__irq34_handler),
    Vector::handler(__irq35_handler),
    Vector::handler(__irq36_handler),
    Vector::handler(__irq37_handler),
    Vector::handler(__irq38_handler),
    Vector::handler(__irq39_handler),
    Vector::handler(__irq40_handler),
    Vector::handler(__irq41_handler),
    Vector::handler(__irq42_handler),
    Vector::handler(__irq43_handler),
    Vector::handler(__irq44_handler),
    Vector::handler(__irq45_handler),
    Vector::handler(__irq46_handler),
    Vector::handler(__irq47_handler),
    Vector::handler(__irq48_handler),
    Vector::handler(__irq49_handler),
    Vector::handler(__irq50_handler),
    Vector::handler(__irq51_handler),
    Vector::handler(__irq52_handler),
    Vector::handler(__irq53_handler),
    Vector::handler(__irq54_handler),
    Vector::handler(__irq55_handler),
    Vector::handler(__irq56_handler),
    Vector::handler(__irq57_handler),
    Vector::handler(__irq58_handler),
    Vector::handler(__irq59_handler),
    Vector::handler(__irq60_handler),
    Vector::handler(__irq61_handler),
    Vector::handler(__irq62_handler),
    Vector::handler(__irq63_handler),
    Vector::handler(__irq64_handler),
    Vector::handler(__irq65_handler),
    Vector::handler(__irq66_handler),
    Vector::handler(__irq67_handler),
    Vector::handler(__irq68_handler),
    Vector::handler(__irq69_handler),
    Vector::handler(__irq70_handler),
    Vector::handler(__irq71_handler),
    Vector::handler(__irq72_handler),
    Vector::handler(__irq73_handler),
    Vector::handler(__irq74_handler),
    Vector::handler(__irq75_handler),
    Vector::handler(__irq76_handler),
    Vector::handler(__irq77_handler),
    Vector::handler(__irq78_handler),
    Vector::handler(__irq79_handler),
    Vector::handler(__irq80_handler),
    Vector::handler(__irq81_handler),
    Vector::handler(__irq82_handler),
    Vector::handler(__irq83_handler),
    Vector::handler(__irq84_handler),
    Vector::handler(__irq85_handler),
    Vector::handler(__irq86_handler),
    Vector::handler(__irq87_handler),
    Vector::handler(__irq88_handler),
    Vector::handler(__irq89_handler),
    Vector::handler(__irq90_handler),
    Vector::handler(__irq91_handler),
    Vector::handler(__irq92_handler),
    Vector::handler(__irq93_handler),
    Vector::handler(__irq94_handler),
    Vector::handler(__irq95_handler),
    Vector::handler(__irq96_handler),
    Vector::handler(__irq97_handler),
    Vector::handler(__irq98_handler),
    Vector::handler(__irq99_handler),
    Vector::handler(__irq100_handler),
    Vector::handler(__irq101_handler),
    Vector::handler(__irq102_handler),
    Vector::handler(__irq103_handler),
    Vector::handler(__irq104_handler),
    Vector::handler(__irq105_handler),
    Vector::handler(__irq106_handler),
    Vector::handler(__irq107_handler),
    Vector::handler(__irq108_handler),
    Vector::handler(__irq109_handler),
    Vector::handler(__irq110_handler),
    Vector::handler(__irq111_handler),
    Vector::handler(__irq112_handler),
    Vector::handler(__irq113_handler),
    Vector::handler(__irq114_handler),
    Vector::handler(__irq115_handler),
    Vector::handler(__irq116_handler),
    Vector::handler(__irq117_handler),
    Vector::handler(__irq118_handler),
    Vector::handler(__irq119_handler),
    Vector::handler(__irq120_handler),
    Vector::handler(__irq121_handler),
    Vector::handler(__irq122_handler),
    Vector::handler(__irq123_handler),
    Vector::handler(__irq124_handler),
    Vector::handler(__irq125_handler),
    Vector::handler(__irq126_handler),
    Vector::handler(__irq127_handler),
    Vector::handler(__irq128_handler),
    Vector::handler(__irq129_handler),
    Vector::handler(__irq130_handler),
    Vector::handler(__irq131_handler),
    Vector::handler(__irq132_handler),
    Vector::handler(__irq133_handler),
    Vector::handler(__irq134_handler),
    Vector::handler(__irq135_handler),
    Vector::handler(__irq136_handler),
    Vector::handler(__irq137_handler),
    Vector::handler(__irq138_handler),
    Vector::handler(__irq139_handler),
    Vector::handler(__irq140_handler),
    Vector::handler(__irq141_handler),
    Vector::handler(__irq142_handler),
    Vector::handler(__irq143_handler),
    Vector::handler(__irq144_handler),
    Vector::handler(__irq145_handler),
    Vector::handler(__irq146_handler),
    Vector::handler(__irq147_handler),
    Vector::handler(__irq148_handler),
    Vector::handler(__irq149_handler),
    Vector::handler(__irq150_handler),
    Vector::handler(__irq151_handler),
    Vector::handler(__irq152_handler),
    Vector::handler(__irq153_handler),
    Vector::handler(__irq154_handler),
    Vector::handler(__irq155_handler),
    Vector::handler(__irq156_handler),
    Vector::handler(__irq157_handler),
    Vector::handler(__irq158_handler),
    Vector::handler(__irq159_handler),
    Vector::handler(__irq160_handler),
    Vector::handler(__irq161_handler),
    Vector::handler(__irq162_handler),
    Vector::handler(__irq163_handler),
    Vector::handler(__irq164_handler),
    Vector::handler(__irq165_handler),
    Vector::handler(__irq166_handler),
    Vector::handler(__irq167_handler),
    Vector::handler(__irq168_handler),
    Vector::handler(__irq169_handler),
    Vector::handler(__irq170_handler),
    Vector::handler(__irq171_handler),
    Vector::handler(__irq172_handler),
    Vector::handler(__irq173_handler),
    Vector::handler(__irq174_handler),
    Vector::handler(__irq175_handler),
    Vector::handler(__irq176_handler),
    Vector::handler(__irq177_handler),
    Vector::handler(__irq178_handler),
    Vector::handler(__irq179_handler),
    Vector::handler(__irq180_handler),
    Vector::handler(__irq181_handler),
    Vector::handler(__irq182_handler),
    Vector::handler(__irq183_handler),
    Vector::handler(__irq184_handler),
    Vector::handler(__irq185_handler),
    Vector::handler(__irq186_handler),
    Vector::handler(__irq187_handler),
    Vector::handler(__irq188_handler),
    Vector::handler(__irq189_handler),
    Vector::handler(__irq190_handler),
    Vector::handler(__irq191_handler),
    Vector::handler(__irq192_handler),
    Vector::handler(__irq193_handler),
    Vector::handler(__irq194_handler),
    Vector::handler(__irq195_handler),
    Vector::handler(__irq196_handler),
    Vector::handler(__irq197_handler),
    Vector::handler(__irq198_handler),
    Vector::handler(__irq199_handler),
    Vector::handler(__irq200_handler),
    Vector::handler(__irq201_handler),
    Vector::handler(__irq202_handler),
    Vector::handler(__irq203_handler),
    Vector::handler(__irq204_handler),
    Vector::handler(__irq205_handler),
    Vector::handler(__irq206_handler),
    Vector::handler(__irq207_handler),
    Vector::handler(__irq208_handler),
    Vector::handler(__irq209_handler),
    Vector::handler(__irq210_handler),
    Vector::handler(__irq211_handler),
    Vector::handler(__irq212_handler),
    Vector::handler(__irq213_handler),
    Vector::handler(__irq214_handler),
    Vector::handler(__irq215_handler),
    Vector::handler(__irq216_handler),
    Vector::handler(__irq217_handler),
    Vector::handler(__irq218_handler),
    Vector::handler(__irq219_handler),
    Vector::handler(__irq220_handler),
    Vector::handler(__irq221_handler),
    Vector::handler(__irq222_handler),
    Vector::handler(__irq223_handler),
    Vector::handler(__irq224_handler),
    Vector::handler(__irq225_handler),
    Vector::handler(__irq226_handler),
    Vector::handler(__irq227_handler),
    Vector::handler(__irq228_handler),
    Vector::handler(__irq229_handler),
    Vector::handler(__irq230_handler),
    Vector::handler(__irq231_handler),
    Vector::handler(__irq232_handler),
    Vector::handler(__irq233_handler),
    Vector::handler(__irq234_handler),
    Vector::handler(__irq235_handler),
    Vector::handler(__irq236_handler),
    Vector::handler(__irq237_handler),
    Vector::handler(__irq238_handler),
    Vector::handler(__irq239_handler),
];

extern "C" {
    static mut __heep_start: u32;

    fn __non_maskable_interrupt();

    fn __hard_fault();

    #[cfg(not(armv6m))]
    fn __memory_management_fault();

    #[cfg(not(armv6m))]
    fn __bus_fault();

    #[cfg(not(armv6m))]
    fn __usage_fault();

    #[cfg(armv8m)]
    fn __secure_fault();

    fn __sv_call();

    #[cfg(not(armv6m))]
    fn __debug_monitor();

    fn __pend_sv();

    fn __sys_tick();
}

extern "C" {
    fn __irq0_handler();
    fn __irq1_handler();
    fn __irq2_handler();
    fn __irq3_handler();
    fn __irq4_handler();
    fn __irq5_handler();
    fn __irq6_handler();
    fn __irq7_handler();
    fn __irq8_handler();
    fn __irq9_handler();
    fn __irq10_handler();
    fn __irq11_handler();
    fn __irq12_handler();
    fn __irq13_handler();
    fn __irq14_handler();
    fn __irq15_handler();
    fn __irq16_handler();
    fn __irq17_handler();
    fn __irq18_handler();
    fn __irq19_handler();
    fn __irq20_handler();
    fn __irq21_handler();
    fn __irq22_handler();
    fn __irq23_handler();
    fn __irq24_handler();
    fn __irq25_handler();
    fn __irq26_handler();
    fn __irq27_handler();
    fn __irq28_handler();
    fn __irq29_handler();
    fn __irq30_handler();
    fn __irq31_handler();
    fn __irq32_handler();
    fn __irq33_handler();
    fn __irq34_handler();
    fn __irq35_handler();
    fn __irq36_handler();
    fn __irq37_handler();
    fn __irq38_handler();
    fn __irq39_handler();
    fn __irq40_handler();
    fn __irq41_handler();
    fn __irq42_handler();
    fn __irq43_handler();
    fn __irq44_handler();
    fn __irq45_handler();
    fn __irq46_handler();
    fn __irq47_handler();
    fn __irq48_handler();
    fn __irq49_handler();
    fn __irq50_handler();
    fn __irq51_handler();
    fn __irq52_handler();
    fn __irq53_handler();
    fn __irq54_handler();
    fn __irq55_handler();
    fn __irq56_handler();
    fn __irq57_handler();
    fn __irq58_handler();
    fn __irq59_handler();
    fn __irq60_handler();
    fn __irq61_handler();
    fn __irq62_handler();
    fn __irq63_handler();
    fn __irq64_handler();
    fn __irq65_handler();
    fn __irq66_handler();
    fn __irq67_handler();
    fn __irq68_handler();
    fn __irq69_handler();
    fn __irq70_handler();
    fn __irq71_handler();
    fn __irq72_handler();
    fn __irq73_handler();
    fn __irq74_handler();
    fn __irq75_handler();
    fn __irq76_handler();
    fn __irq77_handler();
    fn __irq78_handler();
    fn __irq79_handler();
    fn __irq80_handler();
    fn __irq81_handler();
    fn __irq82_handler();
    fn __irq83_handler();
    fn __irq84_handler();
    fn __irq85_handler();
    fn __irq86_handler();
    fn __irq87_handler();
    fn __irq88_handler();
    fn __irq89_handler();
    fn __irq90_handler();
    fn __irq91_handler();
    fn __irq92_handler();
    fn __irq93_handler();
    fn __irq94_handler();
    fn __irq95_handler();
    fn __irq96_handler();
    fn __irq97_handler();
    fn __irq98_handler();
    fn __irq99_handler();
    fn __irq100_handler();
    fn __irq101_handler();
    fn __irq102_handler();
    fn __irq103_handler();
    fn __irq104_handler();
    fn __irq105_handler();
    fn __irq106_handler();
    fn __irq107_handler();
    fn __irq108_handler();
    fn __irq109_handler();
    fn __irq110_handler();
    fn __irq111_handler();
    fn __irq112_handler();
    fn __irq113_handler();
    fn __irq114_handler();
    fn __irq115_handler();
    fn __irq116_handler();
    fn __irq117_handler();
    fn __irq118_handler();
    fn __irq119_handler();
    fn __irq120_handler();
    fn __irq121_handler();
    fn __irq122_handler();
    fn __irq123_handler();
    fn __irq124_handler();
    fn __irq125_handler();
    fn __irq126_handler();
    fn __irq127_handler();
    fn __irq128_handler();
    fn __irq129_handler();
    fn __irq130_handler();
    fn __irq131_handler();
    fn __irq132_handler();
    fn __irq133_handler();
    fn __irq134_handler();
    fn __irq135_handler();
    fn __irq136_handler();
    fn __irq137_handler();
    fn __irq138_handler();
    fn __irq139_handler();
    fn __irq140_handler();
    fn __irq141_handler();
    fn __irq142_handler();
    fn __irq143_handler();
    fn __irq144_handler();
    fn __irq145_handler();
    fn __irq146_handler();
    fn __irq147_handler();
    fn __irq148_handler();
    fn __irq149_handler();
    fn __irq150_handler();
    fn __irq151_handler();
    fn __irq152_handler();
    fn __irq153_handler();
    fn __irq154_handler();
    fn __irq155_handler();
    fn __irq156_handler();
    fn __irq157_handler();
    fn __irq158_handler();
    fn __irq159_handler();
    fn __irq160_handler();
    fn __irq161_handler();
    fn __irq162_handler();
    fn __irq163_handler();
    fn __irq164_handler();
    fn __irq165_handler();
    fn __irq166_handler();
    fn __irq167_handler();
    fn __irq168_handler();
    fn __irq169_handler();
    fn __irq170_handler();
    fn __irq171_handler();
    fn __irq172_handler();
    fn __irq173_handler();
    fn __irq174_handler();
    fn __irq175_handler();
    fn __irq176_handler();
    fn __irq177_handler();
    fn __irq178_handler();
    fn __irq179_handler();
    fn __irq180_handler();
    fn __irq181_handler();
    fn __irq182_handler();
    fn __irq183_handler();
    fn __irq184_handler();
    fn __irq185_handler();
    fn __irq186_handler();
    fn __irq187_handler();
    fn __irq188_handler();
    fn __irq189_handler();
    fn __irq190_handler();
    fn __irq191_handler();
    fn __irq192_handler();
    fn __irq193_handler();
    fn __irq194_handler();
    fn __irq195_handler();
    fn __irq196_handler();
    fn __irq197_handler();
    fn __irq198_handler();
    fn __irq199_handler();
    fn __irq200_handler();
    fn __irq201_handler();
    fn __irq202_handler();
    fn __irq203_handler();
    fn __irq204_handler();
    fn __irq205_handler();
    fn __irq206_handler();
    fn __irq207_handler();
    fn __irq208_handler();
    fn __irq209_handler();
    fn __irq210_handler();
    fn __irq211_handler();
    fn __irq212_handler();
    fn __irq213_handler();
    fn __irq214_handler();
    fn __irq215_handler();
    fn __irq216_handler();
    fn __irq217_handler();
    fn __irq218_handler();
    fn __irq219_handler();
    fn __irq220_handler();
    fn __irq221_handler();
    fn __irq222_handler();
    fn __irq223_handler();
    fn __irq224_handler();
    fn __irq225_handler();
    fn __irq226_handler();
    fn __irq227_handler();
    fn __irq228_handler();
    fn __irq229_handler();
    fn __irq230_handler();
    fn __irq231_handler();
    fn __irq232_handler();
    fn __irq233_handler();
    fn __irq234_handler();
    fn __irq235_handler();
    fn __irq236_handler();
    fn __irq237_handler();
    fn __irq238_handler();
    fn __irq239_handler();
}
