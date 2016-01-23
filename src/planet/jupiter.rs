//! Jupiter

use angle;
use ecliptic;
use nutation;
use planet;

/**
Returns Jupiter's **equatorial semidiameter**

# Returns

* ```eq_semidia```: Equatorial semidiameter *| in radians per AU*

# Arguments

* ```jup_earth_dist```: Jupiter-Earth distance *| in AU*
**/
pub fn eq_semdiameter(jup_earth_dist: f64) -> f64 {
    angle::DegFrmDMS(0, 0, 98.44) / jup_earth_dist
}

/**
Returns Jupiter's **polar semidiameter**

# Returns

* ```pol_semidia```: Polar semidiameter *| in radians per AU*

# Arguments

* ```jup_earth_dist```: Jupiter-Earth distance *| in AU*
**/
pub fn pol_semdiameter(jup_earth_dist: f64) -> f64 {
    angle::DegFrmDMS(0, 0, 92.06) / jup_earth_dist
}

/**
Return quantites used in the **ephemeris** for **physical observations**
of Jupiter

# Arguments

* ```JD```: Julian (Ephemeris) day
* ```mn_oblq```: Mean obliquity of the ecliptic on ```JD``` *| in radians*
* ```nut_in_long```: Nutation in ecliptic longitude on ```JD``` *| in radians*
* ```nut_in_oblq```: Nutation in obliquity of the ecliptic on ```JD``` *| in radians*
**/
pub fn ephemeris(JD: f64,
                mn_oblq_eclip: f64,
                nut_in_long: f64, nut_in_oblq: f64) -> (f64, f64, f64, f64, f64) {
    let d = JD - 2433282.5;
    let T1 = d / 36525.0;

    let asc0 = (268.0 + 0.1061*T1).to_radians();
    let dec0 = (64.5  - 0.0164*T1).to_radians();

    let W1 = angle::LimitTo360(17.710 + 877.90003539*d).to_radians();
    let W2 = angle::LimitTo360(16.838 + 870.27003539*d).to_radians();

    let (l0, b0, R) = planet::heliocen_pos(&planet::Planet::Earth, JD);

    let mut l = 0.0; let mut b = 0.0; let mut r = 0.0;
    let mut x = 0.0; let mut y = 0.0; let mut z = 0.0;
    let mut jup_earth_dist = 0.0;
    let mut light_time = 0.0;

    let mut i: u8 = 1;
    let n: u8 = 2;
    while i <= n {
        let (new_l, new_b, new_r) = planet::heliocen_pos(&planet::Planet::Jupiter, JD - light_time);
        l = new_l; b = new_b; r = new_r;

        let (new_x, new_y, new_z) = planet::geocen_ecl_rect_coords(l0, b0, R, l, b, r);
        x = new_x; y = new_y; z = new_z;

        jup_earth_dist = planet::dist_frm_ecl_rect_coords(x, y, z);
        light_time = planet::light_time(jup_earth_dist);

        i += 1;
    }

    l -= 0.01299_f64.to_radians()*jup_earth_dist / (r*r);
    let (x, y, z) = planet::geocen_ecl_rect_coords(l0, b0, R, l, b, r);
    jup_earth_dist = planet::dist_frm_ecl_rect_coords(x, y, z);

    let asc_s = (mn_oblq_eclip.cos()*l.sin() - mn_oblq_eclip.sin()*b.tan()).atan2(l.cos());
    let dec_s = (mn_oblq_eclip.cos()*b.sin() + mn_oblq_eclip.sin()*b.cos()*l.sin()).asin();

    let D_s = (-dec0.sin()*dec_s.sin() - dec0.cos()*dec_s.cos()*(asc0 - asc_s).cos()).asin();

    let u = y*mn_oblq_eclip.cos() - z*mn_oblq_eclip.sin();
    let v = y*mn_oblq_eclip.sin() + z*mn_oblq_eclip.cos();
    let mut asc = u.atan2(x);
    let mut dec = v.atan2((x*x + u*u).sqrt());
    let zeta = (dec0.sin()*dec.cos()*(asc0 - asc).cos() - dec.sin()*dec0.cos())
               .atan2(dec.cos()*(asc0 - asc).sin());

    let D_e = (-dec0.sin()*dec.sin() - dec0.cos()*dec.cos()*(asc0 - asc).cos()).asin();

    let mut w1 = angle::LimitTo360(W1.to_degrees() - zeta.to_degrees() - 5.07033*jup_earth_dist);
    let mut w2 = angle::LimitTo360(W2.to_degrees() - zeta.to_degrees() - 5.02626*jup_earth_dist);

    let mut C = 57.2958 * (2.0*r*jup_earth_dist + R*R - r*r - jup_earth_dist*jup_earth_dist)
                / (4.0*r*jup_earth_dist);
    if (l - l0).sin() < 0.0 {
        C *= -1.0
    }
    w1 = (w1 + C).to_radians();
    w2 = (w2 + C).to_radians();

    let tru_oblq_eclip = mn_oblq_eclip + nut_in_oblq;

    let q = 0.005693_f64.to_radians();
    asc += q * (asc.cos()*l0.cos()*tru_oblq_eclip.cos() + asc.sin()*l0.sin()) / dec.cos();
    dec += q * (  l0.cos()*tru_oblq_eclip.cos()*(tru_oblq_eclip.tan()*dec.cos()
                - asc.sin()*asc.cos())
                + asc.cos()*dec.sin()*l0.sin());

    let (asc_nut, dec_nut) = nutation::nutation_in_eq_coords(asc, dec, nut_in_long,
                                                          nut_in_oblq, tru_oblq_eclip);
    let asc1 = asc + asc_nut;
    let dec1 = dec + dec_nut;

    let (asc0_nut, dec0_nut) = nutation::nutation_in_eq_coords(asc0, dec0, nut_in_long,
                                                            nut_in_oblq, tru_oblq_eclip);
    let asc01 = asc0 + asc0_nut;
    let dec01 = dec0 + dec0_nut;

    let P = (dec01.cos() * (asc01 - asc1).sin())
            .atan2(dec01.sin()*dec1.cos() - dec01.cos()*dec1.sin()*(asc01 - asc1).cos());

    (D_e, D_s, w1, w2, P)
}

fn Io() {

}

fn Europa() {

}

fn Ganymede() {

}

fn Callisto() {

}
