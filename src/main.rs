use rppal::gpio::{Gpio, OutputPin};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct SoftwarePwm {
    pin: OutputPin,
    duty_cycle: Arc<Mutex<f64>>, // 0.0 to 1.0
    frequency: f64,              // Hz
}

impl SoftwarePwm {
    fn new(pin: OutputPin, duty_cycle: f64, frequency: f64) -> Self {
        Self {
            pin,
            duty_cycle: Arc::new(Mutex::new(duty_cycle)),
            frequency,
        }
    }

    fn start(&mut self) {
        let period = Duration::from_secs_f64(1.0 / self.frequency);

        loop {
            let start = Instant::now();

            let duty_cycle = *self.duty_cycle.lock().unwrap();
            let high_time = period.mul_f64(duty_cycle);
            let low_time = period - high_time;

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
        SoftwarePwm::new(gpio.get(16)?.into_output(), 0.0, 1000.0), // GPIO17 (Pin 11)
        SoftwarePwm::new(gpio.get(26)?.into_output(), 0.0, 1000.0), // GPIO27 (Pin 13)
        SoftwarePwm::new(gpio.get(6)?.into_output(), 0.0, 1000.0),  // GPIO22 (Pin 15)
    ];

    // Clone the Arc<Mutex<f64>> to allow changing duty_cycle from the main thread
    let duty_cycles = pwm_pins
        .iter()
        .map(|pwm| pwm.duty_cycle.clone())
        .collect::<Vec<_>>();

    // Start each PWM in a new thread
    for mut pwm in pwm_pins {
        thread::spawn(move || {
            pwm.start();
        });
    }

    // Change the duty_cycle of the first PWM pin after 2 seconds
    thread::sleep(Duration::from_secs(2));
    {
        let mut duty_cycle = duty_cycles[0].lock().unwrap();
        *duty_cycle = 0.5;
    }

    // Change the duty_cycle of the second PWM pin after 4 seconds
    thread::sleep(Duration::from_secs(2));
    {
        let mut duty_cycle = duty_cycles[1].lock().unwrap();
        *duty_cycle = 0.75;
    }

    // Change the duty_cycle of the third PWM pin after 6 seconds
    thread::sleep(Duration::from_secs(2));
    {
        let mut duty_cycle = duty_cycles[2].lock().unwrap();
        *duty_cycle = 0.25;
    }

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
