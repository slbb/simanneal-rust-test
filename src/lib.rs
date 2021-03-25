use rand::Rng;
use std::time::Instant;
mod display;
use display::{display, init_display};

pub fn simanneal<T: Clone, F1, F2, F3>(
    mut state: T,
    steps: usize,
    mut move_state: F1,
    calc_state_energy: F2,
    mut temperature: f64,
    temperature_change_function: F3,
    update_trial: usize,
) -> (T, f64)
where
    F1: FnMut(&T) -> T,
    F2: Fn(&T) -> f64,
    F3: Fn(usize, usize) -> f64,
{
    let mut rng = rand::thread_rng();
    let mut energy: f64 = calc_state_energy(&state);
    let mut accepts: usize = 0;
    let mut improves: usize = 0;
    let mut best_state = state.clone();
    let mut best_energy = energy;
    let mut trials: usize = 0;
    {
        let mod_ = steps % update_trial;
        if mod_ != 0 {
            trials = mod_ + 1;
        }
    }
    let instant = Instant::now();
    init_display(temperature, energy, &instant);
    for step in 0..steps {
        temperature = temperature_change_function(step, steps);
        let new_state: T = move_state(&state);
        let new_energy: f64 = calc_state_energy(&new_state);
        let d_energy: f64 = new_energy - energy;
        if d_energy < 0.0 {
            improves += 1;
            if energy < best_energy {
                best_state = new_state.clone();
                best_energy = new_energy;
            }
            accepts += 1;
            state = new_state;
            energy = new_energy;
        } else if (-d_energy / temperature).exp() > rng.gen::<f64>() {
            accepts += 1;
            state = new_state;
            energy = new_energy;
        }
        trials += 1;
        if trials == update_trial {
            display(
                temperature,
                energy,
                (accepts as f64 / trials as f64) * 100_f64,
                (improves as f64 / trials as f64) * 100_f64,
                &instant,
                step as f64 / (steps - step) as f64,
            );
            trials = 0;
            accepts = 0;
            improves = 0;
        }
    }
    println!();
    (best_state, best_energy)
}
pub mod default {
    pub fn t(t_init: f64, t_min: f64) -> impl Fn(usize, usize) -> f64 {
        let t_factor: f64 = -((t_init / t_min).ln());
        move |step: usize, steps: usize| t_init * (t_factor * step as f64 / steps as f64).exp()
    }
    pub fn round_figures<T: Into<f64>>(x: T, n: i8) -> f64 {
        let x_f64: f64 = x.into();
        let n_sub_significant: i8 = n - x_f64.abs().log10().ceil() as i8;
        let tmp: f64 = (n_sub_significant.abs() * 10) as f64;
        if n_sub_significant < 0 {
            (x_f64 / tmp).round() * tmp
        } else if n_sub_significant == 0 {
            x_f64.round()
        } else {
            (x_f64 * tmp).round() / tmp
        }
    }
}
