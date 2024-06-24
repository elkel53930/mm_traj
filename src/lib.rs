use serde::{Deserialize, Serialize};
mod gen;

const DT: f32 = 0.001; // 1ms

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq)]
pub struct State {
    pub x: f32,
    pub y: f32,
    pub v: f32,
    pub a: f32,
    pub theta: f32,
    pub omega: f32,
}

pub struct Straight {
    state: State,
    x_origin: f32,
    y_origin: f32,
    theta: f32,
    position: f32,
    distance: f32,
    acceleration: f32,
    final_velocity: f32,
    step: usize,
}

impl Straight {
    pub fn new(state: State, distance: f32, final_velocity: f32) -> Straight {
        let time = 2.0 * distance / (state.v + final_velocity);
        let step = (time / DT) as usize;
        let acceleration = (final_velocity - state.v) / time;

        Straight {
            state,
            x_origin: state.x,
            y_origin: state.y,
            theta: state.theta,
            position: 0.0,
            distance,
            acceleration,
            final_velocity,
            step,
        }
    }

    pub fn origin(&self) -> (f32, f32) {
        (self.y_origin, self.x_origin)
    }
}

impl Iterator for Straight {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 {
            self.state.v = self.final_velocity;
            self.state.a = 0.0;
            self.state.x = self.x_origin + self.distance * self.theta.cos();
            self.state.y = self.y_origin + self.distance * self.theta.sin();
            return None;
        }

        self.state.omega = 0.0;
        self.state.a = self.acceleration;
        self.state.v += self.acceleration * DT;
        self.state.x += self.state.v * DT * self.theta.cos();
        self.state.y += self.state.v * DT * self.theta.sin();
        self.position += self.state.v * DT;
        self.step -= 1;

        if self.position >= self.distance {
            self.state.v = self.final_velocity;
            self.state.a = 0.0;
            self.step = 0; // Ensure that the next call to next() will return None
        }

        Some(self.state)
    }
}


pub struct Pivot {
    state: State,
    gen_sin: gen::GenSin,
}

impl Pivot {
    pub fn new(state: State, final_theta: f32, time: f32) -> Self {
        let gen_sin = gen::GenSin::new(state.theta, state.theta + final_theta, time, DT);

        Pivot {
            state,
            gen_sin,
        }
    }
}

impl Iterator for Pivot {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(theta) = self.gen_sin.next() {
            self.state.omega = (theta - self.state.theta) / DT;
            self.state.theta = theta;
            Some(self.state)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn print_state_label() {
        println!("time,x,y,v,a,theta,omega,");
    }

    fn print_state_for_csv(time: u32, state: State) {
        println!(
            "{},{},{},{},{},{},{},",
            time, state.x, state.y, state.v, state.a, state.theta, state.omega
        );
    }

    fn straight() {
        let initial_state = State {
            x: 0.0,
            y: 0.0,
            v: 1.0,
            a: 0.0,
            theta: 0.0,
            omega: 0.0,
        };
        let distance = 10.0;
        let final_velocity = 2.0;

        let mut straight = Straight::new(initial_state, distance, final_velocity);
        let mut time = 0;
        println!("straight");
        print_state_label();
        while let Some(state) = straight.next() {
            print_state_for_csv(time, state);
            time += 1;
        }
    }

    #[test]
    fn pivot() {
        let initial_state = State {
            x: 0.045,
            y: 0.045,
            v: 0.0,
            a: 0.0,
            theta: std::f32::consts::PI / 2.0,
            omega: 0.0,
        };
        let final_theta = std::f32::consts::PI / 2.0;
        let time = 1.0;

        let mut pivot = Pivot::new(initial_state, final_theta, time);
        let mut time = 0;
        println!("pivot");
        print_state_label();
        while let Some(state) = pivot.next() {
            print_state_for_csv(time, state);
            time += 1;
        }
    }
}
