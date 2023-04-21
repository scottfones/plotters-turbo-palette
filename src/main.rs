use plotters::prelude::*;

mod palettes_1024;
mod palettes_256;

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
    const HEIGHT: u32 = 100;
    const WIDTH: f64 = 800.0;

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
            BitMapBackend::new(&filename, (WIDTH as u32, HEIGHT)).into_drawing_area();
        let child_drawing_areas = root_drawing_area.split_evenly((1, WIDTH as usize));

        for (idx, area) in child_drawing_areas.iter().enumerate() {
            let color_code = ((idx as f64 / WIDTH) * 1024.0) as usize;
            match name {
                BentCoolWarm => {
                    area.fill(&palettes_1024::PaletteBentCoolWarm1024::pick(color_code))?
                }

                BlackBody => area.fill(&palettes_1024::PaletteBlackBody1024::pick(color_code))?,

                ExtendedKindlmann => area.fill(
                    &palettes_1024::PaletteExtendedKindlmann1024::pick(color_code),
                )?,

                Inferno => area.fill(&palettes_1024::PaletteInferno1024::pick(color_code))?,

                Kindlmann => area.fill(&palettes_1024::PaletteKindlmann1024::pick(color_code))?,

                Plasma => area.fill(&palettes_1024::PalettePlasma1024::pick(color_code))?,

                SmoothCoolWarm => {
                    area.fill(&palettes_1024::PaletteSmoothCoolWarm1024::pick(color_code))?
                }

                Turbo => area.fill(&palettes_256::PaletteTurbo256::pick(color_code / 4))?,

                Viridis => area.fill(&palettes_1024::PaletteViridis1024::pick(color_code))?,
            };
        }
        root_drawing_area.present()?;
    }
    Ok(())
}
