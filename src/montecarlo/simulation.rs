use rand::distributions::{Distribution, Uniform};
use std::error::Error;
use super::Options;

fn f_inv(u: f64, lambda: f64) -> f64 {
    -1.0 / lambda * u.ln()
}

/// epochs: number of monte carlo simulations
/// vbs: arrival rate (not actually volume bucket size)
/// n: number of buckets per vpin but not sure why it's needed here
/// alpha: probability of information event occuring
/// mu: arrival rate of informed traders
/// opt.delta: [not important] information event is good/bad
pub fn run(opt: &Options) -> Result<(), Box<dyn Error>>{
    let mut vpin_exp: f64;
    let mut vpin_var: f64;
    let mut vpin_std: f64;
    let mut pin: f64;
    let mut error: f64;
    let between = Uniform::from(0. ..1.0);
    let mut rng = rand::thread_rng();
    
    for alpha in (0..=opt.max_alpha*10).step_by(1).map(|x| x as f64 * 0.1) {
        for mu in (0..=opt.max_mu).step_by(50).map(|x| x as f64) {
            let epsilon: f64 = (opt.vbs - alpha * mu) / 2.0;
            let mut vpin_sum: f64 = 0.;
            let mut vpin_squared_sum: f64 = 0.;

            for _s in 0..opt.epochs {
                let (mut tbv, mut tsv, mut imbalance) = (0f64, 0f64, 0f64);

                for _b in 0..opt.n {
                    let u1: f64 = between.sample(&mut rng);
                    let u2: f64 = between.sample(&mut rng);
                    let u3: f64 = between.sample(&mut rng);

                    let bv = match (u1, u2) {
                        (a, b) if a < alpha && b < opt.delta || a >= alpha => f_inv(u3, epsilon),
                        _ => f_inv(u3, mu + epsilon)
                    };

                    let sv = match (u1, u2) {
                        (a, b) if a < alpha && b < opt.delta => f_inv(u3, mu + epsilon),
                        (a, b) if a < alpha && b >= opt.delta  => f_inv(u3, epsilon),
                        _ => bv
                    };

                    tbv += bv;
                    tsv += sv;
                    imbalance += (bv - sv).abs();
                }

                let vpin: f64 = imbalance / (tbv + tsv);
                vpin_sum += vpin;
                vpin_squared_sum += vpin.powi(2);
            }

            let epochs = opt.epochs as f64;
            vpin_exp = vpin_sum / epochs; 
            // V(X) = S/S-1 * [E(X^2) - E(X)^2]
            vpin_var = epochs/(epochs - 1.0) * (vpin_squared_sum / epochs - vpin_exp.powi(2));
            vpin_std = (vpin_var / epochs).sqrt();
            pin = (alpha * mu) / (alpha * mu + 2.0 * epsilon);
            error = (pin - vpin_exp).abs();

            println!("alpha: {:.1}; mu: {}; true value: {:.4}; std error: {:.4}; est error: {:.4}", 
                alpha, mu, vpin_exp, vpin_std, error);
        }
    }
    Ok(())
}

