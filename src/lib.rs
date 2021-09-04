pub mod css;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let source = "Hello 世界";
        use crate::css::CSS;
        let s = CSS::bright_red("Hello 世界");
        let true_str = format!("{}{}{}", "\033[83m", source, "\033[0m");
        assert_eq!(s, true_str)
    }
}
