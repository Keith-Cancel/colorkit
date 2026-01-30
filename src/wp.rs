//! Standard White Points (such as [`D65`]) and the [`WhitePoint`] trait.
use colorkit::colors::Xyz;

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

    ///// Get the XYZ color of the white point
    fn color() -> Xyz<Self> {
        return white_point_color();
    }
}

/// Calculate the `X` value using a different `Y`
/// using the white point [`WhitePoint_xy::x_i`]
/// and [`WhitePoint_xy::y_i`] values.
pub const fn white_point_x<W: WhitePoint>(y: f32) -> f32 {
    if W::y_i == 0.0 {
        panic!("White point y_i is zero");
    }
    return (W::x_i / W::y_i) * y;
}

/// Calculate the `Z` value using a different `Y`
/// using the white point [`WhitePoint_xy::x_i`]
/// and [`WhitePoint_xy::y_i`] values.
pub const fn white_point_z<W: WhitePoint>(y: f32) -> f32 {
    if W::y_i == 0.0 {
        panic!("White point y_i is zero");
    }
    let tmp = (1.0 - W::x_i - W::y_i) / W::y_i;
    return tmp * y;
}

/// Get the XYZ color of the white point
pub const fn white_point_color<W: WhitePoint>() -> Xyz<W> {
    return Xyz::new(W::X, W::Y, W::Z);
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
    const X: f32 = 0.9504705586542830;
    const Z: f32 = 1.0888287363958847;
}

/// D65 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D65Deg10;
impl WhitePoint_xy for D65Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.3138236469387095;
    const y_i: f32 = 0.3309989854899336;
}
impl WhitePoint for D65Deg10 {
    const X: f32 = 0.9481106006237398;
    const Z: f32 = 1.0730466954321179;
}

/// D50 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D50;
impl WhitePoint_xy for D50 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.3456842223226252;
    const y_i: f32 = 0.3585040259042519;
}
impl WhitePoint for D50 {
    const X: f32 = 0.9642408378837830;
    const Z: f32 = 0.8251281168377378;
}

/// D50 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D50Deg10;
impl WhitePoint_xy for D50Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.3477476825870567;
    const y_i: f32 = 0.3595360243755529;
}
impl WhitePoint for D50Deg10 {
    const X: f32 = 0.9672123487236909;
    const Z: f32 = 0.8141501078947072;
}

/// D55 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D55;
impl WhitePoint_xy for D55 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.3324241024688304;
    const y_i: f32 = 0.3474280390876662;
}
impl WhitePoint for D55 {
    const X: f32 = 0.9568142609956419;
    const Z: f32 = 0.9214796229003290;
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
    const X: f32 = 0.9579954733996483;
    const Z: f32 = 0.9092557470435315;
}

/// D75 White point 2 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D75;
impl WhitePoint_xy for D75 {
    const FOV: u8 = 2;
    const x_i: f32 = 0.2990223004124971;
    const y_i: f32 = 0.3148527378883418;
}
impl WhitePoint for D75 {
    const X: f32 = 0.9497211376276525;
    const Z: f32 = 1.2263668541961194;
}

/// D75 White point 10 Degree FOV
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D75Deg10;
impl WhitePoint_xy for D75Deg10 {
    const FOV: u8 = 10;
    const x_i: f32 = 0.2996799713457530;
    const y_i: f32 = 0.3174032398548366;
}
impl WhitePoint for D75Deg10 {
    const X: f32 = 0.9441616647732098;
    const Z: f32 = 1.2064047896125327;
}
