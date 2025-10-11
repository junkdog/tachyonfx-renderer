pub enum AppEvent {
    CompileDsl(String),
    ReplaceCanvas(String),
    ReplayCurrentEffect,
    Resize(u16, u16),
    Tick,
}
