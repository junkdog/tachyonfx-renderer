use app::App;
use eyre::Result;
use ratatui::Terminal;
use ratzilla::{WebGl2Backend, WebRenderer, backend::webgl2::WebGl2BackendOptions};

use crate::{event_handler::EventHandler, interop::init_global_state};

mod app;
mod dispatcher;
mod event;
mod event_handler;
mod interop;
mod log;
mod theme;

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let events = EventHandler::new(core::time::Duration::from_millis(33));
    init_global_state(events.sender());

    let terminal = create_terminal(
        WebGl2BackendOptions::new()
            .enable_console_debug_api()
            .enable_mouse_selection()
            .grid_id("container")
            .measure_performance(true),
    )?;

    run_app(events, terminal);

    Ok(())
}

fn run_app(events: EventHandler, terminal: Terminal<WebGl2Backend>) {
    let mut app = App::new();
    terminal.draw_web(move |frame| {
        for e in events.iter() {
            app.apply_event(e);
        }

        app.render(frame);
    });
}

fn create_terminal(options: WebGl2BackendOptions) -> Result<Terminal<WebGl2Backend>> {
    let backend = WebGl2Backend::new_with_options(options).map_err(|e| eyre::eyre!("{:?}", e))?;
    let terminal = Terminal::new(backend).map_err(|e| eyre::eyre!("{:?}", e))?;

    Ok(terminal)
}
