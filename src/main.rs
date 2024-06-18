use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::{Duration, Instant};

struct SoftwarePwm {
    pin: OutputPin,
    duty_cycle: f64, // 0.0 to 1.0
    frequency: f64,  // Hz
}

impl SoftwarePwm {
    fn new(pin: OutputPin, duty_cycle: f64, frequency: f64) -> Self {
        Self {
            pin,
            duty_cycle,
            frequency,
        }
    }

    fn start(&mut self) {
        let period = Duration::from_secs_f64(1.0 / self.frequency);
        let high_time = period.mul_f64(self.duty_cycle);
        let low_time = period - high_time;

        loop {
            let start = Instant::now();
            self.pin.set_high();
            thread::sleep(high_time);
            self.pin.set_low();
            thread::sleep(low_time);

            // Check if we've overshot our period and adjust sleep accordingly
            let elapsed = start.elapsed();
            if elapsed < period {
                thread::sleep(period - elapsed);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let mut pwm_pins = vec![
        SoftwarePwm::new(gpio.get(16)?.into_output(), 0.1, 1000.0), // GPIO17 (Pin 11)
        SoftwarePwm::new(gpio.get(27)?.into_output(), 0.50, 1000.0), // GPIO27 (Pin 13)
        SoftwarePwm::new(gpio.get(22)?.into_output(), 0.75, 1000.0), // GPIO22 (Pin 15)
        SoftwarePwm::new(gpio.get(23)?.into_output(), 0.90, 1000.0), // GPIO23 (Pin 16)
    ];

    // Start each PWM in a new thread
    for mut pwm in pwm_pins {
        thread::spawn(move || {
            pwm.start();
        });
    }

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
