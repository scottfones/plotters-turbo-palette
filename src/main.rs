use plotters::prelude::*;

mod palettes_1024;
mod palettes_128;
mod palettes_16;
mod palettes_256;
mod palettes_32;
mod palettes_512;
mod palettes_64;
mod palettes_8;

enum CustomPalette {
    BentCoolWarm,
    BlackBody,
    ExtendedKindlmann,
    Inferno,
    Kindlmann,
    Plasma,
    SmoothCoolWarm,
    Turbo,
    Viridis,
}
use CustomPalette::*;

fn set_filename(p_name: &CustomPalette) -> String {
    let sample_name = match p_name {
        BentCoolWarm => "bent-cool-warm",
        BlackBody => "black-body",
        ExtendedKindlmann => "extended-kindlmann",
        Inferno => "inferno",
        Kindlmann => "kindlmann",
        Plasma => "plasma",
        SmoothCoolWarm => "smooth-cool-warm",
        Turbo => "turbo",
        Viridis => "viridis",
    };
    format!("heatmap-{}.png", sample_name)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HEIGHT: usize = 300;
    const WIDTH: usize = 1024;

    let palettes = vec![
        BentCoolWarm,
        BlackBody,
        ExtendedKindlmann,
        Inferno,
        Kindlmann,
        Plasma,
        SmoothCoolWarm,
        Turbo,
        Viridis,
    ];

    for name in palettes {
        let filename = set_filename(&name);
        let root_drawing_area =
            BitMapBackend::new(&filename, (WIDTH as u32, HEIGHT as u32)).into_drawing_area();
        let child_drawing_areas = root_drawing_area.split_evenly((6, WIDTH));

        for idx in 0..WIDTH {
            match name {
                BentCoolWarm => {
                    child_drawing_areas[idx]
                        .fill(&palettes_1024::PaletteBentCoolWarm1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteBentCoolWarm128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteBentCoolWarm64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteBentCoolWarm32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteBentCoolWarm16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteBentCoolWarm8::pick(idx / 128))?;
                }

                BlackBody => {
                    child_drawing_areas[idx]
                        .fill(&palettes_1024::PaletteBlackBody1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteBlackBody128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteBlackBody64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteBlackBody32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteBlackBody16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteBlackBody8::pick(idx / 128))?;
                }

                ExtendedKindlmann => {
                    child_drawing_areas[idx]
                        .fill(&palettes_1024::PaletteExtendedKindlmann1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteExtendedKindlmann128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteExtendedKindlmann64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteExtendedKindlmann32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteExtendedKindlmann16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteExtendedKindlmann8::pick(idx / 128))?;
                }

                Inferno => {
                    child_drawing_areas[idx].fill(&palettes_1024::PaletteInferno1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteInferno128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteInferno64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteInferno32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteInferno16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteInferno8::pick(idx / 128))?;
                }

                Kindlmann => {
                    child_drawing_areas[idx]
                        .fill(&palettes_1024::PaletteKindlmann1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteKindlmann128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteKindlmann64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteKindlmann32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteKindlmann16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteKindlmann8::pick(idx / 128))?;
                }

                Plasma => {
                    child_drawing_areas[idx].fill(&palettes_1024::PalettePlasma1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PalettePlasma128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PalettePlasma64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PalettePlasma32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PalettePlasma16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PalettePlasma8::pick(idx / 128))?;
                }

                SmoothCoolWarm => {
                    child_drawing_areas[idx]
                        .fill(&palettes_1024::PaletteSmoothCoolWarm1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteSmoothCoolWarm128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteSmoothCoolWarm64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteSmoothCoolWarm32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteSmoothCoolWarm16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteSmoothCoolWarm8::pick(idx / 128))?;
                }

                Turbo => {
                    child_drawing_areas[idx].fill(&palettes_1024::PaletteTurbo1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteTurbo128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteTurbo64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteTurbo32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteTurbo16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteTurbo8::pick(idx / 128))?;
                }

                Viridis => {
                    child_drawing_areas[idx].fill(&palettes_1024::PaletteViridis1024::pick(idx))?;
                    child_drawing_areas[idx + WIDTH]
                        .fill(&palettes_128::PaletteViridis128::pick(idx / 8))?;
                    child_drawing_areas[idx + 2 * WIDTH]
                        .fill(&palettes_64::PaletteViridis64::pick(idx / 16))?;
                    child_drawing_areas[idx + 3 * WIDTH]
                        .fill(&palettes_32::PaletteViridis32::pick(idx / 32))?;
                    child_drawing_areas[idx + 4 * WIDTH]
                        .fill(&palettes_16::PaletteViridis16::pick(idx / 64))?;
                    child_drawing_areas[idx + 5 * WIDTH]
                        .fill(&palettes_8::PaletteViridis8::pick(idx / 128))?;
                }
            };
        }
        root_drawing_area.present()?;
    }
    Ok(())
}
