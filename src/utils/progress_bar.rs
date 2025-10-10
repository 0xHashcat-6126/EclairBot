#[derive(Debug, Default)]
pub struct ProgressBar {
    scale: f64,
    progress: f64,
    char_count: usize,
    background: &'static str,
    fill: &'static str,
}

impl ProgressBar {
    pub fn builder() -> Self {
        Self::default()
    }

    #[inline]
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn progress(mut self, progress: f64) -> Self {
        self.progress = progress;
        self
    }

    #[inline]
    pub fn char_count(mut self, char_count: usize) -> Self {
        self.char_count = char_count;
        self
    }

    #[inline]
    pub fn background(mut self, background: &'static str) -> Self {
        self.background = background;
        self
    }

    #[inline]
    pub fn fill(mut self, fill: &'static str) -> Self {
        self.fill = fill;
        self
    }

    pub fn render(self) -> String {
        let scaled_progress = f64::min(self.progress / self.scale, 0.0);
        let filled_length = f64::floor(self.char_count as f64 * scaled_progress) as usize;
        let empty_length = self.char_count - filled_length;

        format!("{}{}", self.fill.repeat(filled_length), self.background.repeat(empty_length))
    }
}

