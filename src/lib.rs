#![no_std]

use core::{ panic::PanicInfo, ptr::{ addr_of, addr_of_mut }, slice };

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

#[derive(Clone, Copy)]
pub union Handler {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

impl Handler {
    #[inline]
    const fn reserved() -> Self {
        Handler { reserved: 0 }
    }

    #[inline]
    const fn new(func: unsafe extern "C" fn()) -> Self {
        Handler { handler: func }
    }
}

#[no_mangle]
extern "C" fn __default_handler() {
    loop {
    }
}

#[no_mangle]
unsafe extern "C" fn __reset_handler() -> ! {
    init_bss();
    init_data();

    extern "C" {
        fn __main() -> !;
    }

    __main()
}

#[inline]
pub unsafe fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __heep_start: u32;
    }

    addr_of_mut!(__heep_start)
}

#[inline(always)]
unsafe fn init_bss() {
    extern "C" {
        static mut __sbss: u32;
        static __ebss: u32;
    }

    let bss_size = addr_of!(__ebss).offset_from(addr_of!(__sbss)) as usize;
    if bss_size > 0 {
        slice::from_raw_parts_mut(addr_of_mut!(__sbss), bss_size).fill(0);
    }
}

#[inline(always)]
unsafe fn init_data() {
    extern "C" {
        static mut __sdata: u32;
        static __edata: u32;
        static __sidata: u32;
    }

    let data_size = addr_of!(__edata).offset_from(addr_of!(__sdata)) as usize;
    if data_size > 0 {
        let data = slice::from_raw_parts_mut(addr_of_mut!(__sdata), data_size);
        let idata = slice::from_raw_parts(&__sidata as *const u32, data_size);

        data.copy_from_slice(idata);
    }
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = __reset_handler;

#[no_mangle]
#[link_section = ".vector_table.exceptions"]
pub static __EXCEPTIONS: [Handler; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Handler::new(__non_maskable_interrupt),
    // Exception 3: Hard Fault Interrupt.
    Handler::new(__hard_fault),
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Handler::new(__memory_management_fault),
    #[cfg(armv6m)] Handler::reserved(),
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Handler::new(__bus_fault),
    #[cfg(armv6m)] Handler::reserved(),
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Handler::new(__usage_fault),
    #[cfg(armv6m)] Handler::reserved(),
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    #[cfg(armv8m)] Handler::new(__secure_fault),
    #[cfg(not(armv8m))] Handler::reserved(),
    // 8-10: Reserved
    Handler::reserved(),
    Handler::reserved(),
    Handler::reserved(),
    // Exception 11: SV Call Interrupt.
    Handler::new(__sv_call),
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))] Handler::new(__debug_monitor),
    #[cfg(armv6m)] Handler::reserved(),
    // 13: Reserved
    Handler::reserved(),
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Handler::new(__pend_sv),
    // Exception 15: System Tick Interrupt.
    Handler::new(__sys_tick),
];

