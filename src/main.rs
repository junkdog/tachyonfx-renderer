use ratatui::{Frame, Terminal};
use ratzilla::backend::webgl2::WebGl2BackendOptions;
use ratzilla::{WebGl2Backend, WebRenderer};
use eyre::Result;
use crate::event_handler::EventHandler;

mod app;
mod dispatcher;
mod effects;
mod event;
mod event_handler;
mod theme;

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let events = EventHandler::new(core::time::Duration::from_millis(33));

    let options = WebGl2BackendOptions::new()
            .enable_console_debug_api()
            .enable_mouse_selection()
            .grid_id("container")
            .measure_performance(true);

    let backend = WebGl2Backend::new_with_options(options)
        .map_err(|e| eyre::eyre!("{:?}", e))?;
    let terminal = Terminal::new(backend)
        .map_err(|e| eyre::eyre!("{:?}", e))?;


    let mut app = app::App::new(events.sender());

    terminal.draw_web(move |frame| {
        while let Some(event) = events.try_next() {
            app.apply_event(event);
        }

        let elapsed = app.tick();
        let area = frame.area();

        render_ui(frame);
        app.process_effects(elapsed, frame.buffer_mut(), area);

    });

    Ok(())
}

fn render_ui(
    f: &mut Frame<'_>,
) {
    let screen = f.area().as_size();
}