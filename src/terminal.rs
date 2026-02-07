use eyre::Result;
use ratatui::Terminal;
use ratzilla::{WebGl2Backend, backend::webgl2::WebGl2BackendOptions};

pub fn create_terminal(options: WebGl2BackendOptions) -> Result<Terminal<WebGl2Backend>> {
    let backend = WebGl2Backend::new_with_options(options).map_err(|e| eyre::eyre!("{:?}", e))?;

    let terminal = Terminal::new(backend).map_err(|e| eyre::eyre!("{:?}", e))?;
    Ok(terminal)
}
