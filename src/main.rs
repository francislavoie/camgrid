#[macro_use]
extern crate conrod;
#[macro_use]
extern crate serde_derive;
extern crate find_folder;
extern crate notify;

mod config;
mod eventloop;
mod watcher;
mod window;

const CONFIG_PATH: &str = "Camgrid.toml";
const WIDTH: f64 = 1200.0;
const HEIGHT: f64 = 800.0;

fn main() {
    // Load our config from file if possible, else init new config
    let config = config::Config::load(CONFIG_PATH);

    // Set up assets path
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();

    // config.add_path("C:\\Some\\Path");

    // Load our filesystem watcher
    let mut watcher = watcher::Watch::new();

    // Watch the configured paths. All files and directories
    // at that path and below will be monitored for changes.
    for path in config.paths() {
        watcher.watch(path);
    }

    // Build the window
    let mut window = window::Window::new(WIDTH, HEIGHT, "Hello Windows!", &assets);

    // Set up and run event loop
    let mut event_loop = eventloop::EventLoop::new();
    'main: loop {
        // Handle FS events
        watcher.recv();

        let mut events = Vec::new();
        window
            .get_events_loop()
            .poll_events(|event| events.push(event));

        for event in event_loop.next(&mut window.get_events_loop()) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if window.convert_event(event.clone()) {
                event_loop.needs_update();
            }

            if window.should_quit(&event) {
                break 'main;
            }
        }

        // Instantiate and render all widgets in the GUI
        window.render();
    }

    config.save(CONFIG_PATH);
}
