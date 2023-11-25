#[derive(Default)]
pub struct ScrollProvider {
    current_scroll: u32,
    frame_height: u32,
    current_line: u32,
    maximum_lines: Option<u32>,
}

impl ScrollProvider {
    /// Create a new [`ScrollProvider`] with the given frame height and maximum lines.
    pub fn new(frame_height: u32, maximum_lines: Option<u32>) -> Self {
        Self {
            current_scroll: 0,
            current_line: 0,
            frame_height,
            maximum_lines,
        }
    }

    pub fn scroll_down(&mut self, amount: u32) {
        let mut new_line = self.current_line + amount;
        if let Some(max) = self.maximum_lines {
            new_line = new_line.min(max - 1);
        }

        let line_pos = new_line as i32 - self.current_scroll as i32;
        if line_pos >= self.frame_height as i32 {
            // line pos starts from zero but frame height starts from 1 :p
            self.current_scroll = self.current_scroll.saturating_add_signed(line_pos - (self.frame_height as i32 - 1));
        }

        self.current_line = new_line;
    }

    pub fn scroll_up(&mut self, amount: u32) {
        let new_line = self.current_line.saturating_sub(amount);

        if new_line < self.current_scroll {
            self.current_scroll = new_line;
        }

        self.current_line = new_line;
    }

    pub fn move_down(&mut self, amount: u32) {
        // scroll current line to bottom of current frame
        self.current_line = self.current_scroll + self.frame_height - 1;
        self.scroll_down(amount);
    }

    pub fn move_up(&mut self, amount: u32) {
        // scroll current line to top of current_frame
        self.current_line = self.current_scroll;
        self.scroll_up(amount);
    }

    pub fn set_max(&mut self, mut max: u32) {
        max = max.max(1);
        self.maximum_lines = Some(max);
        if self.current_line >= max {
            self.current_scroll = max.saturating_sub(self.frame_height);
            max = max.saturating_sub(1);
            self.current_line = max;
        }
    }

    pub fn clear_max(&mut self) {
        self.maximum_lines = None
    }

    pub fn update_frame_height(&mut self, height: u32) {
        self.frame_height = height
    }

    pub fn get_scroll(&self) -> u32 {
        self.current_scroll
    }

    pub fn get_line(&self) -> u32 {
        self.current_line
    }

    pub fn reset(&mut self) {
        self.current_line = 0;
        self.current_scroll = 0;
    }
}