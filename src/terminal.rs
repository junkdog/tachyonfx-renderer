use eyre::Result;
use ratatui::Terminal;
use ratzilla::{WebGl2Backend, backend::webgl2::WebGl2BackendOptions};

pub fn create_terminal(container_id: &str) -> Result<Terminal<WebGl2Backend>> {
    let backend = WebGl2Backend::new_with_options(
        WebGl2BackendOptions::new()
            .enable_console_debug_api()
            .enable_mouse_selection()
            .grid_id(container_id)
            .measure_performance(true),
    )
    .map_err(|e| eyre::eyre!("{:?}", e))?;

    let terminal = Terminal::new(backend).map_err(|e| eyre::eyre!("{:?}", e))?;

    Ok(terminal)
}
