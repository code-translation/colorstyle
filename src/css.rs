#![allow(unused)]
const ANSI_SET: &str = "\033[";
const ANSI_END: &str = "m";
const ANSI_RESET: &str = "\033[0m";

#[derive(Default)]
pub struct CSS {
    text_style: Option<String>,
    text_color: Option<String>,
    bg_color: Option<String>,
}

#[derive(Copy, Clone)]
pub enum Style {
    Normal,
    Bold,
    Grey,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    Reverse,
    Hide,
    Strikethrough,
}

impl Style {
    fn value(&self) -> String {
        (*self as i32).to_string()
    }
}

#[derive(Copy, Clone)]
pub enum FgColor {
    FgBlack = 30,
    FgRed,
    FgGreen,
    FgYellow,
    FgBlue,
    FgMagenta,
    FgCyan,
    FgWhite,
    FgGray = 82,
    FgBrightRed,
    FgBrightGreen,
    FgBrightYellow,
    FgBrightBlue,
    FgBrightMagenta,
    FgBrightCyan,
    FgBrightWhite,
}

impl FgColor {
    fn value(&self) -> String {
        (*self as i32).to_string()
    }
}

#[derive(Copy, Clone)]
pub enum BgColor {
    BgBlack = 40,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgGray = 92,
    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
}

impl BgColor {
    fn value(&self) -> String {
        (*self as i32).to_string()
    }
}

impl CSS {
    pub fn new() -> CSS {
        CSS::default()
    }

    pub fn set_style(&mut self, s: Style) -> &mut CSS {
        self.text_style = Some(s.value());
        self
    }

    pub fn set_color(&mut self, s: FgColor) -> &mut CSS {
        self.text_color = Some(s.value());
        self
    }

    pub fn set_bg_color(&mut self, s: BgColor) -> &mut CSS {
        self.bg_color = Some(s.value());
        self
    }

    fn decorated_string(&self) -> (String, String) {
        let mut codes: Vec<String> = vec![];

        if let Some(s) = self.text_style.as_ref() {
            codes.push(s.to_string());
        }

        if let Some(s) = self.text_color.as_ref() {
            codes.push(s.to_string());
        }

        if let Some(s) = self.bg_color.as_ref() {
            codes.push(s.to_string());
        }
        codes.push(':'.to_string());
        codes.push(ANSI_END.to_string());
        let mut start = String::from("");
        for v in codes.into_iter() {
            start.push_str(&v);
        }

        (start, ANSI_RESET.to_string())
    }

    pub fn println(&self, text: &str) {
        let (start, end) = self.decorated_string();
        println!("{}{}{}", start, text, end);
    }

    pub fn style_default(&mut self) -> &mut CSS {
        self.set_style(Style::Normal);
        self
    }

    pub fn style_bold(&mut self) -> &mut CSS {
        self.set_style(Style::Bold);
        self
    }

    pub fn style_grey(&mut self) -> &mut CSS {
        self.set_style(Style::Grey);
        self
    }

    pub fn style_italic(&mut self) -> &mut CSS {
        self.set_style(Style::Italic);
        self
    }

    pub fn style_underline(&mut self) -> &mut CSS {
        self.set_style(Style::Underline);
        self
    }

    pub fn style_rapid_blink(&mut self) -> &mut CSS {
        self.set_style(Style::RapidBlink);
        self
    }

    pub fn style_slow_blink(&mut self) -> &mut CSS {
        self.set_style(Style::SlowBlink);
        self
    }
}
