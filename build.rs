use {
    std::io,
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("icon.ico")
            .compile()?;
    }
    Ok(())
}