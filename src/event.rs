pub enum AppEvent {
    Tick,
    Resize(u16, u16),
    ReplaceCanvas(String),
    CompileDsl(String),
}
