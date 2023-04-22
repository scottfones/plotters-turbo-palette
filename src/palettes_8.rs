use plotters::style::Palette;

pub struct PaletteBentCoolWarm8;
pub struct PaletteBlackBody8;
pub struct PaletteExtendedKindlmann8;
pub struct PaletteInferno8;
pub struct PaletteKindlmann8;
pub struct PalettePlasma8;
pub struct PaletteSmoothCoolWarm8;
pub struct PaletteTurbo8;
pub struct PaletteViridis8;

impl Palette for PaletteBentCoolWarm8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (99, 125, 213),
        (149, 173, 227),
        (209, 220, 238),
        (236, 215, 203),
        (222, 158, 134),
        (203, 99, 79),
        (180, 4, 38),
    ];
}

impl Palette for PaletteBlackBody8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (65, 23, 18),
        (128, 31, 27),
        (188, 51, 32),
        (224, 101, 10),
        (232, 161, 26),
        (231, 218, 48),
        (255, 255, 255),
    ];
}

impl Palette for PaletteExtendedKindlmann8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (6, 13, 120),
        (4, 78, 53),
        (48, 116, 6),
        (227, 99, 11),
        (249, 139, 202),
        (225, 206, 253),
        (255, 255, 255),
    ];
}

impl Palette for PaletteInferno8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 4),
        (40, 11, 84),
        (101, 21, 110),
        (159, 42, 99),
        (212, 72, 66),
        (245, 125, 21),
        (250, 193, 39),
        (252, 255, 164),
    ];
}

impl Palette for PaletteKindlmann8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (36, 6, 117),
        (7, 62, 150),
        (5, 115, 97),
        (8, 159, 21),
        (112, 196, 9),
        (250, 208, 146),
        (255, 255, 255),
    ];
}

impl Palette for PalettePlasma8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (13, 8, 135),
        (84, 2, 163),
        (139, 10, 165),
        (185, 50, 137),
        (219, 92, 104),
        (244, 136, 73),
        (254, 188, 43),
        (240, 249, 33),
    ];
}

impl Palette for PaletteSmoothCoolWarm8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (104, 137, 238),
        (154, 186, 255),
        (201, 216, 240),
        (237, 209, 194),
        (247, 168, 137),
        (226, 106, 83),
        (180, 4, 38),
    ];
}

impl Palette for PaletteTurbo8 {
    // calculated from https://gist.github.com/mikhailov-work/ee72ba4191942acecc03fe6da94fc73f
    const COLORS: &'static [(u8, u8, u8)] = &[
        (48, 18, 59),
        (70, 119, 239),
        (27, 208, 214),
        (98, 253, 107),
        (210, 234, 52),
        (255, 155, 45),
        (219, 57, 7),
        (122, 4, 2),
    ];
}

impl Palette for PaletteViridis8 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (68, 1, 84),
        (70, 50, 127),
        (54, 92, 141),
        (39, 127, 142),
        (31, 161, 135),
        (74, 194, 109),
        (159, 218, 58),
        (253, 231, 37),
    ];
}
