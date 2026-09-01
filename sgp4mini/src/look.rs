use crate::math::{atan2, cos, sin, sin_cos, sqrt, TWO_PI};

const AE: f64 = 6378.137;
/// WGS84 flattening
const F: f64 = 1.0 / 298.257223563;
/// earth rotation rate, rad/s
const OMEGA_E: f64 = 7.292115e-5;
/// speed of light, km/s
const C_KM_S: f64 = 299_792.458;

#[derive(Debug, Clone, Copy)]
pub struct Observer {
    /// rad, north positive
    pub latitude: f64,
    /// rad, east positive
    pub longitude: f64,
    /// km above the ellipsoid
    pub altitude_km: f64,
    sin_lat: f64,
    cos_lat: f64,
    /// distance from the spin axis, km
    rho_km: f64,
    /// height above the equatorial plane, km
    z_km: f64,
}

impl Observer {
    pub fn new(latitude: f64, longitude: f64, altitude_km: f64) -> Observer {
        let (sin_lat, cos_lat) = sin_cos(latitude);
        // radius of curvature in the prime vertical, and its polar companion
        let c = 1.0 / sqrt(1.0 - F * (2.0 - F) * sin_lat * sin_lat);
        let s = c * (1.0 - F) * (1.0 - F);
        Observer {
            latitude,
            longitude,
            altitude_km,
            sin_lat,
            cos_lat,
            rho_km: (AE * c + altitude_km) * cos_lat,
            z_km: (AE * s + altitude_km) * sin_lat,
        }
    }
}

/// Where a satellite is as seen from the ground.
#[derive(Debug, Clone, Copy)]
pub struct Look {
    /// rad, clockwise from north, in `[0, 2pi)`
    pub azimuth: f64,
    /// rad, positive above the horizon
    pub elevation: f64,
    /// km
    pub range_km: f64,
    /// km/s, positive while receding
    pub range_rate_km_s: f64,
}

/// Sub-satellite point and height.
#[derive(Debug, Clone, Copy)]
pub struct SubPoint {
    /// rad, north positive (geodetic)
    pub latitude: f64,
    /// rad, east positive, in `[-pi, pi]`
    pub longitude: f64,
    /// km above the ellipsoid
    pub altitude_km: f64,
}

/// `position`/`velocity` are TEME km and km/s, `gmst` the sidereal time for
/// the same instant.
pub fn look(
    observer: &Observer,
    gmst: f64,
    position: [f64; 3],
    velocity: [f64; 3],
) -> Look {
    // observer in TEME
    let theta = gmst + observer.longitude;
    let (sin_th, cos_th) = sin_cos(theta);
    let ox = observer.rho_km * cos_th;
    let oy = observer.rho_km * sin_th;
    let oz = observer.z_km;
    // its velocity is pure rotation about the spin axis
    let ovx = -OMEGA_E * oy;
    let ovy = OMEGA_E * ox;

    let rx = position[0] - ox;
    let ry = position[1] - oy;
    let rz = position[2] - oz;
    let vx = velocity[0] - ovx;
    let vy = velocity[1] - ovy;
    let vz = velocity[2];

    let range = sqrt(rx * rx + ry * ry + rz * rz);

    // rotate the range vector into the topocentric south-east-zenith frame
    let south = observer.sin_lat * cos_th * rx + observer.sin_lat * sin_th * ry
        - observer.cos_lat * rz;
    let east = -sin_th * rx + cos_th * ry;
    let zenith = observer.cos_lat * cos_th * rx
        + observer.cos_lat * sin_th * ry
        + observer.sin_lat * rz;

    // azimuth runs clockwise from north, and north is −south
    let mut azimuth = atan2(east, -south);
    if azimuth < 0.0 {
        azimuth += TWO_PI;
    }

    Look {
        azimuth,
        elevation: atan2(zenith, sqrt(south * south + east * east)),
        range_km: range,
        range_rate_km_s: (rx * vx + ry * vy + rz * vz) / range,
    }
}

/// Geodetic sub-satellite point, iterating the standard latitude fixed point.
pub fn sub_point(gmst: f64, position: [f64; 3]) -> SubPoint {
    let [x, y, z] = position;
    let r_xy = sqrt(x * x + y * y);
    let mut latitude = atan2(z, r_xy);
    let mut c = 1.0;
    // five passes is plenty: the iteration converges geometrically with a
    // ratio near the flattening, 1/298
    for _ in 0..5 {
        let sin_lat = sin(latitude);
        c = 1.0 / sqrt(1.0 - F * (2.0 - F) * sin_lat * sin_lat);
        latitude = atan2(z + AE * c * F * (2.0 - F) * sin_lat, r_xy);
    }
    let mut longitude = atan2(y, x) - gmst;
    // fold into [−pi, pi] without a fmod: the argument is at most a few turns
    while longitude > core::f64::consts::PI {
        longitude -= TWO_PI;
    }
    while longitude < -core::f64::consts::PI {
        longitude += TWO_PI;
    }
    SubPoint {
        latitude,
        longitude,
        altitude_km: r_xy / cos(latitude) - AE * c,
    }
}

/// Received-frequency offset in Hz for a transmitter at `freq_hz`.
///
/// Negative while the satellite recedes, which is the sign the radio has to
/// add to its dial frequency.
pub fn doppler_hz(freq_hz: f64, range_rate_km_s: f64) -> f64 {
    -freq_hz * range_rate_km_s / C_KM_S
}
