use plotters::prelude::*;

mod custom_palettes;

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
                    area.fill(&custom_palettes::PaletteBentCoolWarm1024::pick(color_code))?
                }

                BlackBody => area.fill(&custom_palettes::PaletteBlackBody1024::pick(color_code))?,

                ExtendedKindlmann => area.fill(
                    &custom_palettes::PaletteExtendedKindlmann1024::pick(color_code),
                )?,

                Inferno => area.fill(&custom_palettes::PaletteInferno1024::pick(color_code))?,

                Kindlmann => area.fill(&custom_palettes::PaletteKindlmann1024::pick(color_code))?,

                Plasma => area.fill(&custom_palettes::PalettePlasma1024::pick(color_code))?,

                SmoothCoolWarm => area.fill(&custom_palettes::PaletteSmoothCoolWarm1024::pick(
                    color_code,
                ))?,

                Turbo => area.fill(&custom_palettes::PaletteTurbo256::pick(color_code / 4))?,

                Viridis => area.fill(&custom_palettes::PaletteViridis1024::pick(color_code))?,
            };
        }
        root_drawing_area.present()?;
    }
    Ok(())
}
