use crate::common::{Block, BlockContent, BlockInfo};
use crate::config::LanguageConfig;
use nom::character::anychar;
use nom::sequence::delimited;
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{char, multispace0},
    combinator::{peek, value},
    error::{self, ParseError},
    multi::{many1, many_till},
    sequence::{pair, separated_pair},
    Parser,
};

fn block_name(i: &str) -> IResult<&str, &str> {
    ws(is_not("|")).parse(i)
}

fn template_name(i: &str) -> IResult<&str, &str> {
    ws(is_not("]")).parse(i)
}

fn block_header(i: &str) -> IResult<&str, BlockInfo> {
    let (not_matched, (block_name, template_name)) = delimited(
        char('['),
        separated_pair(block_name, char('|'), template_name),
        char(']'),
    )
    .parse(i)?;

    Ok((
        not_matched,
        BlockInfo {
            name: block_name.to_string(),
            template_name: template_name.to_string(),
        },
    ))
}

fn block_link(i: &str) -> IResult<&str, BlockContent> {
    let (not_matched, link) = delimited(char('{'), is_not("{}"), char('}')).parse(i)?;

    Ok((not_matched, BlockContent::Link(link.to_string())))
}

fn block_embed(i: &str) -> IResult<&str, BlockContent> {
    let (not_matched, embed_string) = delimited(tag("{{"), take_until("}}"), tag("}}")).parse(i)?;

    Ok((not_matched, BlockContent::Embed(embed_string.to_string())))
}

fn markdown(
    language_config: &LanguageConfig,
) -> impl Parser<&str, Output = BlockContent, Error = error::Error<&str>> {
    many_till(
        anychar,
        peek(alt((
            tag(language_config.anubis_character.as_str()),
            tag(language_config.multiline_end.as_str()),
            tag("{"),
        ))),
    )
    .map_res(|(matched_chars, _not_matched)| {
        let matched_string: String = matched_chars.iter().copied().collect();
        if matched_string.is_empty() {
            Err(())
        } else {
            Ok(BlockContent::Markdown(matched_string))
        }
    })
}

fn code(
    language_config: &LanguageConfig,
) -> impl Parser<&str, Output = BlockContent, Error = error::Error<&str>> {
    delimited(
        tag(language_config.multiline_end.as_str()),
        take_until(language_config.multiline_start.as_str()),
        tag(language_config.multiline_start.as_str()),
    )
    .map(|code_string: &str| BlockContent::Code(code_string.to_string()))
}

fn block_content(
    language_config: &LanguageConfig,
) -> impl Parser<&str, Output = BlockContent, Error = error::Error<&str>> {
    alt((
        block_link,
        block_embed,
        code(language_config),
        markdown(language_config),
    ))
}

fn block(
    language_config: &LanguageConfig,
) -> impl Parser<&str, Output = Block, Error = error::Error<&str>> {
    delimited(
        ws(tag(language_config.anubis_character.as_str())),
        pair(block_header, many1(block_content(language_config))),
        tag(language_config.anubis_character.as_str()),
    )
    .map(|(header, content)| Block {
        info: header,
        content,
    })
}

pub fn file_parser(
    language_config: &LanguageConfig,
) -> impl Parser<&str, Output = Vec<Block>, Error = error::Error<&str>> {
    many1(alt((
        block(language_config).map(Option::Some),
        value(
            None,
            take_until(language_config.anubis_character.as_str()).map_res(
                |matched_string: &str| {
                    if matched_string.is_empty() {
                        Err::<Vec<Block>, ()>(())
                    } else {
                        Ok(vec![])
                    }
                },
            ),
        ),
    )))
    .map(|result| result.iter().filter_map(|block| block.clone()).collect())
}

pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}
