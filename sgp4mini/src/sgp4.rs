use crate::math::{
    abs, atan2, clamp, cos, max, pow_2_3, pow_3_2, pow_7_2, reduce_2pi, sin,
    sin_cos, sqrt, TWO_PI,
};

// WGS84
const AE: f64 = 6378.137;
const KE: f64 = 0.07436685316871385;
const J2: f64 = 0.00108262998905;
const J3: f64 = -0.00000253215306;
const J4: f64 = -0.00000161098761;

/// Below this mean motion (rad/min) the orbit is deep space: 2pi/225 min.
const DEEP_SPACE_LIMIT: f64 = TWO_PI / 225.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Period over 225 min, for SDP4 not implemented.
    DeepSpace,
    /// Epoch eccentricity outside `[0, 1)`, or a non-positive mean motion.
    BadElements,
    /// Drag drove the eccentricity out of range: the satellite has decayed
    /// past the point where the model means anything.
    Decayed,
}

/// Orbital elements as they come off a TLE, already converted to radians.
///
/// `mean_motion` is the Kozai convention (what the TLE carries); the
/// Brouwer conversion happens in [`Constants::new`].
#[derive(Debug, Clone, Copy)]
pub struct Elements {
    /// days since J2000 (JD − 2451545.0), UTC
    pub epoch_days_j2000: f64,
    /// rad
    pub inclination: f64,
    /// rad
    pub right_ascension: f64,
    pub eccentricity: f64,
    /// rad
    pub argument_of_perigee: f64,
    /// rad
    pub mean_anomaly: f64,
    /// rad/min, Kozai convention
    pub mean_motion: f64,
    /// B*, earth radii⁻¹
    pub drag_term: f64,
}

/// Position and velocity in the TEME-of-epoch frame.
#[derive(Debug, Clone, Copy)]
pub struct Prediction {
    /// km
    pub position: [f64; 3],
    /// km/s
    pub velocity: [f64; 3],
}

/// Everything derived from the epoch elements that propagation reuses.
///
/// Built once per satellite, then read-only. The `high_*` and `elliptic_*`
/// groups are the crate's `HighAltitude`/`Elliptic` enums flattened into
/// plain fields plus a flag
#[derive(Debug, Clone, Copy)]
pub struct Constants {
    // epoch orbit, Brouwer mean motion
    pub(crate) inclination_0: f64,
    pub(crate) right_ascension_0: f64,
    pub(crate) eccentricity_0: f64,
    pub(crate) argument_of_perigee_0: f64,
    pub(crate) mean_anomaly_0: f64,
    pub(crate) mean_motion_0: f64,

    pub(crate) right_ascension_dot: f64,
    pub(crate) argument_of_perigee_dot: f64,
    pub(crate) mean_anomaly_dot: f64,
    pub(crate) c1: f64,
    pub(crate) c4: f64,
    pub(crate) k0: f64,
    pub(crate) k1: f64,

    pub(crate) a0: f64,
    pub(crate) k2: f64,
    pub(crate) k5: f64,

    /// perigee at or above 220 km: the extended drag terms apply
    pub(crate) high_altitude: bool,
    pub(crate) c5: f64,
    pub(crate) d2: f64,
    pub(crate) d3: f64,
    pub(crate) d4: f64,
    pub(crate) eta: f64,
    pub(crate) k7: f64,
    pub(crate) k8: f64,
    pub(crate) k9: f64,
    pub(crate) k10: f64,

    /// `e_0 > 10^{-4}`: the long-period perigee correction applies
    pub(crate) elliptic: bool,
    pub(crate) k11: f64,
    pub(crate) k12: f64,
    pub(crate) k13: f64,
}

