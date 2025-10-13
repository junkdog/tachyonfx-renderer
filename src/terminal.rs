use eyre::Result;
use ratatui::Terminal;
use ratzilla::{WebGl2Backend, backend::webgl2::WebGl2BackendOptions};
use ratzilla::backend::webgl2::FontAtlasData;

pub fn create_terminal(
    container_id: &str,
    terminal_size: (u16, u16),
) -> Result<Terminal<WebGl2Backend>> {

    let backend = WebGl2Backend::new_with_options(
        WebGl2BackendOptions::new()
            .size(calculate_canvas_size(terminal_size))
            .enable_mouse_selection()
            .grid_id(container_id)
            .measure_performance(true),
    )
    .map_err(|e| eyre::eyre!("{:?}", e))?;

    let terminal = Terminal::new(backend).map_err(|e| eyre::eyre!("{:?}", e))?;
    Ok(terminal)
}

fn calculate_canvas_size(
    terminal_size: (u16, u16),
) -> (u32, u32) {
    let (w, h) = FontAtlasData::default().cell_size;
    (
        terminal_size.0 as u32 * w as u32 - 2,
        terminal_size.1 as u32 * h as u32 - 2,
    )
}