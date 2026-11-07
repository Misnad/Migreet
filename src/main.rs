mod login;
mod view;
use iced::{Theme, window};

fn main() -> iced::Result {
    let _ = iced::application(
        view::MigreetApp::default,
        view::MigreetApp::update,
        view::MigreetApp::view,
    )
    .title("Maigreet")
    .theme(Theme::Dark)
    .window(window::Settings {
        decorations: false,
        fullscreen: true,
        ..Default::default()
    })
    .run();

    Ok(())
}
