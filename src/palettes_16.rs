use plotters::style::Palette;

pub struct PaletteBentCoolWarm16;
pub struct PaletteBlackBody16;
pub struct PaletteExtendedKindlmann16;
pub struct PaletteInferno16;
pub struct PaletteKindlmann16;
pub struct PalettePlasma16;
pub struct PaletteSmoothCoolWarm16;
pub struct PaletteTurbo16;
pub struct PaletteViridis16;

impl Palette for PaletteBentCoolWarm16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (76, 100, 203),
        (96, 122, 212),
        (118, 145, 219),
        (142, 167, 226),
        (168, 189, 231),
        (196, 210, 236),
        (226, 232, 240),
        (240, 229, 224),
        (234, 204, 188),
        (227, 177, 156),
        (220, 151, 126),
        (211, 123, 99),
        (202, 95, 75),
        (192, 62, 55),
        (180, 4, 38),
    ];
}

impl Palette for PaletteBlackBody16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (36, 15, 9),
        (62, 22, 17),
        (90, 27, 22),
        (119, 30, 26),
        (150, 33, 30),
        (180, 38, 34),
        (197, 65, 28),
        (214, 88, 19),
        (228, 112, 7),
        (231, 141, 18),
        (233, 169, 29),
        (233, 195, 39),
        (231, 222, 50),
        (246, 240, 144),
        (255, 255, 255),
    ];
}

impl Palette for PaletteExtendedKindlmann16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (29, 3, 65),
        (8, 6, 119),
        (4, 49, 87),
        (3, 72, 61),
        (4, 92, 26),
        (22, 110, 5),
        (88, 121, 6),
        (165, 120, 8),
        (246, 98, 58),
        (249, 125, 135),
        (250, 144, 225),
        (225, 182, 251),
        (227, 209, 253),
        (233, 234, 254),
        (255, 255, 255),
    ];
}

impl Palette for PaletteInferno16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 4),
        (12, 8, 38),
        (36, 12, 79),
        (66, 10, 104),
        (93, 18, 110),
        (120, 28, 109),
        (147, 38, 103),
        (174, 48, 92),
        (199, 62, 76),
        (221, 81, 58),
        (237, 105, 37),
        (248, 133, 15),
        (252, 165, 10),
        (250, 198, 45),
        (242, 230, 97),
        (252, 255, 164),
    ];
}

impl Palette for PaletteKindlmann16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (37, 3, 57),
        (37, 5, 109),
        (24, 8, 163),
        (8, 51, 160),
        (6, 83, 127),
        (5, 105, 105),
        (6, 127, 83),
        (7, 148, 47),
        (15, 168, 8),
        (63, 186, 9),
        (133, 199, 10),
        (205, 205, 10),
        (251, 210, 163),
        (253, 232, 223),
        (255, 255, 255),
    ];
}

impl Palette for PalettePlasma16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (13, 8, 135),
        (51, 5, 151),
        (80, 2, 162),
        (106, 0, 168),
        (132, 5, 167),
        (156, 23, 158),
        (177, 42, 144),
        (195, 61, 128),
        (211, 81, 113),
        (225, 100, 98),
        (237, 121, 83),
        (246, 143, 68),
        (252, 166, 54),
        (254, 192, 41),
        (249, 220, 36),
        (240, 249, 33),
    ];
}

impl Palette for PaletteSmoothCoolWarm16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (79, 106, 217),
        (101, 133, 236),
        (123, 159, 249),
        (147, 181, 255),
        (170, 199, 253),
        (192, 212, 245),
        (212, 219, 230),
        (229, 216, 209),
        (242, 203, 183),
        (247, 184, 156),
        (246, 160, 129),
        (238, 133, 104),
        (224, 101, 79),
        (204, 64, 57),
        (180, 4, 38),
    ];
}

impl Palette for PaletteTurbo16 {
    // calculated from https://gist.github.com/mikhailov-work/ee72ba4191942acecc03fe6da94fc73f
    const COLORS: &'static [(u8, u8, u8)] = &[
        (48, 18, 59),
        (64, 67, 167),
        (70, 113, 233),
        (62, 155, 255),
        (33, 197, 226),
        (26, 229, 183),
        (70, 248, 132),
        (136, 255, 78),
        (185, 246, 52),
        (226, 221, 55),
        (250, 186, 56),
        (254, 141, 39),
        (240, 91, 17),
        (214, 52, 5),
        (175, 24, 1),
        (122, 4, 2),
    ];
}

impl Palette for PaletteViridis16 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (68, 1, 84),
        (72, 26, 108),
        (71, 47, 125),
        (65, 68, 135),
        (57, 86, 140),
        (49, 104, 142),
        (42, 120, 142),
        (35, 136, 142),
        (31, 152, 139),
        (34, 168, 132),
        (53, 183, 121),
        (84, 197, 104),
        (122, 209, 81),
        (165, 219, 54),
        (210, 226, 27),
        (253, 231, 37),
    ];
}
