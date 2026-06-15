mod viscosity;
mod visualization;
use nalgebra::DVector;
use viscosity::{BloodModel, Fit};

fn MathErr(rpm: &DVector<f64>, visc_exp: &DVector<f64>, spec_err: f64) -> DVector<f64> {
    let mut abs_errors = DVector::zeros(rpm.len());
    for i in 0..rpm.len() {
        let w = rpm[i];
        let err_instrument = 3.0 + (50.0 / w);
        let err_sedimentation = 15.0 * (-w / 20.0).exp();
        let err_wall_slip = 2.0 + 0.05 * w;
        let total_rel_err = err_instrument + err_sedimentation + err_wall_slip + spec_err;
        let final_rel_err = (total_rel_err / 100.0) / 2.0;
        abs_errors[i] = visc_exp[i] * final_rel_err;
    }
    abs_errors
}

fn main() {
    let rpm = DVector::from_vec(vec![6.0, 12.0, 20.0, 30.0, 60.0, 100.0, 150.0]);
    let visc_hep_exp = DVector::from_vec(vec![314.8, 172.3, 106.3, 74.88, 44.89, 25.0, 7.22]);
    let visc_cit_exp = DVector::from_vec(vec![340.0, 195.0, 125.0, 92.00, 58.00, 36.0, 18.55]);
    let abs_err_hep = MathErr(&rpm, &visc_hep_exp, 6.0);
    let abs_err_cit = MathErr(&rpm, &visc_cit_exp, 11.0);
    let heparin_model = BloodModel::new(1.0);
    let citrate_model = BloodModel::new(0.35);

    let initial_guess = Fit {
        eta_0: 15.0,
        eta_inf: 2.5,
        omega_c: 20.0,
        m: 0.8,
    };

    for i in 0..rpm.len() {
        let current_rpm = rpm[i];
        let hep_theory = heparin_model.Viscosity(current_rpm, &initial_guess);
        let cit_theory = citrate_model.Viscosity(current_rpm, &initial_guess);
        println!("RPM: {:>5.1} | Heparin Model: {:>6.2} | Citrate Model: {:>6.2}", current_rpm, hep_theory, cit_theory);
    }

    println!("погрешности гепарина: {:.2}", abs_err_hep);
    println!("погрешности цитрата: {:.2}", abs_err_cit);
    let mut dense_rpm = Vec::new();
    let mut dense_hep = Vec::new();
    let mut dense_cit = Vec::new();
    let mut w = 2.0;
    while w <= 160.0 {
        dense_rpm.push(w);
        dense_hep.push(heparin_model.Viscosity(w, &initial_guess));
        dense_cit.push(citrate_model.Viscosity(w, &initial_guess));
        w += 0.5;
    }
     if let Err(e) = visualization::draw(&rpm, &visc_hep_exp, &visc_cit_exp, &abs_err_hep, &abs_err_cit, &dense_rpm, &dense_hep, &dense_cit) {
        println!("error: {}", e);
    }
}