#[no_mangle]
#[link_section = ".vector_table.interrupts"]
pub static __INTERRUPTS: [Handler; 240] = [
    Handler::new(__irq0_handler),
    Handler::new(__irq1_handler),
    Handler::new(__irq2_handler),
    Handler::new(__irq3_handler),
    Handler::new(__irq4_handler),
    Handler::new(__irq5_handler),
    Handler::new(__irq6_handler),
    Handler::new(__irq7_handler),
    Handler::new(__irq8_handler),
    Handler::new(__irq9_handler),
    Handler::new(__irq10_handler),
    Handler::new(__irq11_handler),
    Handler::new(__irq12_handler),
    Handler::new(__irq13_handler),
    Handler::new(__irq14_handler),
    Handler::new(__irq15_handler),
    Handler::new(__irq16_handler),
    Handler::new(__irq17_handler),
    Handler::new(__irq18_handler),
    Handler::new(__irq19_handler),
    Handler::new(__irq20_handler),
    Handler::new(__irq21_handler),
    Handler::new(__irq22_handler),
    Handler::new(__irq23_handler),
    Handler::new(__irq24_handler),
    Handler::new(__irq25_handler),
    Handler::new(__irq26_handler),
    Handler::new(__irq27_handler),
    Handler::new(__irq28_handler),
    Handler::new(__irq29_handler),
    Handler::new(__irq30_handler),
    Handler::new(__irq31_handler),
    Handler::new(__irq32_handler),
    Handler::new(__irq33_handler),
    Handler::new(__irq34_handler),
    Handler::new(__irq35_handler),
    Handler::new(__irq36_handler),
    Handler::new(__irq37_handler),
    Handler::new(__irq38_handler),
    Handler::new(__irq39_handler),
    Handler::new(__irq40_handler),
    Handler::new(__irq41_handler),
    Handler::new(__irq42_handler),
    Handler::new(__irq43_handler),
    Handler::new(__irq44_handler),
    Handler::new(__irq45_handler),
    Handler::new(__irq46_handler),
    Handler::new(__irq47_handler),
    Handler::new(__irq48_handler),
    Handler::new(__irq49_handler),
    Handler::new(__irq50_handler),
    Handler::new(__irq51_handler),
    Handler::new(__irq52_handler),
    Handler::new(__irq53_handler),
    Handler::new(__irq54_handler),
    Handler::new(__irq55_handler),
    Handler::new(__irq56_handler),
    Handler::new(__irq57_handler),
    Handler::new(__irq58_handler),
    Handler::new(__irq59_handler),
    Handler::new(__irq60_handler),
    Handler::new(__irq61_handler),
    Handler::new(__irq62_handler),
    Handler::new(__irq63_handler),
    Handler::new(__irq64_handler),
    Handler::new(__irq65_handler),
    Handler::new(__irq66_handler),
    Handler::new(__irq67_handler),
    Handler::new(__irq68_handler),
    Handler::new(__irq69_handler),
    Handler::new(__irq70_handler),
    Handler::new(__irq71_handler),
    Handler::new(__irq72_handler),
    Handler::new(__irq73_handler),
    Handler::new(__irq74_handler),
    Handler::new(__irq75_handler),
    Handler::new(__irq76_handler),
    Handler::new(__irq77_handler),
    Handler::new(__irq78_handler),
    Handler::new(__irq79_handler),
    Handler::new(__irq80_handler),
    Handler::new(__irq81_handler),
    Handler::new(__irq82_handler),
    Handler::new(__irq83_handler),
    Handler::new(__irq84_handler),
    Handler::new(__irq85_handler),
    Handler::new(__irq86_handler),
    Handler::new(__irq87_handler),
    Handler::new(__irq88_handler),
    Handler::new(__irq89_handler),
    Handler::new(__irq90_handler),
    Handler::new(__irq91_handler),
    Handler::new(__irq92_handler),
    Handler::new(__irq93_handler),
    Handler::new(__irq94_handler),
    Handler::new(__irq95_handler),
    Handler::new(__irq96_handler),
    Handler::new(__irq97_handler),
    Handler::new(__irq98_handler),
    Handler::new(__irq99_handler),
    Handler::new(__irq100_handler),
    Handler::new(__irq101_handler),
    Handler::new(__irq102_handler),
    Handler::new(__irq103_handler),
    Handler::new(__irq104_handler),
    Handler::new(__irq105_handler),
    Handler::new(__irq106_handler),
    Handler::new(__irq107_handler),
    Handler::new(__irq108_handler),
    Handler::new(__irq109_handler),
    Handler::new(__irq110_handler),
    Handler::new(__irq111_handler),
    Handler::new(__irq112_handler),
    Handler::new(__irq113_handler),
    Handler::new(__irq114_handler),
    Handler::new(__irq115_handler),
    Handler::new(__irq116_handler),
    Handler::new(__irq117_handler),
    Handler::new(__irq118_handler),
    Handler::new(__irq119_handler),
    Handler::new(__irq120_handler),
    Handler::new(__irq121_handler),
    Handler::new(__irq122_handler),
    Handler::new(__irq123_handler),
    Handler::new(__irq124_handler),
    Handler::new(__irq125_handler),
    Handler::new(__irq126_handler),
    Handler::new(__irq127_handler),
    Handler::new(__irq128_handler),
    Handler::new(__irq129_handler),
    Handler::new(__irq130_handler),
    Handler::new(__irq131_handler),
    Handler::new(__irq132_handler),
    Handler::new(__irq133_handler),
    Handler::new(__irq134_handler),
    Handler::new(__irq135_handler),
    Handler::new(__irq136_handler),
    Handler::new(__irq137_handler),
    Handler::new(__irq138_handler),
    Handler::new(__irq139_handler),
    Handler::new(__irq140_handler),
    Handler::new(__irq141_handler),
    Handler::new(__irq142_handler),
    Handler::new(__irq143_handler),
    Handler::new(__irq144_handler),
    Handler::new(__irq145_handler),
    Handler::new(__irq146_handler),
    Handler::new(__irq147_handler),
    Handler::new(__irq148_handler),
    Handler::new(__irq149_handler),
    Handler::new(__irq150_handler),
    Handler::new(__irq151_handler),
    Handler::new(__irq152_handler),
    Handler::new(__irq153_handler),
    Handler::new(__irq154_handler),
    Handler::new(__irq155_handler),
    Handler::new(__irq156_handler),
    Handler::new(__irq157_handler),
    Handler::new(__irq158_handler),
    Handler::new(__irq159_handler),
    Handler::new(__irq160_handler),
    Handler::new(__irq161_handler),
    Handler::new(__irq162_handler),
    Handler::new(__irq163_handler),
    Handler::new(__irq164_handler),
    Handler::new(__irq165_handler),
    Handler::new(__irq166_handler),
    Handler::new(__irq167_handler),
    Handler::new(__irq168_handler),
    Handler::new(__irq169_handler),
    Handler::new(__irq170_handler),
    Handler::new(__irq171_handler),
    Handler::new(__irq172_handler),
    Handler::new(__irq173_handler),
    Handler::new(__irq174_handler),
    Handler::new(__irq175_handler),
    Handler::new(__irq176_handler),
    Handler::new(__irq177_handler),
    Handler::new(__irq178_handler),
    Handler::new(__irq179_handler),
    Handler::new(__irq180_handler),
    Handler::new(__irq181_handler),
    Handler::new(__irq182_handler),
    Handler::new(__irq183_handler),
    Handler::new(__irq184_handler),
    Handler::new(__irq185_handler),
    Handler::new(__irq186_handler),
    Handler::new(__irq187_handler),
    Handler::new(__irq188_handler),
    Handler::new(__irq189_handler),
    Handler::new(__irq190_handler),
    Handler::new(__irq191_handler),
    Handler::new(__irq192_handler),
    Handler::new(__irq193_handler),
    Handler::new(__irq194_handler),
    Handler::new(__irq195_handler),
    Handler::new(__irq196_handler),
    Handler::new(__irq197_handler),
    Handler::new(__irq198_handler),
    Handler::new(__irq199_handler),
    Handler::new(__irq200_handler),
    Handler::new(__irq201_handler),
    Handler::new(__irq202_handler),
    Handler::new(__irq203_handler),
    Handler::new(__irq204_handler),
    Handler::new(__irq205_handler),
    Handler::new(__irq206_handler),
    Handler::new(__irq207_handler),
    Handler::new(__irq208_handler),
    Handler::new(__irq209_handler),
    Handler::new(__irq210_handler),
    Handler::new(__irq211_handler),
    Handler::new(__irq212_handler),
    Handler::new(__irq213_handler),
    Handler::new(__irq214_handler),
    Handler::new(__irq215_handler),
    Handler::new(__irq216_handler),
    Handler::new(__irq217_handler),
    Handler::new(__irq218_handler),
    Handler::new(__irq219_handler),
    Handler::new(__irq220_handler),
    Handler::new(__irq221_handler),
    Handler::new(__irq222_handler),
    Handler::new(__irq223_handler),
    Handler::new(__irq224_handler),
    Handler::new(__irq225_handler),
    Handler::new(__irq226_handler),
    Handler::new(__irq227_handler),
    Handler::new(__irq228_handler),
    Handler::new(__irq229_handler),
    Handler::new(__irq230_handler),
    Handler::new(__irq231_handler),
    Handler::new(__irq232_handler),
    Handler::new(__irq233_handler),
    Handler::new(__irq234_handler),
    Handler::new(__irq235_handler),
    Handler::new(__irq236_handler),
    Handler::new(__irq237_handler),
    Handler::new(__irq238_handler),
    Handler::new(__irq239_handler),
];

extern "C" {
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
