use app::App;
use eyre::Result;
use ratzilla::WebRenderer;

use crate::{event_handler::EventHandler, interop::init_global_state, terminal::create_terminal};

mod app;
mod dispatcher;
mod event;
mod event_handler;
mod interop;
mod log;
mod terminal;
mod theme;

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let events = EventHandler::new(core::time::Duration::from_millis(33));
    init_global_state(events.sender());

    let terminal = create_terminal("container")?;

    run_app(events, terminal);

    Ok(())
}

fn run_app(events: EventHandler, terminal: ratatui::Terminal<ratzilla::WebGl2Backend>) {
    let mut app = App::new();
    terminal.draw_web(move |frame| {
        for e in events.iter() {
            app.apply_event(e);
        }

        app.render(frame);
    });
}
