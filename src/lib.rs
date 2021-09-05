#![crate_name = "colorstyle"]
#![allow(unused)]
//! ColorStyle is a library of styles for command-line text.
//! Used to modify the style of text for standard output to the terminal interface, you can change the foreground colour of the text, the background colour, add underline and bold, etc.
//!
//! ColorStyle 是一个用于命令行文本的样式库。
//! 用于标准输出到终端界面的文本的样式修改，可以修改文本前景色，背景色，增加下划线和加粗显等。
//!	# Example
//!	```
//!	let text = colorstyle::green("green");
//! println!("a {} text", text);

//!	let text = colorstyle::blue("Blue");
//!	println!("a {} text\n", text);

//!	let text = colorstyle::CSS::new().color_red().sprint("red");
//!	println!("a {} text",text);

//!	colorstyle::CSS::new().style_italic().color_red().bg_yellow().println("a italic red bgYellow text: {} Hello 世界!");
//!	colorstyle::CSS::new().style_bold().println("a bold text: {}Hello 世界!");
//!	colorstyle::CSS::new().style_italic().println("a italic text: {}Hello 世界!");
//!	colorstyle::CSS::new().color_magenta().println("a magenta text: {}Hello 世界!");
//!	colorstyle::CSS::new().bg_cyan().println("a background color cyan text: {}Hello 世界!");
//!	colorstyle::CSS::new().bg_cyan().println("a background color cyan text");

