use nalgebra::{Vector4};

pub fn calc_4d_matrix(
    from: Vector4<f32>,
    to: Vector4<f32>,
    up: Vector4<f32>,
    over: Vector4<f32>,
) -> (Vector4<f32>, Vector4<f32>, Vector4<f32>, Vector4<f32>) {
    let wd = to - from;
    let norm = norm4(&wd);
    let wd = wd.scale(1.0 / norm);

    let wa = cross4(&up, &over, &wd);
    let norm = norm4(&wa);
    let wa = wa.scale(1.0 / norm);

    let wb = cross4(&over, &wd, &wa);
    let norm = norm4(&wb);
    let wb = wb.scale(1.0 / norm);

    let wc = cross4(&wd, &wa, &wb);

    (wa, wb, wc, wd)
}

fn cross4(u: &Vector4<f32>, v: &Vector4<f32>, w: &Vector4<f32>) -> Vector4<f32> {
    let a = (v[0] * w[1]) - (v[1] * w[0]);
    let b = (v[0] * w[2]) - (v[2] * w[0]);
    let c = (v[0] * w[3]) - (v[3] * w[0]);
    let d = (v[1] * w[2]) - (v[2] * w[1]);
    let e = (v[1] * w[3]) - (v[3] * w[1]);
    let f = (v[2] * w[3]) - (v[3] * w[2]);

    Vector4::new(
        (u[1] * f) - (u[2] * e) + (u[3] * d),
        -(u[0] * f) + (u[2] * c) - (u[3] * b),
        (u[0] * e) - (u[1] * c) + (u[3] * a),
        -(u[0] * d) + (u[1] * b) - (u[2] * a),
    )
}

pub fn dot4(v: &Vector4<f32>, u: &Vector4<f32>) -> f32 {
    (v[0] * u[0]) + (v[1] * u[1]) + (v[2] * u[2]) + (v[3] * u[3])
}

fn norm4(v: &Vector4<f32>) -> f32 {
    dot4(v, v).sqrt()
}
