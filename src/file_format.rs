
pub enum Format {
    PP3(u32, u32, u32),
}

impl Format {
    pub fn print_header(&self) -> String {
        match self {
            Format::PP3(width, height, max_color) => {
                format!("P3\n{} {}\n{}\n", width, height, max_color)
            },
        }
    }
    pub fn max_color(&self) -> u32 {
        match self {
            Format::PP3(_, _, max_color) => *max_color,
        }
    }
}
