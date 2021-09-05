# colorstyle in Rust
colorStyle is a library of styles for command-line text.
Inspired by https://github.com/flylog/colorstyle.
Is the implementation of colorstyle written in Rust.

## example

```
	let text = colorstyle::green("green");
 	println!("a {} text", text);

 	let text = colorstyle::blue("Blue");
	println!("a {} text\n", text);

	let text = colorstyle::CSS::new().color_red().sprint("red");
	println!("a {} text",text);

	colorstyle::CSS::new().style_italic().color_red().bg_yellow().println("a italic red bgYellow text: {} Hello 世界!");

	colorstyle::CSS::new().style_bold().println("a bold text: {}Hello 世界!");

	colorstyle::CSS::new().style_italic().println("a italic text: {}Hello 世界!");

	colorstyle::CSS::new().color_magenta().println("a magenta text: {}Hello 世界!");

	colorstyle::CSS::new().bg_cyan().println("a background color cyan text: {}Hello 世界!");

	colorstyle::CSS::new().bg_cyan().println("a background color cyan text");
	
	let mut css = colorstyle::CSS::new();
	css.style_strikethrough().println("删除线文本");

	css.style_underline().println("下划线文本");

	css.style_reverse().println("反显文本");
```

## Doc

See this document at [RustDoc](https://docs.rs/colorstyle)
