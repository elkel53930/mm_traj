use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TrajResult {
    Done(State),
    Continue(State),
}

pub trait Trajectory {
    fn proceed(&mut self) -> TrajResult;
}

#[derive(Debug)]
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

    pub fn proceed(&mut self) -> TrajResult {
        if self.step == 0 {
            self.state.v = self.final_velocity;
            self.state.a = 0.0;
            self.state.x = self.x_origin + self.distance * self.theta.cos();
            self.state.y = self.y_origin + self.distance * self.theta.sin();
            return TrajResult::Done(self.state);
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
            return TrajResult::Done(self.state);
        }

        TrajResult::Continue(self.state)
    }

    pub fn origin(&self) -> (f32, f32) {
        (self.y_origin, self.x_origin)
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

    #[test]
    fn it_works() {
        let state = State {
            x: 0.045,
            y: 0.027,
            v: 0.0,
            a: 0.0,
            theta: std::f32::consts::PI / 2.0,
            omega: 0.0,
        };
        let mut straight = Straight::new(state, 0.02, 1.0);

        print_state_label();

        let mut mm_state;
        let mut time = 0;
        loop {
            match straight.proceed() {
                TrajResult::Done(state) => {
                    print_state_for_csv(time, state);
                    time += 1;
                    mm_state = state;
                    break;
                }
                TrajResult::Continue(state) => {
                    print_state_for_csv(time, state);
                    time += 1;
                }
            }
        }

        let mut straight = Straight::new(mm_state, 0.2, 0.0);

        loop {
            match straight.proceed() {
                TrajResult::Done(state) => {
                    mm_state = state;
                    print_state_for_csv(time, state);
                    time += 1;
                    break;
                }
                TrajResult::Continue(state) => {
                    print_state_for_csv(time, state);
                    time += 1;
                }
            }
        }
    }
}
