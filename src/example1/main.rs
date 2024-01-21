pub mod monitor;
pub use crate::monitor as m;

// A function to mock inputs
// Writes observed values to a reference of input struct
fn sense(input: &mut m::MonitorInput, iteration_number: i32) {
    input.temperature = 20.0 + 4.0 * f64::sin((iteration_number as f64) * 0.1);
    println!("Observed current temperature is {}", input.temperature);

    input.speed = 90.0 + 20.0 * f64::sin((iteration_number as f64) * 0.3);
    println!("Observed current speed is {}", input.speed)
}

fn heat_on() {
    println!("Turning heater on!")
}

fn heat_off() {
    println!("Turning heater off!")
}

fn speed_warning() {
    println!("Warning! Speed is too high!")
}

fn actuate(output: &m::MonitorOutput) {
    if output.heat_off {
        heat_off();
    }

    if output.heat_on {
        heat_on();
    }

    if output.speed_warning {
        speed_warning();
    }
}

fn main() {
    let mut monitor = m::Monitor::new();
    let mut output: m::MonitorOutput = m::MonitorOutput::new();
    let mut input = m::MonitorInput::new();

    for iteration_number in 0..30 {
        println!("Running iteration {}", iteration_number);

        sense(&mut input, iteration_number);
        monitor.step(&input, &mut output);
        actuate(&output);
        println!("")
    }
}
