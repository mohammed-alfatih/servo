/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

<% data.new_style_struct("List", inherited=True) %>

${helpers.single_keyword("list-style-position", "outside inside")}

// TODO(pcwalton): Implement the full set of counter styles per CSS-COUNTER-STYLES [1] 6.1:
//
//     decimal-leading-zero, armenian, upper-armenian, lower-armenian, georgian, lower-roman,
//     upper-roman
//
// [1]: http://dev.w3.org/csswg/css-counter-styles/
${helpers.single_keyword("list-style-type", """
    disc none circle square decimal arabic-indic bengali cambodian cjk-decimal devanagari
    gujarati gurmukhi kannada khmer lao malayalam mongolian myanmar oriya persian telugu thai
    tibetan lower-alpha upper-alpha cjk-earthly-branch cjk-heavenly-stem lower-greek hiragana
    hiragana-iroha katakana katakana-iroha disclosure-open disclosure-closed
""")}

<%helpers:longhand name="list-style-image">
    use cssparser::{ToCss, Token};
    use std::fmt;
    use url::Url;
    use values::LocalToCss;

    #[derive(Debug, Clone, PartialEq, Eq, HeapSizeOf)]
    pub enum SpecifiedValue {
        None,
        Url(Url),
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                SpecifiedValue::None => dest.write_str("none"),
                SpecifiedValue::Url(ref url) => url.to_css(dest),
            }
        }
    }

    pub mod computed_value {
        use cssparser::{ToCss, Token};
        use std::fmt;
        use url::Url;
        use values::LocalToCss;

        #[derive(Debug, Clone, PartialEq, HeapSizeOf)]
        pub struct T(pub Option<Url>);

        impl ToCss for T {
            fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
                match self.0 {
                    None => dest.write_str("none"),
                    Some(ref url) => url.to_css(dest),
                }
            }
        }
    }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value<Cx: TContext>(&self, _context: &Cx) -> computed_value::T {
            match *self {
                SpecifiedValue::None => computed_value::T(None),
                SpecifiedValue::Url(ref url) => computed_value::T(Some(url.clone())),
            }
        }
    }

    pub fn parse(context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        if input.try(|input| input.expect_ident_matching("none")).is_ok() {
            Ok(SpecifiedValue::None)
        } else {
            Ok(SpecifiedValue::Url(context.parse_url(&*try!(input.expect_url()))))
        }
    }
    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(None)
    }
</%helpers:longhand>

<%helpers:longhand name="quotes">
    use std::borrow::Cow;
    use std::fmt;
    use values::computed::ComputedValueAsSpecified;

    use cssparser::{ToCss, Token};

    pub use self::computed_value::T as SpecifiedValue;

    pub mod computed_value {
        #[derive(Debug, Clone, PartialEq, HeapSizeOf)]
        pub struct T(pub Vec<(String,String)>);
    }

    impl ComputedValueAsSpecified for SpecifiedValue {}

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            let mut first = true;
            for pair in &self.0 {
                if !first {
                    try!(dest.write_str(" "));
                }
                first = false;
                try!(Token::QuotedString(Cow::from(&*pair.0)).to_css(dest));
                try!(dest.write_str(" "));
                try!(Token::QuotedString(Cow::from(&*pair.1)).to_css(dest));
            }
            Ok(())
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(vec![
            ("\u{201c}".to_owned(), "\u{201d}".to_owned()),
            ("\u{2018}".to_owned(), "\u{2019}".to_owned()),
        ])
    }

    pub fn parse(_: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue,()> {
        if input.try(|input| input.expect_ident_matching("none")).is_ok() {
            return Ok(SpecifiedValue(Vec::new()))
        }

        let mut quotes = Vec::new();
        loop {
            let first = match input.next() {
                Ok(Token::QuotedString(value)) => value.into_owned(),
                Ok(_) => return Err(()),
                Err(()) => break,
            };
            let second = match input.next() {
                Ok(Token::QuotedString(value)) => value.into_owned(),
                _ => return Err(()),
            };
            quotes.push((first, second))
        }
        if !quotes.is_empty() {
            Ok(SpecifiedValue(quotes))
        } else {
            Err(())
        }
    }
</%helpers:longhand>
