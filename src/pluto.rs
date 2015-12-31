use angle;

/**
Computes Pluto's **equatorial semidiameter**

# Arguments

* ```distance_to_earth```: Pluto's distance from Earth *(AU)*
**/
pub fn Semidiameter(distance_to_earth: f64) -> f64 {
    angle::PureDegrees(0.0, 0.0, 2.07) / distance_to_earth
}

/**
Computes the **heliocentric coordinates** of Pluto. **Valid only for the years 1885 AD - 2099 AD.**

The coordinates are with reference to the epoch J2000.0.

# Return variables

```(longitude, latitude, radius_vector)```

* ```longitude```: Heliocentric longitude *(radians)*
* ```latitude```: Heliocentric latitude *(radians)*
* ```radius_vector```: Heliocentric radius vector *(radians)*

# Arguments

```julian_century```: Julian century
**/

pub fn HeliocentricCoords(julian_century: f64) -> (f64, f64, f64) {

    struct terms(i8, i8, i8, f64, f64, f64, f64, f64, f64);
    let tuple_terms = [
        terms(0, 0, 1, -19.799805, 19.850055, -5.452852, -14.974862, 6.6865439, 6.8951812),
        terms(0, 0, 2, 0.897144, -4.954829, 3.527812, 1.672790, -1.1827535, -0.0332538),
        terms(0, 0, 3, 0.611149, 1.211027, -1.050748, 0.327647, 0.1593179, -0.1438890),
        terms(0, 0, 4, -0.341243, -0.189585, 0.178690, -0.292153, -0.0018444, 0.0483220),
        terms(0, 0, 5, 0.129287, -0.034992, 0.018650, 0.100340, -0.0065977, -0.0085431),
        terms(0, 0, 6, -0.038164, 0.030893, -0.030697, -0.025823, 0.0031174, -0.0006032),
        terms(0, 1, -1, 00.020442, -0.009987, 0.004878, 0.011248, -0.0005794, 0.0022161),
      	terms(0, 1, 0, -0.004063, -0.005071, 0.000226, -0.000064, 0.0004601, 0.0004032),
      	terms(0, 1, 1, -0.006016, -0.003336, 0.00203, -0.000836, -0.0001729, 0.0000234),
      	terms(0, 1, 2, -0.003956, 0.003039, 0.000069, -0.000604, -0.0000415, 0.0000702),
      	terms(0, 1, 3, -0.000667, 0.003572, -0.000247, -0.000567, 0.0000239, 0.0000723),
      	terms(0, 2, -2, 0.001276, 0.000501, -0.000057, 0.000001, 0.0000067, -0.0000067),
      	terms(0, 2, -1, 0.001152, -0.000917, -0.000122, 0.000175, 0.0001034, -0.0000451),
      	terms(0, 2, 0, 0.00063, -0.001277, -0.000049, -0.000164, -0.0000129, 0.0000504),
      	terms(1, -1, 0, 0.002571, -0.000459, -0.000197, 0.000199, 0.000048, -0.0000231),
      	terms(1, -1, 1, 0.000899, -0.001449, -0.000025, 0.000217, 0.0000002, -0.0000441),
      	terms(1, 0, -3, -0.001016, 0.001043, 0.000589, -0.000248, -0.0003359, 0.0000265),
      	terms(1, 0, -2, -0.002343, -0.001012, -0.000269, 0.000711, 0.0007856, -0.0007832),
      	terms(1, 0, -1, 0.007042, 0.000788, 0.000185, 0.000193, 0.0000036, 0.0045763),
      	terms(1, 0, 0, 0.001199, -0.000338, 0.000315, 0.000807, 0.0008663, 0.0008547),
      	terms(1, 0, 1, 0.000418, -0.000067, -0.00013, -0.000043, -0.0000809, -0.0000769),
      	terms(1, 0, 2, 0.00012, -0.000274, 0.000005, 0.000003, 0.0000263, -0.0000144),
      	terms(1, 0, 3, -0.00006, -0.000159, 0.000002, 0.000017, -0.0000126, 0.0000032),
      	terms(1, 0, 4, -0.000082, -0.000029, 0.000002, 0.000005, -0.0000035, -0.0000016),
      	terms(1, 1, -3, -0.000036, -0.000029, 0.000002, 0.000003, -0.0000019, -0.0000004),
      	terms(1, 1, -2, -0.00004, 0.000007, 0.000003, 0.000001, -0.0000015, 0.0000008),
      	terms(1, 1, -1, -0.000014, 0.000022, 0.000002, -0.000001, -0.0000004, 0.0000012),
      	terms(1, 1, 0, 0.000004, 0.000013, 0.000001, -0.000001, 0.0000005, 0.0000006),
      	terms(1, 1, 1, 0.000005, 0.000002, 0.0, -0.000001, 0.0000003, 0.0000001),
      	terms(1, 1, 3, -0.000001, 0.0, 0.0, 0.0, 0.0000006, -0.0000002),
      	terms(2, 0, -6, 0.000002, 0.0, 0.0, -0.000002, 0.0000002, 0.0000002),
      	terms(2, 0, -5, -0.000004, 0.000005, 0.000002, 0.000002, -0.0000002, -0.0000002),
      	terms(2, 0, -4, 0.000004, -0.000007, -0.000007, 0.0, 0.0000014, 0.0000013),
      	terms(2, 0, -3, 0.000014, 0.000024, 0.00001, -0.000008, -0.0000063, 0.0000013),
      	terms(2, 0, -2, -0.000049, -0.000034, -0.000003, 0.00002, 0.0000136, -0.0000236),
      	terms(2, 0, -1, 0.000163, -0.000048, 0.000006, 0.000005, 0.0000273, 0.0001065),
      	terms(2, 0, 0, 0.000009, -0.000024, 0.000014, 0.000017, 0.0000251, 0.0000149),
      	terms(2, 0, 1, -0.000004, 0.000001, -0.000002, 0.0, -0.0000025, -0.0000009),
      	terms(2, 0, 2, -0.000003, 0.000001, 0.0, 0.0, 0.0000009, -0.0000002),
      	terms(2, 0, 3, 0.000001, 0.000003, 0.0, 0.0, -0.0000008, 0.0000007),
      	terms(3, 0, -2, -0.000003, -0.000001, 0.0, 0.000001, 0.0000002, -0.000001),
      	terms(3, 0, -1, 0.000005, -0.000003, 0.0, 0.0, 0.0000019, 0.0000035),
      	terms(3, 0, 0, 0.0, 0.0, 0.000001, 0.0, 0.000001, 0.0000003)];

    let j = 34.35 + 3034.9057*julian_century;
    let s = 50.08 + 1222.1138*julian_century;
    let p = 238.96 + 144.9600*julian_century;

    let mut long = (238.958116f64 + 144.96*julian_century).to_radians();
    let mut lat = -3.908239_f64.to_radians();
    let mut r = 40.7241346;

    for x in tuple_terms.iter() {
        let alpha = ((x.0 as f64)*j + (x.1 as f64)*s + (x.2 as f64)*p).to_radians();
        let alpha_sin = alpha.sin();
        let alpha_cos = alpha.cos();
        long += x.3.to_radians()*alpha_sin + x.4.to_radians()*alpha_cos;
        lat += x.5.to_radians()*alpha_sin + x.6.to_radians()*alpha_cos;
        r += x.7*alpha_sin + x.8*alpha_cos;
    }

    (long, lat, r)
}
