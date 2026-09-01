pub const PI: f64 = core::f64::consts::PI;
pub const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;
pub const FRAC_PI_4: f64 = core::f64::consts::FRAC_PI_4;
pub const TWO_PI: f64 = 2.0 * PI;

/// `2pi` split so that `n * TWO_PI_HI` is exact for any integer `n` that fits
/// in 30 bits; the three parts sum to `2pi` to within 3.4e-31.
const TWO_PI_HI: f64 = 6.283185005187988;
const TWO_PI_MID: f64 = 3.0199157663446385e-7;
const TWO_PI_LO: f64 = 2.1561211432632476e-14;
const INV_TWO_PI: f64 = 0.15915494309189535;

/// `1.5 * 2^52`: adding then subtracting it rounds to the nearest integer
/// (ties to even) without calling a float-to-int conversion routine. Valid
/// for `|x| < 2^51`, which every caller here satisfies.
const ROUND_MAGIC: f64 = 6755399441055744.0;

#[inline]
pub fn abs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffff)
}

#[inline]
pub fn copysign(x: f64, sign: f64) -> f64 {
    f64::from_bits(
        (x.to_bits() & 0x7fff_ffff_ffff_ffff) | (sign.to_bits() & (1 << 63)),
    )
}

#[inline]
pub fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
pub fn clamp(x: f64, lo: f64, hi: f64) -> f64 {
    min(max(x, lo), hi)
}

/// Round half to even.
#[inline]
pub fn round(x: f64) -> f64 {
    (x + ROUND_MAGIC) - ROUND_MAGIC
}

#[inline]
pub fn round_to_i32(x: f64) -> i32 {
    (x + ROUND_MAGIC).to_bits() as u32 as i32
}

pub fn floor(x: f64) -> f64 {
    let r = round(x);
    if r > x {
        r - 1.0
    } else {
        r
    }
}

pub fn sqrt(x: f64) -> f64 {
    if !(x > 0.0) {
        return 0.0;
    }
    let mut y = f64::from_bits((x.to_bits() >> 1) + 0x1ff8_0000_0000_0000);
    y = 0.5 * (y + x / y);
    y = 0.5 * (y + x / y);
    y = 0.5 * (y + x / y);
    y = 0.5 * (y + x / y);
    0.5 * (y + x / y)
}

pub fn cbrt(x: f64) -> f64 {
    if x == 0.0 {
        return x;
    }
    let a = abs(x);
    let mut y = f64::from_bits(a.to_bits() / 3 + 0x2a9f_7625_3119_d328);
    y -= (y - a / (y * y)) * (1.0 / 3.0);
    y -= (y - a / (y * y)) * (1.0 / 3.0);
    y -= (y - a / (y * y)) * (1.0 / 3.0);
    y -= (y - a / (y * y)) * (1.0 / 3.0);
    y -= (y - a / (y * y)) * (1.0 / 3.0);
    copysign(y, x)
}

/// `x^(2/3)`
#[inline]
pub fn pow_2_3(x: f64) -> f64 {
    let c = cbrt(x);
    c * c
}

/// `x^(3/2)`
#[inline]
pub fn pow_3_2(x: f64) -> f64 {
    x * sqrt(x)
}

/// `x^(7/2)`
#[inline]
pub fn pow_7_2(x: f64) -> f64 {
    x * x * x * sqrt(x)
}

/// Fold into `[-pi, pi]`.
pub fn reduce_2pi(x: f64) -> f64 {
    let n = round(x * INV_TWO_PI);
    ((x - n * TWO_PI_HI) - n * TWO_PI_MID) - n * TWO_PI_LO
}

/// Fold into `[0, 2pi)`.
pub fn wrap_2pi(x: f64) -> f64 {
    let r = reduce_2pi(x);
    if r < 0.0 {
        r + TWO_PI
    } else {
        r
    }
}

// fdlibm minimax kernels, valid on |x| <= pi/4.
const S1: f64 = -1.66666666666666324348e-01;
const S2: f64 = 8.33333333332248946124e-03;
const S3: f64 = -1.98412698298579493134e-04;
const S4: f64 = 2.75573137070700676789e-06;
const S5: f64 = -2.50507602534068634195e-08;
const S6: f64 = 1.58969099521155010221e-10;

