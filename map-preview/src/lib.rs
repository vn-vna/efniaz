pub fn save_preview() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write("target/preview.png", map_core::render_debug_tile()?)?;
    Ok(())
}

pub fn save_roads() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os().nth(1).ok_or("pass a .pmtiles file")?;
    let archive = pmtiles::PmTilesReader::new_with_path(path)?;

    // Zoom, column, row. This tile covers part of central Hanoi.
    let coord = pmtiles::TileCoord::new(10, 813, 450)?;
    let bytes = archive
        .get_tile_decompressed(coord)?
        .ok_or("tile not found in archive")?;

    let data = map_core::render_roads_tile(bytes.to_vec())?;
    std::fs::write("target/roads.png", data)?;

    Ok(())
}