//!	let mut css = colorstyle::CSS::new();
//!	css.style_strikethrough().println("删除线文本");
//!	css.style_underline().println("下划线文本");
//!	css.style_reverse().println("反显文本");
//!	```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let source = "Hello 世界";
        let s = crate::bright_red("Hello 世界");
        let true_str = format!("{}{}{}", "\033[83m", source, "\033[0m");
        assert_eq!(s, true_str)
    }
}
use std::fmt::format;
/// ANSI转义码，设置文本样式的开头
const ANSI_SET: &str = "\033[";
/// 设置结束字符
const ANSI_END: &str = "m";
/// ANSI转义码,重置设置到常规
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
    /// 新建一个文本样式对象
    ///
    /// Create a new text style object
    /// # Example
    /// ```
    /// let mut css = colorstyle::CSS::new();
    /// css.style_italic().color_green().bg_yellow();
    /// css.println("a italic green bgYellow text:Hello 世界!");
    /// ```
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

    /// like println!()
    /// 在标准输出打印装饰过的文本
    ///
    /// Print decorated text in the standard output
    /// # Example
    /// ```
    /// colorstyle::CSS::new().bg_blue().println("a background color blue text");
    /// ```
    pub fn println(&mut self, text: &str) {
        let (start, end) = self.decorated_string();
        println!("{}{}{}", start, text, end);
    }

    /// 返回装饰过的文本
    ///
    /// Return the decorated text
    /// # Example
    /// ```
    /// let text = colorstyle::CSS::new().color_bright_magenta().sprint("bright_magenta");
    /// println!("a {} text", text);
    /// ```
    pub fn sprint(&mut self, text: &str) -> String {
        let (mut start, end) = self.decorated_string();
        format!("{}{}{}", start, text, end)
    }

    // 设置字体样式方法
    /// 设置字体样式为常规
    ///
    /// Set font style to default
    pub fn style_default(&mut self) -> &mut CSS {
        self.set_style(Style::Normal)
    }

    /// 设置字体样式为粗体
    ///
    /// Set font style to bold
    /// # Example
    /// ```
    /// colorstyle::CSS::new().style_bold().println("a bold text: Hello 世界!");
    ///  
    /// //or
    ///  let mut css = colorstyle::CSS::new();
    ///  css.style_bold().println("a bold text: Hello 世界!");
    /// ```
    pub fn style_bold(&mut self) -> &mut CSS {
        self.set_style(Style::Bold)
    }

    /// 设置字体样式为灰显
    ///
    /// Set font style to grey
    pub fn style_grey(&mut self) -> &mut CSS {
        self.set_style(Style::Grey)
    }

    /// 设置字体样式为斜体
    ///
    /// Set font style to italic
    pub fn style_italic(&mut self) -> &mut CSS {
        self.set_style(Style::Italic)
    }

    /// 设置字体样式为下划线
    ///
    /// Set font style to underline
    pub fn style_underline(&mut self) -> &mut CSS {
        self.set_style(Style::Underline)
    }

    /// 设置字体样式为缓慢闪烁
    ///
    /// Set font style t0 rapid blink
    pub fn style_rapid_blink(&mut self) -> &mut CSS {
        self.set_style(Style::RapidBlink)
    }

    /// 设置字体样式为快速闪烁
    ///
    /// Set font style t0 slow blink
    pub fn style_slow_blink(&mut self) -> &mut CSS {
        self.set_style(Style::SlowBlink)
    }

    /// 设置字体样式为反显
    ///
    /// Set font style to reverse
    pub fn style_reverse(&mut self) -> &mut CSS {
        self.set_style(Style::Reverse)
    }

    /// 设置字体样式为删除线,可能不支持
    ///
    /// Set font style to strikethrough，may not be supported
    pub fn style_hide(&mut self) -> &mut CSS {
        self.set_style(Style::Hide)
    }

    /// 设置字体样式为删除线,可能不支持
    ///
    /// Set font style to strikethrough，may not be supported
    pub fn style_strikethrough(&mut self) -> &mut CSS {
        self.set_style(Style::Strikethrough)
    }

    // 设置前景色方法
    /// 设置前景色为黑色
    ///
    /// Set foreground colour to black
    pub fn color_black(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBlack)
    }

    /// 设置前景色为红色
    ///
    /// Set foreground colour to red
    /// # Example
    /// ```
    /// colorstyle::CSS::new().color_red().println("a red text: Hello 世界!");
    /// ```
    pub fn color_red(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgRed)
    }

    /// 设置前景色为绿色
    ///
    /// Set foreground colour to green
    pub fn color_green(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgGreen)
    }

    /// 设置前景色为黄色
    ///
    /// Set foreground colour to yellow
    pub fn color_yellow(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgYellow)
    }

    /// 设置前景色为蓝色
    ///
    /// Set foreground colour to blue
    pub fn color_bule(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBlue)
    }

    /// 设置前景色为品红色
    ///
    /// Set foreground colour to magenta
    pub fn color_magenta(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgMagenta)
    }

    /// 设置前景为青色
    ///
    /// Set foreground colour to cyan
    pub fn color_cyan(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgCyan)
    }

    /// 设置前景色为白色
    ///
    /// Set foreground colour to whilte
    pub fn color_white(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgWhite)
    }

    /// 设置前景色为灰色
    ///
    /// Set foreground colour to cray
    pub fn color_gray(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgGray)
    }

    /// 设置前景色为亮红色
    ///
    /// Set foreground colour to bright red
    pub fn color_bright_red(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightRed)
    }

    /// 设置前景色为亮绿色
    ///
    /// Set foreground colour to bright green
    pub fn color_bright_green(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightGreen)
    }

    /// 设置前景色为亮黄色
    ///
    /// Set foreground colour to bright yellow
    pub fn color_bright_yellow(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightYellow)
    }

    /// 设置前景色为亮蓝色
    ///
    /// Set foreground colour to bright blue
    pub fn color_bright_bule(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightBlue)
    }

    /// 设置前景色为亮品红色
    ///
    /// Set foreground colour to bright magenta
    pub fn color_bright_magenta(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightMagenta)
    }

    /// 设置前景亮青色
    ///
    /// Set foreground colour to bright cyan
    pub fn color_bright_cyan(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightCyan)
    }

    /// 设置前景色为亮白色
    ///
    /// Set foreground colour to bright whilte
    pub fn color_bright_white(&mut self) -> &mut CSS {
        self.set_color(FgColor::FgBrightWhite)
    }

    // 设置背景色方法
    /// 设置背景颜色为黑色
    ///
    /// Set the background colour to black
    pub fn bg_black(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBlack)
    }

    /// 设置背景颜色为红色
    ///
    /// Set the background colour to red
    pub fn bg_red(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgRed)
    }

    /// 设置背景颜色为绿色
    ///
    /// Set the background colour to green
    pub fn bg_green(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgGreen)
    }

    /// 设置背景颜色为黄色
    ///
    /// Set the background colour to yellow
    pub fn bg_yellow(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgYellow)
    }

    /// 设置背景颜色为蓝色
    ///
    /// Set the background colour to blue
    pub fn bg_blue(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBlue)
    }

    /// 设置背景颜色为品红
    ///
    /// Set the background colour to magenta
    pub fn bg_magenta(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgMagenta)
    }

    /// 设置背景颜色为青色
    ///
    /// Set the background colour to cyan
    pub fn bg_cyan(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgCyan)
    }

    /// 设置背景颜色为白色
    ///
    /// Set the background colour to white
    pub fn bg_white(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgWhite)
    }

    /// 设置背景颜色为灰色
    ///
    /// Set the background colour to gray
    pub fn bg_gray(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgGray)
    }

    /// 设置背景颜色为亮红色
    ///
    /// Set the background colour to bright red
    pub fn bg_bright_red(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightRed)
    }

    /// 设置背景颜色为亮绿色
    ///
    /// Set the background colour to bright green
    pub fn bg_bright_green(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightGreen)
    }

    /// 设置背景颜色为亮黄色
    ///
    /// Set the background colour to bright yellow
    pub fn bg_bright_yellow(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightYellow)
    }

    /// 设置背景颜色为亮蓝色
    ///
    /// Set the background colour to bright blue
    pub fn bg_bright_blue(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightMagenta)
    }

    /// 设置背景颜色为亮品红色
    ///
    /// Set the background colour to  bright magenta
    pub fn bg_bright_magenta(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightMagenta)
    }

    /// 设置背景颜色为亮青色
    ///
    /// Set the background colour to bright cyan

    pub fn bg_bright_cyan(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightCyan)
    }

    ///  设置背景颜色为亮白色
    ///
    /// Set the background colour to bright white
    pub fn bg_bright_white(&mut self) -> &mut CSS {
        self.set_bg_color(BgColor::BgBrightWhite)
    }
}
// 方便函数
/// 生成黑色的文本
///
/// Generate black text
pub fn black(text: &str) -> String {
    let mut c = CSS::new();
    c.color_black().sprint(text)
}

