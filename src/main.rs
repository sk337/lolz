use cve_rs::transmute;
use euclid::Vector3D;

struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let vector = V3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let normalized = normalize(&vector);

    println!(
        "Normalized vector: x: {}, y: {}, z: {}",
        normalized.x, normalized.y, normalized.z
    );

    let lol = transmute::<V3, Vector3D<f32, u32>>(normalized);
    println!("Transmuted vector: {:#?}", lol);
}

fn normalize(v: &V3) -> V3 {
    let len_sqred = v.x * v.x + v.y * v.y + v.z * v.z;
    let inv_len = Q_rsqrt(len_sqred);
    V3 {
        x: v.x * inv_len,
        y: v.y * inv_len,
        z: v.z * inv_len,
    }
}

fn Q_rsqrt(num: f32) -> f32 {
    const THREEHALVES: f32 = 1.5;

    let x2 = num * 0.5;
    let mut y = num;
    let mut i = y.to_bits();
    i = 0x5f3759df - (i >> 1);
    y = f32::from_bits(i);
    y = y * (THREEHALVES - (x2 * y * y));
    y = y * (THREEHALVES - (x2 * y * y));

    y
}
