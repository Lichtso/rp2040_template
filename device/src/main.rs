#![no_std]
#![no_main]

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

extern "C" fn unimplemented_handler() {
    panic!();
}

#[no_mangle]
#[link_section = ".interrupt_vector"]
static interrupt_vector: [unsafe extern "C" fn(); 42] = [
    unimplemented_handler; 42
];

#[no_mangle]
#[link_section = ".main"]
pub extern fn main() -> ! {
    let cp = rp2040_hal::pac::CorePeripherals::take().unwrap();
    let mut pac = rp2040_hal::pac::Peripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 8_000_000);
    let sio = rp2040_hal::sio::Sio::new(pac.SIO);
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.gpio25.into_push_pull_output();

    use embedded_hal::digital::v2::OutputPin;
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(1000);
        led_pin.set_low().unwrap();
        delay.delay_ms(1000);
    }
}
