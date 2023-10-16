// shamelessly stealing from bevy goes brr
pub trait SrgbColorSpace {
    fn linear_to_nonlinear_srgb(self) -> Self;
    fn nonlinear_to_linear_srgb(self) -> Self;
}

impl SrgbColorSpace for f32 {
    #[inline]
    fn linear_to_nonlinear_srgb(self) -> f32 {
        if self <= 0.0 {
            return self;
        }

        if self <= 0.0031308 {
            self * 12.92 // linear falloff in dark values
        } else {
            (1.055 * self.powf(1.0 / 2.4)) - 0.055 // gamma curve in other area
        }
    }
    #[inline]
    fn nonlinear_to_linear_srgb(self) -> f32 {
        if self <= 0.0 {
            return self;
        }
        if self <= 0.04045 {
            self / 12.92 // linear falloff in dark values
        } else {
            ((self + 0.055) / 1.055).powf(2.4) // gamma curve in other area
        }
    }
}
pub struct LchRepresentation;
impl LchRepresentation {
    // References available at http://brucelindbloom.com/ in the "Math" section

    // CIE Constants
    // http://brucelindbloom.com/index.html?LContinuity.html (16) (17)
    const CIE_EPSILON: f32 = 216.0 / 24389.0;
    const CIE_KAPPA: f32 = 24389.0 / 27.0;
    // D65 White Reference:
    // https://en.wikipedia.org/wiki/Illuminant_D65#Definition
    const D65_WHITE_X: f32 = 0.95047;
    const D65_WHITE_Y: f32 = 1.0;
    const D65_WHITE_Z: f32 = 1.08883;

    /// converts a color in LCH space to sRGB space
    #[inline]
    pub fn lch_to_nonlinear_srgb(lightness: f32, chroma: f32, hue: f32) -> [f32; 3] {
        let lightness = lightness * 100.0;
        let chroma = chroma * 100.0;

        // convert LCH to Lab
        // http://www.brucelindbloom.com/index.html?Eqn_LCH_to_Lab.html
        let l = lightness;
        let a = chroma * hue.to_radians().cos();
        let b = chroma * hue.to_radians().sin();

        // convert Lab to XYZ
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_XYZ.html
        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;
        let xr = {
            let fx3 = fx.powf(3.0);

            if fx3 > Self::CIE_EPSILON {
                fx3
            } else {
                (116.0 * fx - 16.0) / Self::CIE_KAPPA
            }
        };
        let yr = if l > Self::CIE_EPSILON * Self::CIE_KAPPA {
            ((l + 16.0) / 116.0).powf(3.0)
        } else {
            l / Self::CIE_KAPPA
        };
        let zr = {
            let fz3 = fz.powf(3.0);

            if fz3 > Self::CIE_EPSILON {
                fz3
            } else {
                (116.0 * fz - 16.0) / Self::CIE_KAPPA
            }
        };
        let x = xr * Self::D65_WHITE_X;
        let y = yr * Self::D65_WHITE_Y;
        let z = zr * Self::D65_WHITE_Z;

        // XYZ to sRGB
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html (sRGB, XYZ to RGB [M]-1)
        let red = x * 3.2404542 + y * -1.5371385 + z * -0.4985314;
        let green = x * -0.969266 + y * 1.8760108 + z * 0.041556;
        let blue = x * 0.0556434 + y * -0.2040259 + z * 1.0572252;

        [
            red.linear_to_nonlinear_srgb().clamp(0.0, 1.0),
            green.linear_to_nonlinear_srgb().clamp(0.0, 1.0),
            blue.linear_to_nonlinear_srgb().clamp(0.0, 1.0),
        ]
    }

    /// converts a color in sRGB space to LCH space
    #[inline]
    pub fn nonlinear_srgb_to_lch([red, green, blue]: [f32; 3]) -> (f32, f32, f32) {
        // RGB to XYZ
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
        let red = red.nonlinear_to_linear_srgb();
        let green = green.nonlinear_to_linear_srgb();
        let blue = blue.nonlinear_to_linear_srgb();

        // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html (sRGB, RGB to XYZ [M])
        let x = red * 0.4124564 + green * 0.3575761 + blue * 0.1804375;
        let y = red * 0.2126729 + green * 0.7151522 + blue * 0.072175;
        let z = red * 0.0193339 + green * 0.119192 + blue * 0.9503041;

        // XYZ to Lab
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Lab.html
        let xr = x / Self::D65_WHITE_X;
        let yr = y / Self::D65_WHITE_Y;
        let zr = z / Self::D65_WHITE_Z;
        let fx = if xr > Self::CIE_EPSILON {
            xr.cbrt()
        } else {
            (Self::CIE_KAPPA * xr + 16.0) / 116.0
        };
        let fy = if yr > Self::CIE_EPSILON {
            yr.cbrt()
        } else {
            (Self::CIE_KAPPA * yr + 16.0) / 116.0
        };
        let fz = if yr > Self::CIE_EPSILON {
            zr.cbrt()
        } else {
            (Self::CIE_KAPPA * zr + 16.0) / 116.0
        };
        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        // Lab to LCH
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html
        let c = (a.powf(2.0) + b.powf(2.0)).sqrt();
        let h = {
            let h = b.to_radians().atan2(a.to_radians()).to_degrees();

            if h < 0.0 {
                h + 360.0
            } else {
                h
            }
        };

        ((l / 100.0).clamp(0.0, 1.5), (c / 100.0).clamp(0.0, 1.5), h)
    }
}
