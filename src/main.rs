use app::App;

pub mod app;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    App::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Heightmap").run();
}
