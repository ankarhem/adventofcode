use std::str::FromStr;
use winnow::ascii::digit1;
use winnow::error::{StrContext, StrContextValue};
use winnow::{PResult, Parser};

pub fn number<F>(input: &mut &str) -> PResult<F>
where
    F: FromStr, <F as FromStr>::Err: std::error::Error, <F as FromStr>::Err: Send, <F as FromStr>::Err: Sync, <F as FromStr>::Err: 'static
{
    digit1
        .try_map(|s: &str| s.parse::<F>())
        .context(StrContext::Label("digit"))
        .context(StrContext::Expected(StrContextValue::Description(
            "expected a number",
        )))
        .parse_next(input)
}