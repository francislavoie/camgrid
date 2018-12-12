use conrod::{
    backend::glium::{
        glium::{
            self,
            glutin::{self, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
            texture, Display, Surface,
        },
        Renderer,
    },
    event::Input,
    image::Map,
    widget, Colorable, Positionable, Widget,
};
use std::path::Path;

pub struct Window {
    events_loop: glutin::EventsLoop,
    display: Display,
    ui: conrod::Ui,
    ids: Ids,
    image_map: Map<texture::Texture2d>,
    renderer: Renderer,
}

widget_ids!(
    struct Ids {
        text
    }
);

impl Window {
    pub fn new<P, S>(width: f64, height: f64, title: S, assets: P) -> Window
    where
        P: AsRef<Path>,
        S: Into<String>,
    {
        let events_loop = glutin::EventsLoop::new();

        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));

        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = Display::new(window_builder, context, &events_loop).unwrap();

        // Construct the UI
        let mut ui = conrod::UiBuilder::new([width, height]).build();

        // Generate the widget IDs
        let ids = Ids::new(ui.widget_id_generator());

        // Describes each of the widget -> image mappings
        let image_map = Map::<texture::Texture2d>::new();

        // A type used to convert Primitives into Command, to draw to Surface
        let renderer = Renderer::new(&display).unwrap();

        // Load the fonts
        let font_path = assets.as_ref().join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        Window {
            events_loop: events_loop,
            display: display,
            ui: ui,
            ids: ids,
            image_map: image_map,
            renderer: renderer,
        }
    }

    pub fn get_events_loop(&mut self) -> &mut glium::glutin::EventsLoop {
        &mut self.events_loop
    }

    pub fn convert_event(&mut self, event: &glutin::Event) -> Option<Input> {
        // Use the `winit` backend feature to convert the winit event to a conrod one.
        let input = conrod::backend::winit::convert_event(event.clone(), &self.display);

        if input.is_some() {
            let event = input.unwrap().clone();
            self.ui.handle_event(event.clone());
            return Some(event);
        }

        input
    }

    pub fn should_quit(&self, event: &Event) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn render(&mut self) {
        // Instantiate all widgets in the GUI.
        {
            let ui = &mut self.ui.set_widgets();

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Hello World!")
                .middle_of(ui.window)
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(self.ids.text, ui);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer
                .fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.renderer
                .draw(&self.display, &mut target, &self.image_map)
                .unwrap();
            target.finish().unwrap();
        }
    }
}
