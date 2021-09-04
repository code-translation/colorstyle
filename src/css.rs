#![allow(unused)]
use std::fmt::format;
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
        codes.push(ANSI_SET.to_string());
        let mut count = 0;
        if let Some(s) = self.text_style.as_ref() {
            count = count + 1;
            codes.push(s.to_string());
        }

        if let Some(s) = self.text_color.as_ref() {
            count = count + 1;
            codes.push(s.to_string());
        }

        if let Some(s) = self.bg_color.as_ref() {
            count = count + 1;
            codes.push(s.to_string());
        }
        if count > 1 {
            codes.push(';'.to_string());
        }
        codes.push(ANSI_END.to_string());
        let mut start = String::from("");
        for v in codes.into_iter() {
            start.push_str(&v);
        }

        (start, ANSI_RESET.to_string())
    }

    pub fn sprint(&mut self, text: &str) -> String {
        let (mut start, end) = self.decorated_string();
        format!("{}{}{}", start, text, end)
    }

    pub fn println(&self, text: &str) {
        let (start, end) = self.decorated_string();
        println!("{}{}{}", start, text, end);
    }

    // 设置样式方法
    pub fn style_default(&mut self) -> &mut CSS {
        self.set_style(Style::Normal)
    }

    pub fn style_bold(&mut self) -> &mut CSS {
        self.set_style(Style::Bold)
    }

    pub fn style_grey(&mut self) -> &mut CSS {
        self.set_style(Style::Grey)
    }

    pub fn style_italic(&mut self) -> &mut CSS {
        self.set_style(Style::Italic)
    }

    pub fn style_underline(&mut self) -> &mut CSS {
        self.set_style(Style::Underline)
    }

    pub fn style_rapid_blink(&mut self) -> &mut CSS {
        self.set_style(Style::RapidBlink)
    }

    pub fn style_slow_blink(&mut self) -> &mut CSS {
        self.set_style(Style::SlowBlink)
    }

    pub fn style_reverse(&mut self) -> &mut CSS {
        self.set_style(Style::Reverse)
    }

    pub fn style_hide(&mut self) -> &mut CSS {
        self.set_style(Style::Hide)
    }

    pub fn style_strikethrough(&mut self) -> &mut CSS {
        self.set_style(Style::Strikethrough)
    }

    // 设置前景色方法
    pub fn color_black(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBlack)
    }

    pub fn color_red(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgRed)
    }

    pub fn color_green(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgGreen)
    }

    pub fn color_yellow(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgYellow)
    }

    pub fn color_bule(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBlue)
    }

    pub fn color_magenta(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgMagenta)
    }

    pub fn color_cyan(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgCyan)
    }

    pub fn color_white(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgWhite)
    }

    pub fn color_gray(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgGray)
    }

    pub fn color_bright_red(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightRed)
    }

    pub fn color_bright_green(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightGreen)
    }

    pub fn color_bright_yellow(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightYellow)
    }

    pub fn color_bright_bule(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightBlue)
    }

    pub fn color_bright_magenta(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightMagenta)
    }

    pub fn color_bright_cyan(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightCyan)
    }

    pub fn color_bright_white(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightWhite)
    }
    // 设置背景色方法
    pub fn bg_black(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBlack)
    }

    pub fn bg_red(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgRed)
    }

    pub fn bg_green(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgGreen)
    }

    pub fn bg_yellow(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgYellow)
    }

    pub fn bg_blue(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBlue)
    }

    pub fn bg_magenta(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgMagenta)
    }

    pub fn bg_cyan(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgCyan)
    }

    pub fn bg_white(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgWhite)
    }

    pub fn bg_gray(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgGray)
    }

    pub fn bg_bright_red(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightRed)
    }

    pub fn bg_bright_green(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightGreen)
    }

    pub fn bg_bright_yellow(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightYellow)
    }

    pub fn bg_bright_blue(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightMagenta)
    }

    pub fn bg_bright_cyan(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightCyan)
    }

    pub fn bg_bright_white(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightWhite)
    }
    // 方便函数
    pub fn black(text: &str) -> String {
        let mut c = CSS::new();
        c.color_black().sprint(text)
    }

    pub fn red(text: &str) -> String {
        let mut c = CSS::new();
        c.color_red().sprint(text)
    }

    pub fn green(text: &str) -> String {
        let mut c = CSS::new();
        c.color_green().sprint(text)
    }

    pub fn yellow(text: &str) -> String {
        let mut c = CSS::new();
        c.color_yellow().sprint(text)
    }

    pub fn blue(text: &str) -> String {
        let mut c = CSS::new();
        c.color_bule().sprint(text)
    }

    pub fn magenta(text: &str) -> String {
        let mut c = CSS::new();
        c.color_magenta().sprint(text)
    }

    pub fn cyan(text: &str) -> String {
        let mut c = CSS::new();
        c.color_cyan().sprint(text)
    }

    pub fn white(text: &str) -> String {
        let mut c = CSS::new();
        c.color_white().sprint(text)
    }

    pub fn gray(text: &str) -> String {
        let mut c = CSS::new();
        c.color_gray().sprint(text)
    }

    pub fn bright_red(text: &str) -> String {
        let mut c = CSS::new();
        c.color_bright_red().sprint(text)
    }

    pub fn bright_yellow(text: &str) -> String {
        let mut c = CSS::new();
        c.color_yellow().sprint(text)
    }

    pub fn bright_blue(text: &str) -> String {
        let mut c = CSS::new();
        c.color_bright_bule().sprint(text)
    }

    pub fn bright_magenta(text: &str) -> String {
        let mut c = CSS::new();
        c.color_bright_magenta().sprint(text)
    }

    pub fn bright_cyan(text: &str) -> String {
        let mut c = CSS::new();
        c.color_bright_cyan().sprint(text)
    }

    pub fn bright_white(text: &str) -> String {
        let mut c = CSS::new();
        c.color_bright_white().sprint(text)
    }
}
