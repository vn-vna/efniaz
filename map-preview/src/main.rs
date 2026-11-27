fn main() -> Result<(), Box<dyn std::error::Error>> {
    map_preview::save_roads()?;
    Ok(())
}