impl Constants {
    pub fn new(elements: &Elements) -> Result<Constants, Error> {
        if elements.mean_motion <= 0.0
            || elements.eccentricity < 0.0
            || elements.eccentricity >= 1.0
        {
            return Err(Error::BadElements);
        }

        let e0 = elements.eccentricity;
        let (sin_i0, cos_i0) = sin_cos(elements.inclination);

        let p1 = cos_i0;
        let p2 = 1.0 - e0 * e0;
        let k6 = 3.0 * p1 * p1 - 1.0;

        let mean_motion = {
            let a1 = pow_2_3(KE / elements.mean_motion);
            let p0 = 0.75 * J2 * k6 / pow_3_2(p2);
            let d1 = p0 / (a1 * a1);
            let inner = a1
                * (1.0 - d1 * d1 - d1 * (1.0 / 3.0 + 134.0 * d1 * d1 / 81.0));
            let d0 = p0 / (inner * inner);
            elements.mean_motion / (1.0 + d0)
        };
        if mean_motion <= 0.0 {
            return Err(Error::BadElements);
        }
        if mean_motion <= DEEP_SPACE_LIMIT {
            return Err(Error::DeepSpace);
        }

        let a0 = pow_2_3(KE / mean_motion);
        let p3 = a0 * (1.0 - e0);

        let p4 = AE * (p3 - 1.0);
        let p5 = if p4 < 98.0 {
            20.0
        } else if p4 < 156.0 {
            p4 - 78.0
        } else {
            78.0
        };

        let s = p5 / AE + 1.0;
        let p6 = {
            let q = (120.0 - p5) / AE;
            let q2 = q * q;
            q2 * q2
        };

        let xi = 1.0 / (a0 - s);
        let xi2 = xi * xi;
        let p7 = p6 * xi2 * xi2;
        let eta = a0 * e0 * xi;
        let eta2 = eta * eta;

        let p8 = abs(1.0 - eta2);
        let p9 = p7 / pow_7_2(p8);

        let c1 = elements.drag_term
            * (p9
                * mean_motion
                * (a0 * (1.0 + 1.5 * eta2 + e0 * eta * (4.0 + eta2))
                    + 0.375 * J2 * xi / p8
                        * k6
                        * (8.0 + 3.0 * eta2 * (8.0 + eta2))));

        let p10 = {
            let t = a0 * p2;
            1.0 / (t * t)
        };
        let p11 = 1.5 * J2 * p10 * mean_motion;
        let p12 = 0.5 * p11 * J2 * p10;
        let p13 = -0.46875 * J4 * p10 * p10 * mean_motion;
        let p1_2 = p1 * p1;
        let p1_4 = p1_2 * p1_2;
        let b0 = sqrt(p2);

        let p14 = -p11 * p1
            + (0.5 * p12 * (4.0 - 19.0 * p1_2)
                + 2.0 * p13 * (3.0 - 7.0 * p1_2))
                * p1;

        let k14 = -0.5 * p11 * (1.0 - 5.0 * p1_2)
            + 0.0625 * p12 * (7.0 - 114.0 * p1_2 + 395.0 * p1_4)
            + p13 * (3.0 - 36.0 * p1_2 + 49.0 * p1_4);

        let p15 = mean_motion
            + 0.5 * p11 * b0 * k6
            + 0.0625 * p12 * b0 * (13.0 - 78.0 * p1_2 + 137.0 * p1_4);

        let c4 = elements.drag_term
            * (2.0
                * mean_motion
                * p9
                * a0
                * p2
                * (eta * (2.0 + 0.5 * eta2) + e0 * (0.5 + 2.0 * eta2)
                    - J2 * xi / (a0 * p8)
                        * (-3.0
                            * k6
                            * (1.0 - 2.0 * e0 * eta
                                + eta2 * (1.5 - 0.5 * e0 * eta))
                            + 0.75
                                * (1.0 - p1_2)
                                * (2.0 * eta2 - e0 * eta * (1.0 + eta2))
                                * cos(2.0 * elements.argument_of_perigee))));

        let k0 = 3.5 * p2 * (-p11 * p1) * c1;
        let k1 = 1.5 * c1;

        let mut c = Constants {
            inclination_0: elements.inclination,
            right_ascension_0: elements.right_ascension,
            eccentricity_0: e0,
            argument_of_perigee_0: elements.argument_of_perigee,
            mean_anomaly_0: elements.mean_anomaly,
            mean_motion_0: mean_motion,
            right_ascension_dot: p14,
            argument_of_perigee_dot: k14,
            mean_anomaly_dot: p15,
            c1,
            c4,
            k0,
            k1,
            a0,
            k2: -0.5 * (J3 / J2) * sin_i0,
            k5: -0.25 * (J3 / J2) * sin_i0 * (3.0 + 5.0 * p1)
                / if abs(1.0 + p1) > 1.5e-12 {
                    1.0 + p1
                } else {
                    1.5e-12
                },
            high_altitude: false,
            c5: 0.0,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
            eta,
            k7: 0.0,
            k8: 0.0,
            k9: 0.0,
            k10: 0.0,
            elliptic: false,
            k11: 0.0,
            k12: 0.0,
            k13: 0.0,
        };

        if p3 >= 220.0 / AE + 1.0 {
            c.high_altitude = true;
            let c1_2 = c1 * c1;
            let d2 = 4.0 * a0 * xi * c1_2;
            let p16 = d2 * xi * c1 / 3.0;
            let d3 = (17.0 * a0 + s) * p16;
            let d4 = 0.5 * p16 * a0 * xi * (221.0 * a0 + 31.0 * s) * c1;
            let (sin_m0, cos_m0) = sin_cos(elements.mean_anomaly);

            c.c5 = elements.drag_term
                * (2.0
                    * p9
                    * a0
                    * p2
                    * (1.0 + 2.75 * (eta2 + eta * e0) + eta * e0 * eta2));
            c.d2 = d2;
            c.d3 = d3;
            c.d4 = d4;
            c.k7 = sin_m0;
            c.k8 = d2 + 2.0 * c1_2;
            c.k9 = 0.25 * (3.0 * d3 + c1 * (12.0 * d2 + 10.0 * c1_2));
            c.k10 = 0.2
                * (3.0 * d4
                    + 12.0 * c1 * d3
                    + 6.0 * d2 * d2
                    + 15.0 * c1_2 * (2.0 * d2 + c1_2));

            if e0 > 1.0e-4 {
                c.elliptic = true;
                let t = 1.0 + eta * cos_m0;
                c.k11 = t * t * t;
                c.k12 = elements.drag_term
                    * (-2.0 * p7 * xi * (J3 / J2) * mean_motion * sin_i0 / e0)
                    * cos(elements.argument_of_perigee);
                c.k13 = -2.0 / 3.0 * p7 * elements.drag_term / (e0 * eta);
            }
        }

        Ok(c)
    }

