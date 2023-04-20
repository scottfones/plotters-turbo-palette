use plotters::prelude::*;

mod custom_palettes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HEIGHT: u32 = 100;
    const WIDTH: f64 = 800.0;

    let palettes = vec![
        "bent-cool-warm",
        "black-body",
        "extended-kindlmann",
        "inferno",
        "kindlmann",
        "plasma",
        "smooth-cool-warm",
        "turbo",
        "viridis",
    ];

    for name in palettes {
        let filename = format!("heatmap-{}.png", &name);
        let root_drawing_area =
            BitMapBackend::new(&filename, (WIDTH as u32, HEIGHT)).into_drawing_area();
        let child_drawing_areas = root_drawing_area.split_evenly((1, WIDTH as usize));

        for (idx, area) in child_drawing_areas.iter().enumerate() {
            let color_code = ((idx as f64 / WIDTH) * 1024.0) as usize;
            match name {
                "bent-cool-warm" => {
                    area.fill(&custom_palettes::PaletteBentCoolWarm1024::pick(color_code))?
                }

                "black-body" => {
                    area.fill(&custom_palettes::PaletteBlackBody1024::pick(color_code))?
                }

                "extended-kindlmann" => area.fill(
                    &custom_palettes::PaletteExtendedKindlmann1024::pick(color_code),
                )?,

                "inferno" => area.fill(&custom_palettes::PaletteInferno1024::pick(color_code))?,

                "kindlmann" => {
                    area.fill(&custom_palettes::PaletteKindlmann1024::pick(color_code))?
                }

                "plasma" => area.fill(&custom_palettes::PalettePlasma1024::pick(color_code))?,

                "smooth-cool-warm" => area.fill(
                    &custom_palettes::PaletteSmoothCoolWarm1024::pick(color_code),
                )?,

                "turbo" => area.fill(&custom_palettes::PaletteTurbo256::pick(color_code / 4))?,

                "viridis" => area.fill(&custom_palettes::PaletteViridis1024::pick(color_code))?,

                _ => unreachable!("unknown palette name: {name}"),
            };
        }
        root_drawing_area.present()?;
    }
    Ok(())
}
