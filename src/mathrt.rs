//! EXPERIMENT ONLY - size probe
#![allow(dead_code)]

extern "C" fn dadd(a: f64, b: f64) -> f64 { a + b }
extern "C" fn dsub(a: f64, b: f64) -> f64 { a - b }
extern "C" fn dmul(a: f64, b: f64) -> f64 { a * b }
extern "C" fn ddiv(a: f64, b: f64) -> f64 { a / b }
extern "C" fn dcmp(a: f64, b: f64) -> i32 { if a < b { -1 } else if a > b { 1 } else if a == b { 0 } else { 2 } }
extern "C" fn i2d(a: i32) -> f64 { a as f64 }
extern "C" fn u2d(a: u32) -> f64 { a as f64 }
extern "C" fn l2d(a: i64) -> f64 { a as f64 }
extern "C" fn d2i(a: f64) -> i32 { a as i32 }
extern "C" fn d2l(a: f64) -> i64 { a as i64 }
extern "C" fn d2f(a: f64) -> f32 { a as f32 }
extern "C" fn f2d(a: f32) -> f64 { a as f64 }

#[repr(C)]
pub struct SoftTable {
    pub dadd: extern "C" fn(f64, f64) -> f64,
    pub dsub: extern "C" fn(f64, f64) -> f64,
    pub dmul: extern "C" fn(f64, f64) -> f64,
    pub ddiv: extern "C" fn(f64, f64) -> f64,
    pub dcmp: extern "C" fn(f64, f64) -> i32,
    pub i2d: extern "C" fn(i32) -> f64,
    pub u2d: extern "C" fn(u32) -> f64,
    pub l2d: extern "C" fn(i64) -> f64,
    pub d2i: extern "C" fn(f64) -> i32,
    pub d2l: extern "C" fn(f64) -> i64,
    pub d2f: extern "C" fn(f64) -> f32,
    pub f2d: extern "C" fn(f32) -> f64,
}

#[used]
#[unsafe(no_mangle)]
pub static MATH_SOFT: SoftTable = SoftTable {
    dadd, dsub, dmul, ddiv, dcmp, i2d, u2d, l2d, d2i, d2l, d2f, f2d,
};

#[cfg(feature = "mathlibm")]
mod trig {
    extern "C" fn m_sin(a: f64) -> f64 { libm::sin(a) }
    extern "C" fn m_cos(a: f64) -> f64 { libm::cos(a) }
    extern "C" fn m_atan2(a: f64, b: f64) -> f64 { libm::atan2(a, b) }
    extern "C" fn m_sqrt(a: f64) -> f64 { libm::sqrt(a) }
    extern "C" fn m_floor(a: f64) -> f64 { libm::floor(a) }
    #[repr(C)]
    pub struct TrigTable {
        pub sin: extern "C" fn(f64) -> f64,
        pub cos: extern "C" fn(f64) -> f64,
        pub atan2: extern "C" fn(f64, f64) -> f64,
        pub sqrt: extern "C" fn(f64) -> f64,
        pub floor: extern "C" fn(f64) -> f64,
    }
    #[used]
    #[unsafe(no_mangle)]
    pub static MATH_TRIG: TrigTable = TrigTable {
        sin: m_sin, cos: m_cos, atan2: m_atan2, sqrt: m_sqrt, floor: m_floor,
    };
}

#[cfg(feature = "mathpow")]
mod powmod {
    extern "C" fn m_pow(a: f64, b: f64) -> f64 { libm::pow(a, b) }
    extern "C" fn m_fmod(a: f64, b: f64) -> f64 { libm::fmod(a, b) }
    extern "C" fn m_asin(a: f64) -> f64 { libm::asin(a) }
    #[repr(C)]
    pub struct PowTable {
        pub pow: extern "C" fn(f64, f64) -> f64,
        pub fmod: extern "C" fn(f64, f64) -> f64,
        pub asin: extern "C" fn(f64) -> f64,
    }
    #[used]
    #[unsafe(no_mangle)]
    pub static MATH_POW: PowTable = PowTable { pow: m_pow, fmod: m_fmod, asin: m_asin };
}
