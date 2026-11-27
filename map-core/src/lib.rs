pub fn render_debug_tile() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut image = tiny_skia::Pixmap::new(256, 256).ok_or("Cannot create image")?;
    image.fill(tiny_skia::Color::BLACK);
    let road = create_road(&[(32.0, 32.0), (32.0, 128.0), (180.0, 128.0)])?;
    let mut paint = tiny_skia::Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    let mut stroke = tiny_skia::Stroke::default();
    stroke.width = 6.0;

    image.stroke_path(
        &road,
        &paint,
        &stroke,
        tiny_skia::Transform::identity(),
        None,
    );

    Ok(image.encode_png()?)
}

pub fn render_roads_tile(bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let reader = mvt_reader::Reader::new(bytes)?;
    let roads = reader
        .get_layer_metadata()?
        .into_iter()
        .find(|l| l.name == "roads")
        .ok_or("Cannot find any roads")?;

    let scale = 4096.0 / roads.extent as f32;
    let mut image = tiny_skia::Pixmap::new(4096, 4096).ok_or("Cannot create image")?;
    image.fill(tiny_skia::Color::from_rgba8(0, 0, 0, 0));
    let mut paint = tiny_skia::Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;
    let mut stroke = tiny_skia::Stroke::default();
    stroke.width = 2.0;

    for feature in reader.get_features(roads.layer_index)? {
        match feature.geometry {
            geo_types::Geometry::LineString(line) => {
                draw_line(&mut image, &paint, &stroke, scale, &line)?
            }
            geo_types::Geometry::MultiLineString(lines) => {
                for line in &lines.0 {
                    draw_line(&mut image, &paint, &stroke, scale, &line)?;
                }
            }
            _ => {}
        }
    }

    Ok(image.encode_png()?)
}

pub fn draw_line(
    image: &mut tiny_skia::Pixmap,
    paint: &tiny_skia::Paint,
    stroke: &tiny_skia::Stroke,
    scale: f32,
    line: &geo_types::LineString<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let points: Vec<(f32, f32)> = line
        .points()
        .into_iter()
        .map(|p| (p.x() * scale, p.y() * scale))
        .collect();
    let path = create_road(&points)?;
    image.stroke_path(&path, paint, stroke, tiny_skia::Transform::identity(), None);
    Ok(())
}

pub fn create_road(path: &[(f32, f32)]) -> Result<tiny_skia::Path, Box<dyn std::error::Error>> {
    if path.len() < 2 {
        return Err("Too few points".into());
    }

    let mut path_builder = tiny_skia::PathBuilder::new();
    let (fx, fy) = path[0];
    path_builder.move_to(fx, fy);
    for &(x, y) in &path[1..] {
        path_builder.line_to(x, y);
    }
    Ok(path_builder.finish().ok_or("Empty road path")?)
}

pub fn tile_layers(
    bytes: Vec<u8>,
) -> Result<Vec<mvt_reader::layer::Layer>, Box<dyn std::error::Error>> {
    let reader = mvt_reader::Reader::new(bytes)?;
    Ok(reader.get_layer_metadata()?)
}
