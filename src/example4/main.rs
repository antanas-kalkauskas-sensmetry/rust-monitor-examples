pub mod monitor;
use monitor::MonitorTriggers;

pub use crate::monitor as m;

// A function to mock inputs
// Writes observed values to a reference of input struct
fn sense(input: &mut m::MonitorInput, iteration_number: i32) {
    input.temperature = 20.0 + 4.0 * f64::sin((iteration_number as f64) * 0.1);
    println!("Observed current temperature is {}", input.temperature);

    input.speed = 90.0 + 20.0 * f64::sin((iteration_number as f64) * 0.3);
    println!("Observed current speed is {}", input.speed)
}

struct Triggers { }

// Implementation of MonitorTriggers trait with trigger functions
impl MonitorTriggers for Triggers {
    fn heat_on(average_temperature: f64) {
        println!("Turning heater on! Moving average of the temperature: {}", average_temperature);
    }
    
    fn heat_off(average_temperature: f64) {
        println!("Turning heater off! Moving average of the temperature: {}", average_temperature);
    }
    
    fn speed_warning(speed: f64) {
        println!("Warning! Speed is too high! Observed speed: {}", speed);
    }    
}

fn main() {
    let mut monitor: m::Monitor<Triggers> = m::Monitor::<Triggers>::new();
    let mut input = m::MonitorInput::new();

    for iteration_number in 0..30 {
        println!("Running iteration {}", iteration_number);

        sense(&mut input, iteration_number);
        monitor.step(&input);
        println!("")
    }
}