/// 生成红色的文本
///
/// Generate red text
pub fn red(text: &str) -> String {
    let mut c = CSS::new();
    c.color_red().sprint(text)
}

/// 生成绿色的文本
///
/// Generate green text
/// # Example
/// ```
/// let text = colorstyle::green("green");
/// println!("a {} text\n", text);
/// ```
pub fn green(text: &str) -> String {
    let mut c = CSS::new();
    c.color_green().sprint(text)
}

/// 生成黄色的文本
///
/// Generate yellow text
pub fn yellow(text: &str) -> String {
    let mut c = CSS::new();
    c.color_yellow().sprint(text)
}

/// 生成蓝色的文本
///
/// Generate blue text
/// # Example
/// ```
/// let text = colorstyle::blue("blue");
/// println!("a {} text",text);
/// ```
pub fn blue(text: &str) -> String {
    let mut c = CSS::new();
    c.color_bule().sprint(text)
}

/// 生成品红色的文本
///
/// Generate magenta text
pub fn magenta(text: &str) -> String {
    let mut c = CSS::new();
    c.color_magenta().sprint(text)
}

/// 生成青色的文本
///
/// Generate cyan text
pub fn cyan(text: &str) -> String {
    let mut c = CSS::new();
    c.color_cyan().sprint(text)
}

/// 生成白色的文本
///
/// Generate white text
pub fn white(text: &str) -> String {
    let mut c = CSS::new();
    c.color_white().sprint(text)
}

/// 生成灰色的文本
///
/// Generate gray text
pub fn gray(text: &str) -> String {
    let mut c = CSS::new();
    c.color_gray().sprint(text)
}

/// 生成亮红色的文本
///
/// Generate bright red text
pub fn bright_red(text: &str) -> String {
    let mut c = CSS::new();
    c.color_bright_red().sprint(text)
}

/// 生成亮黄色的文本
///
/// Generate bright yellow text
pub fn bright_yellow(text: &str) -> String {
    let mut c = CSS::new();
    c.color_yellow().sprint(text)
}

/// 生成亮蓝的文本
///
/// Generate bright blue text
pub fn bright_blue(text: &str) -> String {
    let mut c = CSS::new();
    c.color_bright_bule().sprint(text)
}

/// 生成亮品红的文本
///
/// Generate bright magenta text
pub fn bright_magenta(text: &str) -> String {
    let mut c = CSS::new();
    c.color_bright_magenta().sprint(text)
}

/// 生成亮青色的文本
///
/// Generate bright cyan text
pub fn bright_cyan(text: &str) -> String {
    let mut c = CSS::new();
    c.color_bright_cyan().sprint(text)
}

/// 生成亮白色的文本
///
/// Generate bright white text
pub fn bright_white(text: &str) -> String {
    let mut c = CSS::new();
    c.color_bright_white().sprint(text)
}
