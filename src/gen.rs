use std::f32::consts::PI;

pub struct GenSin {
    start: f32,
    end: f32,
    dt: f32,
    step: usize,
    current_step: usize,
    s: f32,
    current_t: f32,
}

impl GenSin {
    #[allow(dead_code)]
    pub fn new(start: f32, end: f32, time: f32, dt: f32) -> Self {
        let step = (time / dt) as usize;
        let s = (end - start) / 2.0 / PI;
        let dt = dt * 2.0 * PI / time;
        
        GenSin {
            start,
            end,
            dt,
            step,
            current_step: 0,
            s,
            current_t: 0.0,
        }
    }
}

impl Iterator for GenSin {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step > self.step {
            return None;
        }

        let result = if self.current_step == self.step {
            self.end
        } else {
            let t = self.current_t;
            self.current_t += self.dt;
            (t - t.sin()) * self.s + self.start
        };

        self.current_step += 1;
        Some(result)
    }
}

pub struct GenLinear {
    end: f32,
    step: usize,
    current_step: usize,
    current_value: f32,
    increment: f32,
}

impl GenLinear {
    #[allow(dead_code)]
    pub fn new(start: f32, end: f32, time: f32, dt: f32) -> Self {
        let step = (time / dt) as usize;
        let increment = (end - start) / step as f32;

        GenLinear {
            end,
            step,
            current_step: 0,
            current_value: start,
            increment,
        }
    }
}

impl Iterator for GenLinear {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step > self.step {
            return None;
        }

        let result = if self.current_step == self.step {
            self.end
        } else {
            let value = self.current_value;
            self.current_value += self.increment;
            value
        };

        self.current_step += 1;
        Some(result)
    }
}

// Usage
/*
fn main() {
    let start = 5.0;
    let end = 10.0;
    let time = 5.0;
    let dt = 0.05;

    println!("GenSin results:");
    let gensin = GenSin::new(start, end, time, dt);
    for value in gensin {
        println!("{}", value);
    }

    println!("\nGenLinear results:");
    let genlinear = GenLinear::new(start, end, time, dt);
    for value in genlinear {
        println!("{}", value);
    }
}
*/