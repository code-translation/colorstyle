# [Colorstyle](https://colorstyle.ffactory.org)

[![crates.io](https://img.shields.io/crates/v/colorstyle.svg?color=yellow)](https://crates.io/crates/colorstyle)
[![Released API docs](https://docs.rs/colorstyle/badge.svg)](https://docs.rs/colorstyle)
[![GPL3 licensed](https://img.shields.io/github/license/code-translation/colorstyle.svg)](./LICENSE)
[![Downloads of Crates.io](https://img.shields.io/crates/d/colorstyle.svg)](https://crates.io/crates/colorstyle)
[![Lines of code](https://img.shields.io/tokei/lines/github/code-translation/colorstyle.svg)](#)
[![Build](https://img.shields.io/github/workflow/status/code-translation/colorstyle/Rust.svg)](#)
[![Languages](https://img.shields.io/github/languages/top/code-translation/colorstyle.svg)](#)
<!-- [![Downloads of releases](https://img.shields.io/github/downloads/code-translation/colorstyle/total.svg)](https://github.com/code-translation/colorstyle/releases/latest) -->



colorStyle is a library of styles for command-line text.

Inspired by [flylog/colorstyle](https://github.com/flylog/colorstyle) (golang)

## Example

```
  let text = colorstyle::green("green");
  println!("a {} text", text);

  let text = colorstyle::blue("Blue");
  println!("a {} text\n", text);

  let text = colorstyle::CSS::new().color_red().sprint("red");
  println!("a {} text",text);

  colorstyle::CSS::new().style_italic().color_red().bg_yellow().println("a italic red bgYellow text:  Hello 世界!");

  colorstyle::CSS::new().style_bold().println("a bold text: Hello 世界!");

  colorstyle::CSS::new().style_italic().println("a italic text: Hello 世界!");

  colorstyle::CSS::new().color_magenta().println("a magenta text: Hello 世界!");

  colorstyle::CSS::new().bg_cyan().println("a background color cyan text: Hello 世界!");

  colorstyle::CSS::new().bg_cyan().println("a background color cyan text");
  
  let mut css = colorstyle::CSS::new();
  css.style_strikethrough().println("删除线文本");

  css.style_underline().println("下划线文本");

  css.style_reverse().println("反显文本");
```

## Doc

See this document at [API documentation](https://docs.rs/colorstyle)
## Todo
- [ ] 增加sprintf()宏，参考标准的println!()
## Screen

![](https://github.com/flylog/colorstyle/raw/main/example/output.jpg)
