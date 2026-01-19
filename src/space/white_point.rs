#[allow(non_camel_case_types)]
/// The *`x`*, *`y`* chromaticity and FOV of a white point.
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

/// D50 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D50;
impl WhitePoint_xy for D50 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.3456842223226256;
    const y_i: f32 = 0.3585040259042516;
}
impl WhitePoint for D50 {
    const X: f32 = 0.9642408378837846;
    const Z: f32 = 0.8251281168377381;
}

/// D50 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D50Deg10;
impl WhitePoint_xy for D50Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.3477476825870566;
    const y_i: f32 = 0.3595360243755528;
}
impl WhitePoint for D50Deg10 {
    const X: f32 = 0.9672123487236909;
    const Z: f32 = 0.8141501078947077;
}

/// D55 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D55;
impl WhitePoint_xy for D55 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.3324241024688303;
    const y_i: f32 = 0.3474280390876662;
}
impl WhitePoint for D55 {
    const X: f32 = 0.9568142609956416;
    const Z: f32 = 0.9214796229003291;
}

/// D55 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D55Deg10;
impl WhitePoint_xy for D55Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.3341163364302535;
    const y_i: f32 = 0.3487660909759536;
}
impl WhitePoint for D55Deg10 {
    const X: f32 = 0.9579954733996484;
    const Z: f32 = 0.9092557470435318;
}

/// D75 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D75;
impl WhitePoint_xy for D75 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.2990223004124971;
    const y_i: f32 = 0.3148527378883419;
}
impl WhitePoint for D75 {
    const X: f32 = 0.9497211376276523;
    const Z: f32 = 1.2263668541961188;
}

/// D75 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D75Deg10;
impl WhitePoint_xy for D75Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.2996799713457529;
    const y_i: f32 = 0.3174032398548367;
}
impl WhitePoint for D75Deg10 {
    const X: f32 = 0.9441616647732094;
    const Z: f32 = 1.2064047896125327;
}
