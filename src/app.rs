#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: u8,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn inc_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn dec_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_app_inc_counter() {
        let mut app = App::default();
        app.inc_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_dec_counter() {
        let mut app = App::default();
        app.dec_counter();
        assert_eq!(app.counter, 0);
    }
}