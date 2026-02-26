use eyre::Result;
use ratatui::Terminal;
use ratzilla::{WebGl2Backend, backend::webgl2::WebGl2BackendOptions};

pub fn create_terminal(options: WebGl2BackendOptions) -> Result<Terminal<WebGl2Backend>> {
    let backend = WebGl2Backend::new_with_options(options).map_err(|e| eyre::eyre!("{:?}", e))?;

    let terminal = Terminal::new(backend).map_err(|e| eyre::eyre!("{:?}", e))?;
    Ok(terminal)
}

pub fn create_terminal_with_resize(
    options: WebGl2BackendOptions,
    terminal_size: (u16, u16),
) -> Result<Terminal<WebGl2Backend>> {
    let mut backend =
        WebGl2Backend::new_with_options(options).map_err(|e| eyre::eyre!("{:?}", e))?;

    let (cell_w, cell_h) = backend.cell_size();
    let width = terminal_size.0 as u32 * cell_w as u32;
    let height = terminal_size.1 as u32 * cell_h as u32;
    backend
        .set_size(width, height)
        .map_err(|e| eyre::eyre!("{:?}", e))?;

    Terminal::new(backend).map_err(|e| eyre::eyre!("{:?}", e))
}
