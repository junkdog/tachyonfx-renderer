use ratatui::Terminal;
use ratzilla::backend::webgl2::WebGl2BackendOptions;
use ratzilla::{WebGl2Backend, WebRenderer};
use eyre::Result;
use crate::event_handler::EventHandler;
use crate::interop::init_global_state;

mod app;
mod dispatcher;
mod effects;
mod event;
mod event_handler;
mod theme;
mod interop;

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    let events = EventHandler::new(core::time::Duration::from_millis(33));
    init_global_state(events.sender());

    let terminal = create_terminal(WebGl2BackendOptions::new()
        .enable_console_debug_api()
        .enable_mouse_selection()
        .grid_id("container")
        .measure_performance(true)
    )?;


    let mut app = app::App::new(events.sender());

    terminal.draw_web(move |frame| {
        for e in events.iter() {
            app.apply_event(e);
        }

        app.render(frame);
    });

    Ok(())
}

fn create_terminal(options: WebGl2BackendOptions) -> Result<Terminal<WebGl2Backend>> {
    let backend = WebGl2Backend::new_with_options(options)
        .map_err(|e| eyre::eyre!("{:?}", e))?;
    let terminal = Terminal::new(backend)
        .map_err(|e| eyre::eyre!("{:?}", e))?;
    
    Ok(terminal)
}