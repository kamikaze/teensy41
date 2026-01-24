//! Accelerating/decelerating LED blink pattern

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    const MIN_DELAY_MS: u32 = 5;   // Fastest blink (50ms)
    const MAX_DELAY_MS: u32 = 250;  // Slowest blink (500ms)
    const STEP_MS: u32 = 5;         // Speed change step

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        pit: bsp::hal::pit::Pit<2>,
        current_delay_ms: u32,
        accelerating: bool,  // true = getting faster, false = getting slower
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            pins,
            pit: (_, _, mut pit, _),
            mut gpio2,
            ..
        } = board::t40(cx.device);

        let led = board::led(&mut gpio2, pins.p13);
        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(board::PERCLK_FREQUENCY / 500 * MAX_DELAY_MS);
        pit.enable();

        (
            Shared {},
            Local {
                led,
                pit,
                current_delay_ms: MAX_DELAY_MS,
                accelerating: true,
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = PIT, local = [led, pit, current_delay_ms, accelerating])]
    fn variable_blink(cx: variable_blink::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;
        let delay = cx.local.current_delay_ms;
        let accelerating = cx.local.accelerating;

        // Toggle LED
        led.toggle();

        // Update speed - accelerate (decrease delay) or decelerate (increase delay)
        if *accelerating {
            if *delay > MIN_DELAY_MS + STEP_MS {
                *delay -= STEP_MS;
            } else {
                *delay = MIN_DELAY_MS;
                *accelerating = false;  // Start slowing down
            }
        } else if *delay < MAX_DELAY_MS - STEP_MS {
            *delay += STEP_MS;
        } else {
            *delay = MAX_DELAY_MS;
            *accelerating = true;   // Start speeding up again
        }

        // Set new timer value
        pit.set_load_timer_value(board::PERCLK_FREQUENCY / 1_000 * *delay);

        // Clear timer interrupt
        while pit.is_elapsed() {
            pit.clear_elapsed();
        }
    }
}