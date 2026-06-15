use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct ViscoConst {
    pub r0: f64, // radius of red clood cell
    pub h0: f64, // width of red blood cell
    pub w: f64, //  field for the Quetta gap width
    pub phi_0: f64, // field for hematocrit
    pub phi_max: f64, // red blood cell unit limit
    pub eta_plasma: f64, // field for Newtonian plasma viscsity
}

impl ViscoConst {
    pub fn new() -> Self {
        Self {
            r0: 3.75,
            h0: 2.0,
            w: 100.0,
            phi_0: 0.42,
            phi_max: 0.65,
            eta_plasma: 1.2,
        }
    }
}

#[derive(Debug)]
pub struct Fit {
    pub eta_0: f64, // vicosity of big remnats on low speeds
    pub eta_inf: f64, // visosity of single agregats on low speeds
    pub omega_c: f64, // critical speed for disaggregation shift
    pub m: f64, // cross index
}

pub struct BloodModel {
    pub constants: ViscoConst,
    pub lambda_d: f64, // Debye length field
}

impl BloodModel {
    pub fn new(lambda_d: f64) -> Self {
        let constants = ViscoConst::new();
        Self {
            constants: constants,
            lambda_d: lambda_d,
        }
    }

    pub fn Viscosity(&self, omega: f64, params: &Fit) -> f64 {
        let debye_correction = 1.5 * (self.lambda_d / 1000.0);
        let volume_0 = PI * self.constants.r0.powi(2) * self.constants.h0;
        let r_eff = self.constants.r0 + debye_correction;
        let h_eff = self.constants.h0 + 2.0 * debye_correction;
        let volume_eff = PI * r_eff.powi(2) * h_eff;
        let phi = self.constants.phi_0 * (volume_eff / volume_0);
        let delta_w = 2.5 * (omega / (omega + 40.0)); // width of plasma layer

        let mut phi_core = phi * (self.constants.w / (self.constants.w - 2.0 * delta_w));
        if phi_core >= self.constants.phi_max {
            phi_core = self.constants.phi_max - 0.00001; // &&&&&&&&
        }
        let ratio = omega / params.omega_c; // current speed / omega0
        let power_term = ratio.powf(params.m);
        let eta_intrinsic = params.eta_inf + (params.eta_0 - params.eta_inf) / (1.0 + power_term); // characteristic viscosity of red blood cells
        let packing_term = 1.0 - (phi_core / self.constants.phi_max);
        let exponent = -eta_intrinsic * self.constants.phi_max; // calculate power
        let eta_core = self.constants.eta_plasma * packing_term.powf(exponent); // calculating the macroscopic viscosity of the dense central core
        let w_ratio = 2.0 * delta_w / self.constants.w; // ratio delta_plasma/delta_slot_of_visco
        let inv_eta = w_ratio * (1.0 / self.constants.eta_plasma) + (1.0 - w_ratio) * (1.0 / eta_core); // integrate by layers
        1.0 / inv_eta
    }
}
