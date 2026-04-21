#![cfg_attr(
    all(feature = "desktop", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[cfg(feature = "desktop")]
fn main() {
    app_lib::run();
}

#[cfg(not(feature = "desktop"))]
fn main() {
    eprintln!("The desktop binary requires the `desktop` feature.");
    std::process::exit(1);
}
