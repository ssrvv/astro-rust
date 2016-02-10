# astro-rust

[![](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saurvs/astro-rust/blob/master/LICENSE.md)
[![](https://travis-ci.org/saurvs/astro-rust.svg?branch=master)](https://travis-ci.org/saurvs/astro-rust)
[![](http://meritbadge.herokuapp.com/astro)](https://crates.io/crates/astro)

**Contents**

* [About](#about)
* [Usage](#usage)
* [Algorithms](#algorithms)
* [Contributing](#contributing)
* [References](#references)

[API Docs](https://saurvs.github.io/astro-rust/)

> This project is currently a work in progress.
> Some APIs may be updated occasionally.

## About

```astro-rust``` is an MIT licensed library of astronomical algorithms for the Rust programming language.

Implemented capabilities include planetary, solar, lunar and planetary satellite positioning, corrections for precession, nutation, parallax, and aberration, calculation of physical ephemeris of Mars, Jupiter, and the ring system of Saturn, finding times of rise, set and transit of celestial bodies (and [much more](#algorithms)).

The main reference used as the source of algorithms is the famous book *Astronomical Algorithms by Jean Meeus*, whose almost every chapter has been addressed here, with functions that are well-documented and tests that use example data from the book; in some cases such as ΔT approximation and planetary heliocentric positioning, more accurate methods are used (from sources mentioned in the [references](#references) section).

The end goal (of this project) is to build a modern, well-tested, well-documented library of algorithms for future use in astronomy. And doing it all in Rust is very much a part of that.

## Usage

* Add the dependency ```astro``` in your ```Cargo.toml```
  ```toml
  [dependencies]
  astro = "1.0.1"
  ```

* Include the crate ```astro``` in your code
  ```rust
  extern crate astro;

  use astro::*;
  ```

* Find the Julian day (the most important step for almost everything)
  ```rust
  // time of the Apollo 11 moon landing

  let day_of_month = time::DayOfMonth{day: 20,
				 			          hr : 20,
                                      min: 18,
                                      sec: 4.0,
                                      time_zone: 0.0 // at Greenwhich
                                     };

  let date = time::Date{year       : 1969,
                        month      : 7, // July
                        decimal_day: time::decimal_day(&day_of_month),
                        cal_type   : time::CalType::Gregorian};

  let julian_day = time::julian_day(&date);

  // for higher accuracy in specifying the time of interest,
  // find the Julian Ephemeris day;
  // calculate delta T using the built-in function, or get an observed
  // value from the Astronomical Almanac

  let delta_t = time::approx_delta_t(date.year, date.month);

  let julian_ephm_day = time::julian_emph_day(julian_day, delta_t);
  ```

* Find the *geocentric* ecliptic point and radius vector for the Sun and the Moon
  ```rust

  // for the Sun

  let (sun_ecl_point, rad_vec_sun) = sun::geocen_ecl_pos(julian_day);

  // sun_ecl_point.long    - ecliptic longitude (radians)
  // sun_ecl_point.lat     - ecliptic latitude (radians)
  // rad_vec_sun - distance between the Sun and the Earth (AU)

  // similarly for the Moon

  let (moon_ecl_point, rad_vec_moon) = lunar::geocen_ecl_pos(julian_day);

  ```

* Find the *heliocentric* coordinates and radius vector for Jupiter
  ```rust
  let (long, lat, rad_vec) = planet::heliocen_pos(&planet::Planet::Jupiter, julian_day);
  ```

* Find the corrections for nutation in ecliptic longitude and obliquity of the ecliptic
  ```rust
  let (nut_in_long, nut_in_oblq) = nutation::nutation(julian_day);
  ```

* Find the geodesic distance between two locations on the Earth
  ```rust
	// geodesic distance between the Observatoire de Paris and
    // the US Naval Observatory at Washington DC

    let paris = coords::GeographPoint{long: angle::deg_frm_dms(-2, 20, 14.0).to_radians(),
                                      lat : angle::deg_frm_dms(48, 50, 11.0).to_radians()};

    let washington = coords::GeographPoint{long: angle::deg_frm_dms(77,  3, 56.0).to_radians(),
                                           lat : angle::deg_frm_dms(38, 55, 17.0).to_radians()};

	// angle::deg_frm_dms() converts degrees expressed in degrees,
	// minutes and seconds into a fractional degree

    let distance = planet::earth::geodesic_dist(&paris, &washington); // in meters
  ```

* Convert equatorial coordinates to ecliptic coordinates
  ```rust
	// equatorial coordinates of the star Pollux

    let right_ascension = 116.328942_f64.to_radians();
    let declination = 28.026183_f64.to_radians();

    // mean obliquity of the ecliptic

    let oblq_eclip = 23.4392911_f64.to_radians();

    // you can also get oblq_eclip from ecliptic::mn_oblq_IAU(julian_day)
    // for the Julian day on which the coordinates of the star
    // were observed

    // also make sure to type #[macro_use] before including the crate
    // to use macros

    // now, convert equatorial coordinates to ecliptic coordinates

    let (ecl_long, ecl_lat) = EclFrmEq!(right_ascension, declination, oblq_eclip);
  ```

* Convert equatorial coordinates to galactic coordinates
  ```rust
	// equatorial coordinates of the Nova Serpentis 1978

    let right_ascension = angle::deg_frm_hms(17, 48, 59.74).to_radians();
    let declination = angle::deg_frm_dms(-14, 43, 8.2).to_radians();

    // convert to galactic coordinates

    let (gal_long, gal_lat) = GalFrmEq!(right_ascension, declination);
  ```

## Algorithms

For information on the modules and functions available, see the
[Rust API Documentation](https://saurvs.github.io/astro-rust/).

The following is a categorical high level listing of all the physical
quantities you can calculate or actions you can perform, using the
algorithms that have been implemented so far:

###### The 8 Solar System Planets

* heliocentric coordinates (using the *full* VSOP87-D solution)
* orbital elements referred to the mean equinox of the date

###### The Solar System Planets excluding Earth

* geocentric ecliptic and equatorial coordinates from heliocentric coordinates
* apparent visual magnitude
* equatorial semidiameter
* polar semidiameter for Saturn and Jupiter
* illuminated fraction of the planetary disk
* position angle of the bright limb
* phase angle

###### The Sun

* geocentric ecliptic coordinates
* geocentric rectangular coordinates referred to the mean equinox of the date
* ephemeris for physical observations
* start of different Carrington's synodic rotation cycles

###### The Moon

* times of New Moon, Full Moon, First Quarter and Last Quarter
* geocentric ecliptic coordinates
* optical, physical and topocentric liberations
* times of passage through the nodes
* illuminated fraction of the lunar disk
* equatorial horizontal parallax

###### The Earth

* high accuracy geodesic distance
* eccentricity of the meridian
* angle between the diurnal path and the horizon

###### Pluto

* heliocentric coordinates
* mean orbital elements near 2000 AD
* apparent visual magnitude
* equatorial semidiameter

###### Mars

* ecliptic coordinates of the north pole
* ephemeris for physical observations

###### Jupiter

* apparent rectangular coordinates of Io, Europa, Ganymede and Callisto
with respect to Jupiter as seen from Earth
* ephemeris for physical observations

###### Saturn

* apparent rectangular coordinates of Mimas, Enceladus, Tethys, Dione, Rhea,
Titan, Hyperion and Iapetus with respect to Saturn as seen from Earth
* elements of Saturn's ring system

###### Transit

* Times of rise, transit and set for the stars, planets, Sun, and Moon

###### Ecliptic

* mean obliquity, using the IAU and Laskar formulae
* angle between the ecliptic and the horizon from Earth
* longitudes of the two ecliptic points on the horizon from Earth

###### Nutation in

* ecliptic longitude
* obliquity of the ecliptic
* right ascension and declination

###### Atmospheric refraction

* refraction correction for true altitude to
obtain apparent altitude, and vice-versa
* effect of local pressure and temperature

###### Aberration

* solar aberration in ecliptic longitude
* stellar aberration in equatorial coordinates

###### Transform

* ecliptic coordinates to equatorial coordinates, and vice-versa
* equatorial coordinates to local horizontal coordinates, and vice-versa
* ecliptic coordinates to galactic coordinates, and vice-versa
* heliocentric coordinates to ecliptic/equatorial geocentric coordinates
* orbital elements to heliocentric coordinates
* ecliptic/equatorial coordinates from one epoch to another
  (due to precession)
* orbital elements from one equinox to another

###### Elliptic orbits

* eccentric anomaly, true anomaly and radius vector of a body in orbit
* times of passage through the nodes

###### Parabolic orbits

* true anomaly and radius vector of a body in orbit
* times of passage through the nodes

###### Near-parabolic orbits

* true anomaly and radius vector of a body in orbit

###### Time

* Julian day from Gregorian and Julian dates, and vice-versa
* Julian century and millennium
* analytic approximation for ΔT, with [surprisingly good accuracy](http://eclipse.gsfc.nasa.gov/SEcat5/uncertainty.html) in recent times
* mean and apparent Sidereal time

###### Stars

* combined magnitude of two or more stars
* absolute magnitude
* brightness ratio of two stars from their difference in magnitudes, and vice-versa
* angle between a vector from a star to the Earth's north
  celestial pole and a vector from the same star to the north
  pole of the ecliptic
* equatorial coordinates at a different time due to proper
  motion and radial velocity

###### Binary stars

* angular separation
* apparent position angle
* eccentric of the apparent orbit
* radius vector
* true anomaly
* mean annual motion of the companion star
* mean anomaly of the companion star

###### Asteroids

* true diameter (from absolute magnitude and albedo)
* apparent diameter (from true diameter and distance to the Earth)

## Contributing

Anyone interested to contribute in any way possible is encouraged to do so. Not all the algorithms in Meeus's book have been implemented yet. Documentation and tests need to be written for them as well. Refactored code and minor optimizations for the existing code are also welcome.

A fun suggestion is the addition of the recent [IAU 2000/2006 precession-nutation model](http://62.161.69.131/iers/conv2010/conv2010_c5.html). This method improves upon the existing model implemented here *"by taking into account the effect of mantle anelasticity, ocean tides, electromagnetic couplings produced between the fluid outer core and the mantle as well as between the solid inner core and fluid outer core"*.

## References

1. Most algorithms: [Astronomical Algorithms, 2nd edition (Meeus)](http://www.willbell.com/math/mc1.htm)
2. Planetary heliocentric positioning: [VSOP87-D](http://cdsarc.u-strasbg.fr/viz-bin/qcat?VI/81/)
3. Approximating ΔT: [Five Millennium Canon of Solar Eclipses (Espenak and Meeus)](http://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html)
4. Some physical constants: [World Geodetic System 1984](https://confluence.qps.nl/pages/viewpage.action?pageId=29855173)