const C1: f64 = 4.16666666666666019037e-02;
const C2: f64 = -1.38888888888741095749e-03;
const C3: f64 = 2.48015872894767294178e-05;
const C4: f64 = -2.75573143513906633035e-07;
const C5: f64 = 2.08757232129817482790e-09;
const C6: f64 = -1.13596475577881948265e-11;

fn sin_kernel(x: f64) -> f64 {
    let z = x * x;
    x + x * z * (S1 + z * (S2 + z * (S3 + z * (S4 + z * (S5 + z * S6)))))
}

fn cos_kernel(x: f64) -> f64 {
    let z = x * x;
    1.0 - 0.5 * z
        + z * z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6)))))
}

/// Octant dispatch shared by [`sin`] and [`cos`]: reduces `x` to `[-pi, pi]`,
/// then to `[-pi/4, pi/4]` plus a quadrant index.
#[inline]
fn quadrant(x: f64) -> (f64, u32) {
    let r = reduce_2pi(x);
    let n = round(r * (2.0 / PI));
    // n * pi/2 with pi/2 split the same way 2pi is, so the product stays exact
    let q = ((r - n * (TWO_PI_HI * 0.25)) - n * (TWO_PI_MID * 0.25))
        - n * (TWO_PI_LO * 0.25);
    (q, (n as i32 & 3) as u32)
}

pub fn sin(x: f64) -> f64 {
    let (q, k) = quadrant(x);
    match k {
        0 => sin_kernel(q),
        1 => cos_kernel(q),
        2 => -sin_kernel(q),
        _ => -cos_kernel(q),
    }
}

pub fn cos(x: f64) -> f64 {
    let (q, k) = quadrant(x);
    match k {
        0 => cos_kernel(q),
        1 => -sin_kernel(q),
        2 => -cos_kernel(q),
        _ => sin_kernel(q),
    }
}

/// Returns `(sin x, cos x)` from one reduction
pub fn sin_cos(x: f64) -> (f64, f64) {
    let (q, k) = quadrant(x);
    let (s, c) = (sin_kernel(q), cos_kernel(q));
    match k {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        _ => (-c, s),
    }
}

// fdlibm minimax coefficients for atan on |x| <= 0.4375.
const A0: f64 = 3.33333333333329318027e-01;
const A1: f64 = -1.99999999998764832476e-01;
const A2: f64 = 1.42857142725034663711e-01;
const A3: f64 = -1.11111104054623557880e-01;
const A4: f64 = 9.09088713343650656196e-02;
const A5: f64 = -7.69187620504482999495e-02;
const A6: f64 = 6.66107313738753120669e-02;
const A7: f64 = -5.83357013379057348645e-02;
const A8: f64 = 4.97687799461593236017e-02;
const A9: f64 = -3.65315727442169155270e-02;
const A10: f64 = 1.62858201153657823623e-02;

fn atan_kernel(x: f64) -> f64 {
    let z = x * x;
    let p = A0
        + z * (A1
            + z * (A2
                + z * (A3
                    + z * (A4
                        + z * (A5
                            + z * (A6
                                + z * (A7
                                    + z * (A8 + z * (A9 + z * A10)))))))));
    x - x * z * p
}

/// tan(pi/8) and tan(3pi/8), the two split points that keep the kernel argument
/// inside its validity range.
const TAN_PI_8: f64 = 0.41421356237309503;
const TAN_3PI_8: f64 = 2.414213562373095;

pub fn atan(x: f64) -> f64 {
    let ax = abs(x);
    let r = if ax > TAN_3PI_8 {
        FRAC_PI_2 - atan_kernel(1.0 / ax)
    } else if ax > TAN_PI_8 {
        FRAC_PI_4 + atan_kernel((ax - 1.0) / (ax + 1.0))
    } else {
        atan_kernel(ax)
    };
    copysign(r, x)
}

pub fn atan2(y: f64, x: f64) -> f64 {
    if x > 0.0 {
        atan(y / x)
    } else if x < 0.0 {
        // Sign bit rather than `y >= 0.0`: it is the IEEE-correct branch for
        // a negative zero
        if y.to_bits() >> 63 == 0 {
            atan(y / x) + PI
        } else {
            atan(y / x) - PI
        }
    } else if y > 0.0 {
        FRAC_PI_2
    } else if y < 0.0 {
        -FRAC_PI_2
    } else {
        0.0
    }
}
