use std::ops::Range;
use tincture::{ColorSpace, Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) const TRANSPARENT: (Oklch, u8) = (Oklch::BLACK, 0);

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 275.0)
    }

    pub(crate) fn ui_blue(&self) -> Oklch {
        oklch(0.53700125, 0.15737669, 254.78726)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.75, 0.15, 30.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.8, 0.1, 60.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.8, 0.1, 100.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.75, 0.12, 135.0)
    }

    pub(crate) fn light_green(&self) -> Oklch {
        oklch(0.85, 0.1, 135.0)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.75, 0.08, 180.0)
    }

    pub(crate) fn light_teal(&self) -> Oklch {
        oklch(0.9, 0.1, 180.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.75, 0.12, 220.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(0.85, 0.1, 220.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.7, 0.16, 305.0)
    }

    pub(crate) fn light_purple(&self) -> Oklch {
        oklch(0.8, 0.12, 305.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.75, 0.17, 355.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkestBg,
    DarkerBg,
    DarkBg,
    Bg,
    LightBg,
    BrightBg,
    DarkFg,
    FadedFg,
    Fg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.1..0.9)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::DarkestBg => 0.01,
            Self::DarkerBg => 0.01,
            Self::DarkBg => 0.01,
            Self::Bg => 0.016,
            Self::LightBg => 0.022,
            Self::BrightBg => 0.05,
            Self::DarkFg => 0.03,
            Self::FadedFg => 0.05,
            Self::Fg => 0.01,
        }
    }

    fn value(self) -> f32 {
        match self {
            Self::DarkestBg => 0.0,
            Self::DarkerBg => 0.15,
            Self::DarkBg => 0.19,
            Self::Bg => 0.22,
            Self::LightBg => 0.235,
            Self::BrightBg => 0.45,
            Self::DarkFg => 0.5,
            Self::FadedFg => 0.7,
            Self::Fg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