    /// `t` is minutes since the elements' epoch; it may be negative.
    pub fn propagate(&self, t: f64) -> Result<Prediction, Error> {
        let t2 = t * t;

        let p22 = self.right_ascension_0
            + self.right_ascension_dot * t
            + self.k0 * t2;
        let p23 = self.argument_of_perigee_0 + self.argument_of_perigee_dot * t;
        let p24 = self.mean_anomaly_0 + self.mean_anomaly_dot * t;

        let (argument_of_perigee, mean_anomaly, a, p27) = if !self.high_altitude
        {
            (
                p23,
                p24 + self.mean_motion_0 * self.k1 * t2,
                {
                    let f = 1.0 - self.c1 * t;
                    self.a0 * f * f
                },
                self.eccentricity_0 - self.c4 * t,
            )
        } else {
            let (arg, p26) = if self.elliptic {
                let u = 1.0 + self.eta * cos(p24);
                let p25 = self.k13 * (u * u * u - self.k11) + self.k12 * t;
                (p23 - p25, p24 + p25)
            } else {
                (p23, p24)
            };
            let t3 = t2 * t;
            let t4 = t2 * t2;
            (
                arg,
                p26 + self.mean_motion_0
                    * (self.k1 * t2
                        + self.k8 * t3
                        + t4 * (self.k9 + t * self.k10)),
                {
                    let f = 1.0
                        - self.c1 * t
                        - self.d2 * t2
                        - self.d3 * t3
                        - self.d4 * t4;
                    self.a0 * f * f
                },
                self.eccentricity_0
                    - (self.c4 * t + self.c5 * (sin(p26) - self.k7)),
            )
        };

        if !(-0.001..1.0).contains(&p27) {
            return Err(Error::Decayed);
        }
        let eccentricity = max(p27, 1.0e-6);
        let mean_motion = KE / pow_3_2(a);

        let p37 = 1.0 / (a * (1.0 - eccentricity * eccentricity));
        let (sin_w, cos_w) = sin_cos(argument_of_perigee);

        let axn = eccentricity * cos_w;
        let ayn = eccentricity * sin_w + p37 * self.k2;

        let p38 = reduce_2pi(
            mean_anomaly + argument_of_perigee + p37 * self.k5 * axn,
        );

        let mut ew = p38;
        for _ in 0..10 {
            let (sin_ew, cos_ew) = sin_cos(ew);
            let delta = (p38 - ayn * cos_ew + axn * sin_ew - ew)
                / (1.0 - cos_ew * axn - sin_ew * ayn);
            if abs(delta) < 1.0e-12 {
                break;
            }
            ew += clamp(delta, -0.95, 0.95);
        }
        let (sin_ew, cos_ew) = sin_cos(ew);

        let p39 = axn * axn + ayn * ayn;

        let pl = a * (1.0 - p39);
        if pl < 0.0 {
            return Err(Error::Decayed);
        }

        let p40 = axn * sin_ew - ayn * cos_ew;

        let r = a * (1.0 - (axn * cos_ew + ayn * sin_ew));
        let r_dot = sqrt(a) * p40 / r;
        let b = sqrt(1.0 - p39);
        let p41 = p40 / (1.0 + b);
        let p42 = a / r * (sin_ew - ayn - axn * p41);
        let p43 = a / r * (cos_ew - axn + ayn * p41);

        let u = atan2(p42, p43);
        let p44 = 2.0 * p43 * p42;
        let p45 = 1.0 - 2.0 * p42 * p42;

        let (sin_i, cos_i) = sin_cos(self.inclination_0);
        let cos_i2 = cos_i * cos_i;
        let k3 = sin_i * sin_i;
        let k4 = 7.0 * cos_i2 - 1.0;
        let k6 = 3.0 * cos_i2 - 1.0;

        let half_j2_over_pl = 0.5 * J2 / pl;
        let p46 = half_j2_over_pl / pl;
        let rk =
            r * (1.0 - 1.5 * p46 * b * k6) + 0.5 * half_j2_over_pl * k3 * p45;
        let uk = u - 0.25 * p46 * k4 * p44;
        let inclination_k =
            self.inclination_0 + 1.5 * p46 * cos_i * sin_i * p45;
        let right_ascension_k = p22 + 1.5 * p46 * cos_i * p44;
        let rk_dot = r_dot - mean_motion * half_j2_over_pl * k3 * p44 / KE;
        let rfk_dot = sqrt(pl) / r
            + mean_motion * half_j2_over_pl * (k3 * p45 + 1.5 * k6) / KE;

        let (sin_uk, cos_uk) = sin_cos(uk);
        let (sin_raan, cos_raan) = sin_cos(right_ascension_k);
        let (sin_ik, cos_ik) = sin_cos(inclination_k);
        let u0 = -sin_raan * cos_ik * sin_uk + cos_raan * cos_uk;
        let u1 = cos_raan * cos_ik * sin_uk + sin_raan * cos_uk;
        let u2 = sin_ik * sin_uk;
        let v0 = -sin_raan * cos_ik * cos_uk - cos_raan * sin_uk;
        let v1 = cos_raan * cos_ik * cos_uk - sin_raan * sin_uk;
        let v2 = sin_ik * cos_uk;

        const VK: f64 = AE * KE / 60.0;
        Ok(Prediction {
            position: [rk * u0 * AE, rk * u1 * AE, rk * u2 * AE],
            velocity: [
                (rk_dot * u0 + rfk_dot * v0) * VK,
                (rk_dot * u1 + rfk_dot * v1) * VK,
                (rk_dot * u2 + rfk_dot * v2) * VK,
            ],
        })
    }
}
