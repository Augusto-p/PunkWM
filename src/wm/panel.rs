pub struct PanelOptions {
    open: bool,
    width: u16,
    current_width: u16,
}

impl PanelOptions {
    /// Constructor
    pub fn new(width: u16) -> Self {
        Self {
            open: false,
            width,
            current_width: 0,
        }
    }

    // -------- getters --------
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn current_width(&self) -> u16 {
        self.current_width
    }

    // -------- setters --------
    pub fn open(&mut self) {
        if !self.open{
            self.current_width = self.current_width.saturating_add(self.width);
            self.open = true;
        }
    }
    pub fn close(&mut self) {
        if self.open{
            self.current_width = self.current_width.saturating_sub(self.width);
            self.open = false;
        }
    }
    pub fn set_current_width(&mut self, current_width: u16) {
        self.current_width = current_width;
    }


}