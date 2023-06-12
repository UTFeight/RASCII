pub const BLOCK: &str = " ░▒▓█";
pub const CHINESE: &str = "\u{3000}一二十人丁口王日木金華爱黑墨龍龘";
#[rustfmt::skip]
pub const DEFAULT: &str = "  .`^\"\\,:;Il!i><~+_-?][}{1)(|\\\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B$@";
pub const EMOJI: &str = "   。，🧔👶🗣👥👤👀👁🦴🦷🫁🫀🧠👃🦻👂👅🦀👿🦀👄🤳💅🖖👆🙏🤝🦿🦾💪🤏👌🤘🤞👊🤚🤛🙌😾😿🙀😺👾👽👻💀👺🦀👹🤡💤😴🥸🥳🥶🥵🤮🤢🤕😭😓😯😰😨😱😮😩😫🙁😔😡🤬😠🙄😐😶🧐😛🤗🤐🤑😝🤩😋😊😉🤣😅😆";
pub const RUSSIAN: &str = "  ЯЮЭЬЫЪЩШЧЦХФУТСPПОНМЛКЙИЗЖЁЕДГВБА";
pub const SLIGHT: &str = "  .`\"\\:I!>~_?[{)|\\\\YLpda*W8%@$";

#[derive(Debug, Clone)]
pub enum Charset<'a> {
    Block,
    Chinese,
    Default,
    Emoji,
    Russian,
    Slight,
    Custom(&'a str),
}

/// `Charset::new("block")` -> `Charset::Custom("block")`
impl<'a> Charset<'a> {
    #[allow(dead_code)]
    pub fn new(s: &'a str) -> Self {
        Self::Custom(s)
    }
}

/// `"block".into()` -> `Charset::Block`
impl<'a> From<&'a str> for Charset<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "block" => Self::Block,
            "chinese" => Self::Chinese,
            "default" => Self::Default,
            "emoji" => Self::Emoji,
            "russian" => Self::Russian,
            "slight" => Self::Slight,
            _ => Self::Custom(s),
        }
    }
}

/// `Charset::Block.into()` -> `" ░▒▓█"`
impl<'a> From<&Charset<'a>> for &'a str {
    fn from(val: &Charset<'a>) -> Self {
        match val {
            Charset::Block => BLOCK,
            Charset::Chinese => CHINESE,
            Charset::Default => DEFAULT,
            Charset::Emoji => EMOJI,
            Charset::Russian => RUSSIAN,
            Charset::Slight => SLIGHT,
            Charset::Custom(s) => s,
        }
    }
}
