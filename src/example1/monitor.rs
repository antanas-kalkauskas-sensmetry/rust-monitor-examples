struct MonitorState {
    temperature_buffer: [f64; 5],
    temperature_buffer_index: usize,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            temperature_buffer: [20.0; 5],
            temperature_buffer_index: 0,
        }
    }
}

impl MonitorState {
    fn new() -> Self {
        Default::default()
    }
}

pub struct MonitorInput {
    pub temperature: f64,
    pub speed: f64,
}

impl Default for MonitorInput {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            speed: 0.0,
        }
    }
}

impl MonitorInput {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct MonitorOutput {
    pub heat_on: bool,
    pub heat_off: bool,
    pub speed_warning: bool,
}

impl Default for MonitorOutput {
    fn default() -> Self {
        MonitorOutput {
            heat_on: false,
            heat_off: false,
            speed_warning: false,
        }
    }
}

impl MonitorOutput {
    pub fn new() -> Self {
        MonitorOutput::default()
    }
}

pub struct Monitor {
    state: MonitorState,
}

fn internal_step(input: &MonitorInput, state: &mut MonitorState, output: &mut MonitorOutput) {
    state.temperature_buffer[state.temperature_buffer_index] = input.temperature;
    state.temperature_buffer_index = (state.temperature_buffer_index + 1) % 5;

    let temperature_moving_average: f64 = (state.temperature_buffer.iter().sum::<f64>()) / 5.0;
    output.heat_on = temperature_moving_average < 18.0;
    output.heat_off = temperature_moving_average > 22.0;
    output.speed_warning = input.speed > 100.0;
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            state: MonitorState::new(),
        }
    }
    pub fn step(&mut self, input: &MonitorInput, output: &mut MonitorOutput) {
        internal_step(input, &mut self.state, output);
    }
}
