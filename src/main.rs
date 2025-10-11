use ratatui::Terminal;
use ratzilla::backend::webgl2::WebGl2BackendOptions;
use ratzilla::{WebGl2Backend, WebRenderer};
use eyre::Result;

mod app;
mod dispatcher;
mod effects;
mod event;
mod event_handler;
mod theme;

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let options = WebGl2BackendOptions::new()
            .measure_performance(true)
            .grid_id("container")
            .enable_console_debug_api();

    let backend = WebGl2Backend::new_with_options(options)
        .map_err(|e| eyre::eyre!("{:?}", e))?;
    let terminal = Terminal::new(backend)
        .map_err(|e| eyre::eyre!("{:?}", e))?;

    let mut last_tick = web_time::Instant::now();

    terminal.draw_web(move |frame| {
        let now = web_time::Instant::now();
        let elapsed = now.duration_since(last_tick);
        last_tick = now;

        // frame.render_effect(&mut effect, frame.area(), elapsed.into());
    });
    Ok(())
}
