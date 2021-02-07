use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let scale_point = scale.value();

        oklch(
            lerp(scale_point, 0.22..0.9),
            lerp(scale_point, 0.015..0.03),
            280.0,
        )
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.75, 0.17, 355.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkerBg,
    Bg,
    FadedFg,
    Fg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::DarkerBg => 0.0,
            Self::Bg => 0.07,
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
