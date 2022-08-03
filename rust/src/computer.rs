use gdnative::prelude::*;
use rust_lisp::*;
use std::time::{Duration, Instant};

pub struct Computer {
    state: Option<State>,
    last_tick: Instant,
    clock_rate: Duration,
}

impl Computer {
    pub fn new(clock_rate_us: u64) -> Computer {
        Computer {
            state: None,
            last_tick: Instant::now(),
            clock_rate: Duration::from_micros(clock_rate_us),
        }
    }
    pub fn start(&mut self, arg: &str) {
        if self.is_locked() {
            godot_print!("tikn");
            panic!("ALREADY TICKIN")
        }
        if let Ok(expr) = parse(arg) {
            let mut state = State::empty();
            state.bind_builtin(builtin! {
                fn + (a:Number, b:Number) => int!(a+b)
            });
            state.begin(expr);
            self.state = Some(state);
            godot_print!("parsed up {:?}", self.state);
        } else {
            godot_print!("Falllll");
            panic!("FAILED TO PARSE {}", arg);
        }
    }
    pub fn tick(&mut self) {
        if let Some(state) = &mut self.state {
            // godot_print!("TICK {:?}", state);
            state.tick();
        }
    }
    pub fn progress(&mut self) -> Option<Result<Expression, EvaluationError>> {
        let now = Instant::now();
        let time_since = now.duration_since(self.last_tick);
        let ticks = time_since.as_micros() / self.clock_rate.as_micros();
        for _ in 0..ticks {
            self.tick();
            self.last_tick = self.last_tick + self.clock_rate;
        }
        if let Some(state) = &mut self.state {
            let value = state.get_value();
            if value.is_some() {
                self.state = None;
            }
            value
        } else {
            None
        }
    }
    pub fn is_locked(&self) -> bool {
        self.state.is_some()
    }
}
