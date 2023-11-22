


const M: u32 = 365;
const Q_lower: u32 = 15;
const Q_upper: u32 = 30;


fn calculate_eps_exact(m: u32, q: u32) -> f64{

    let mut res = 1.0;
    for i in 1..=q-1{
        res *= (m as f64 - i as f64)/(m as f64);
    }
    return 1.0 - res;
    // 0.0
}

fn calculate_epx_approx(m: u32, q: u32) -> f64{
    let q_f64 = q as f64;
    let x: f64 = q_f64 * (q_f64 - 1.0) / (2.0 * M as f64);
    let eps_approx = 1.0 - std::f64::consts::E.powf(-x);
    eps_approx
}

fn main() {

    println!("| Q | eps_exact | epx_approx |");
    for q in Q_lower..=Q_upper{
        let eps_exact = calculate_eps_exact(M, q);
        let eps_approx = calculate_epx_approx(M, q);
        println!("| {:?} | {:.5} | {:.5} |", q, eps_exact, eps_approx);
    }

}
