use app::App;

pub mod app;
pub mod terrain;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 768;

fn main() {
    App::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Heightmap").run();
}
