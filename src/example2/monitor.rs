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

pub struct HeatOnVerdict {
    pub verdict: bool,
    pub arg0: f64,
}

pub struct HeatOffVerdict {
    pub verdict: bool,
    pub arg0: f64,
}

pub struct SpeedWarningVerdict {
    pub verdict: bool,
    pub arg0: f64,
}

pub struct MonitorOutput {
    pub heat_on_verdict: HeatOnVerdict,
    pub heat_off_verdict: HeatOffVerdict,
    pub speed_warning_verdict: SpeedWarningVerdict,
}

impl Default for MonitorOutput {
    fn default() -> Self {
        MonitorOutput {
            heat_on_verdict: HeatOnVerdict {
                verdict: false,
                arg0: 0.0,
            },
            heat_off_verdict: HeatOffVerdict {
                verdict: false,
                arg0: 0.0,
            },
            speed_warning_verdict: SpeedWarningVerdict {
                verdict: false,
                arg0: 0.0,
            },
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
    output.heat_on_verdict.verdict = temperature_moving_average < 18.0;
    output.heat_on_verdict.arg0 = temperature_moving_average;

    output.heat_off_verdict.verdict = temperature_moving_average > 22.0;
    output.heat_off_verdict.arg0 = temperature_moving_average;

    output.speed_warning_verdict.verdict = input.speed > 100.0;
    output.speed_warning_verdict.arg0 = input.speed;
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
