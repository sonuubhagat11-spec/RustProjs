use winresource::WindowsResource;

fn main() {
    // Only run this script if compiling on/for Windows
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut haahaaland = WindowsResource::new();
        haahaaland.set_icon("finder.ico");
        haahaaland.compile().unwrap();
    }
}
