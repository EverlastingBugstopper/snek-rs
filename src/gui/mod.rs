pub struct Gui {}

impl Gui {
    pub fn new() -> Self {
        Self {}
    }

    #[tracing(level = "info")]
    pub fn run() -> ! {
        while true {}
    }
}
