#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeScript {
    // Add more scripts for supporting more languages.
    Common,
    Han,
    Latin,
}


pub fn get_unicode_script(c: char) -> UnicodeScript {

    // For now, this only supports chinese, subset of latin characters and
    // punctuations.
    match c as usize {
        // C0 control codes, ASCII Punctuation and Symbols, ASCII digits.
        0x0000..=0x0040 => UnicodeScript::Common,
        0x005B..=0x0060 => UnicodeScript::Common,
        0x007B..=0x007E => UnicodeScript::Common,

        // Multiplication and division signs.
        0x00D7 => UnicodeScript::Common,
        0x00F7 => UnicodeScript::Common,

        // Latin-1 punctuations and symbols: counted in "Common".
        0x00A0..=0x00BF => UnicodeScript::Common,

        // Spacing Modifier letters.
        0x02B0..=0x02FF => UnicodeScript::Common,

        // Chinese characters in CJK unicode block.
        0x4E00..=0x9FFF => UnicodeScript::Han,
        0x3400..=0x4DBF => UnicodeScript::Han,
        0xF900..=0xFAFF => UnicodeScript::Han,
        0x20000..=0x2A6DF => UnicodeScript::Han,
        0x2A700..=0x2B73F => UnicodeScript::Han,
        0x2B740..=0x2B81F => UnicodeScript::Han,
        0x2B920..=0x2CEAF => UnicodeScript::Han,
        0x2F800..=0x2FA1F => UnicodeScript::Han,

        // Latin alphabet: uppercase and lowercase.
        0x0041..=0x005A => UnicodeScript::Latin,
        0x0061..=0x007A => UnicodeScript::Latin,

        // Latin alphabet: uppercase with accents.
        0x00C0..=0x00D6 => UnicodeScript::Latin,
        0x00D8..=0x00DE => UnicodeScript::Latin,

        // Latin alphabet: lowercase with accents.
        0x00DF..=0x00F6 => UnicodeScript::Latin,
        0x00F8..=0x00FF => UnicodeScript::Latin,

        // Latin Extended (A/B): European, historic and non-European.
        0x0100..=0x01BF => UnicodeScript::Latin,

        // Latin Extended B: African, Croatian, Pinyin, Slovenian, Romanian
        //     Livonian, Sinology, Miscellaneous,
        0x01C0..=0x024F => UnicodeScript::Latin,

        // Latin Extended C/D/E.
        0x0230..=0x02B0 => UnicodeScript::Latin,

        // Any character not covered above is "common" by default.
        _ => UnicodeScript::Common,
    }
}
