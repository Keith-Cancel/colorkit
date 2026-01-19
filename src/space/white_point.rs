#[allow(non_camel_case_types)]
/// The `*x*`, *y*` and FOV of a white point.
pub trait WhitePoint_xy: Copy {
    /// Field of View Angle
    const FOV: u8;
    #[allow(non_upper_case_globals)]
    /// Whites points chromaticity *`x`* value.
    const x_i: f32;
    #[allow(non_upper_case_globals)]
    /// White points chromaticity *`y`* value.
    const y_i: f32;
}

/// The reference white point for a color space.
pub trait WhitePoint: WhitePoint_xy {
    /// White point `X`
    const X: f32 = white_point_x::<Self>(1.0);
    /// White point `Y` in range [0, 1.0]
    const Y: f32 = 1.0;
    /// White point `Z`
    const Z: f32 = white_point_z::<Self>(1.0);

    /// Calculate the `X` value using a different `Y`
    /// using the white point [`WhitePoint_xy::x_i`]
    /// and [`WhitePoint_xy::y_i`] values.
    fn calc_x(y: f32) -> f32 {
        return white_point_x::<Self>(y);
    }

    /// Calculate the `Z` value using a different `Y`
    /// using the white point [`WhitePoint_xy::x_i`]
    /// and [`WhitePoint_xy::y_i`] values.
    fn calc_z(y: f32) -> f32 {
        return white_point_z::<Self>(y);
    }
}

/// Calculate the `X` value using a different `Y`
/// using the white point [`WhitePoint_xy::x_i`]
/// and [`WhitePoint_xy::y_i`] values.
const fn white_point_x<W: WhitePoint>(y: f32) -> f32 {
    if W::y_i == 0.0 {
        panic!("White point y_i is zero");
    }
    return (W::x_i / W::y_i) * y;
}

/// Calculate the `Z` value using a different `Y`
/// using the white point [`WhitePoint_xy::x_i`]
/// and [`WhitePoint_xy::y_i`] values.
const fn white_point_z<W: WhitePoint>(y: f32) -> f32 {
    if W::y_i == 0.0 {
        panic!("White point y_i is zero");
    }
    let tmp = (1.0 - W::x_i - W::y_i) / W::y_i;
    return tmp * y;
}

/// D65 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D65;
// Used the data here and my script tools/spectrum_xyz.py to derive these
// so the X, Y, Z and x_i and y_i match up better.
// https://cie.co.at/datatable/cie-1931-colour-matching-functions-2-degree-observer
// https://cie.co.at/datatable/cie-standard-illuminant-d65
// When using:
// const x_i: f32 = 0.31272;
// const y_i: f32 = 0.32903;
// I noticed that X, Y, Z where slightly different the what I see published. I could
// just change the default values for the X, Y, and Z, but also wanted the calc functions
// to be more accurate.
impl WhitePoint_xy for D65 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.3127268710265648;
    const y_i: f32 = 0.3290232066412840;
}
impl WhitePoint for D65 {
    const X: f32 = 0.9504705586542831;
    const Z: f32 = 1.0888287363958840;
}

/// D65 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D65Deg10;
impl WhitePoint_xy for D65Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.3138236469387096;
    const y_i: f32 = 0.3309989854899336;
}
impl WhitePoint for D65Deg10 {
    const X: f32 = 0.9481106006237403;
    const Z: f32 = 1.0730466954321181;
}
