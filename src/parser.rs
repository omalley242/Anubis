use nom::{branch::alt, bytes::{complete::{is_not, tag, take_till, take_until, take_while}}, character::complete::{alphanumeric1, char, multispace0, one_of}, combinator::{opt, peek, value}, error::{self, ParseError}, multi::{many0, many1, separated_list0}, sequence::{self, separated_pair}, Parser};
use nom::sequence::delimited;
use nom::IResult;
use nom::character::anychar;
use nom::Err;
use crate::common::{AnubisError, LanguageConfig};




#[derive(Debug, Clone, PartialEq)]
pub struct BlockInfo<'a> {
    name: &'a str,                       //Page Name
    template_name: &'a str,             //Template to use when rendering
}

// Parsed Block of Markdown
#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownBlock<'a> {
    info: BlockInfo<'a>,
    content: Vec<BlockContent<'a>>,              //the markdown content within the block
}

// Parsed Block of Code
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock<'a> {
    info: BlockInfo<'a>,
    code_content: String,               //Code Block Content
}

// A wrapper to generalise block types
#[derive(Debug, Clone, PartialEq)]
pub enum Block<'a> {
    Markdown(MarkdownBlock<'a>),
    Code(CodeBlock<'a>),
}

// A liist of all the different types of data within a block
#[derive(Debug, Clone, PartialEq)]
pub enum BlockContent<'a>{
    Markdown(&'a str),    //Any normal content code or markdown
    Code(&'a str),      //Code string
    Link(&'a str),      //Link to anotherblock
    Embed(&'a str)      //Block to be rendered 
}


pub fn parse_string<'a>(
    file_contents: String,
    anubis_config: &'a LanguageConfig,
    language_config: &'a LanguageConfig
) -> Result<Vec<Block<'a>>, AnubisError> {
            
    println!("char:");

    return Ok(vec![])
}


/*
@[Page Name|Template Name]

    ## This is an example of a markdown block

    Template To Render
    {{ Block Name }}

    Template To Link
    { Block Name }
@
*/

//@[Code Block Name|Code Template]{


//@}

fn block_name<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    ws(is_not("|")).parse(i)
}

fn template_name<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    ws(take_until("]")).parse(i)
}

fn block_header<'a>(i: &'a str) -> IResult<&'a str, BlockInfo<'a>> {
    let (not_matched, (block_name, template_name)) = delimited(
    char('['),
    separated_pair(block_name, char('|'), template_name),
    char(']')
    ).parse(i)?;

    Ok((not_matched, BlockInfo {name: block_name, template_name: template_name}))    
}

fn block_link<'a>(i: &'a str) -> IResult<&'a str, BlockContent> {
    let (not_matched, link) = delimited(
        char('{'),
        is_not("{}"),
        char('}')
    ).parse(i)?;

    Ok((not_matched, BlockContent::Link(link)))
}

fn block_embed<'a>(i: &'a str) -> IResult<&'a str, BlockContent> {
    let (not_matched, embed_string) = delimited(
        tag("{{"),
        take_until("}}"),
        tag("}}")
    ).parse(i)?;

    Ok((not_matched, BlockContent::Embed(embed_string)))
}

fn block_content<'a>(i: &'a str) -> IResult<&'a str, Vec<BlockContent>> {
    many0(
        alt((
            block_embed,
            block_link,
            markdown
        ))
    ).parse(i)
}

fn markdown<'a>(i: &'a str) -> IResult<&'a str, BlockContent> {
    let (not_matched, markdown_string) = is_not("{@").parse(i)?;
    Ok((not_matched, BlockContent::Markdown(markdown_string)))
}

fn markdown_block<'a>(i: &'a str, config: LanguageConfig) -> IResult<&'a str, MarkdownBlock> {
    let (not_matched, (block_info, block_content)) = delimited(
        tag(config.multiline_start.as_str()),
        (ws(block_header), block_content),
        tag(config.multiline_end.as_str()),
    ).parse(i)?;

    Ok((not_matched, MarkdownBlock {info: block_info, content: block_content}))
}

//Remove leading and trailing whitespace
pub fn ws<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}
   

#[cfg(test)]
mod tests {
   use super::*;
   use nom::error::Error;
use test_case::test_case;

   #[test_case("[Markdown Block Name|Template Name]", Ok(("", BlockInfo { name: "Markdown Block Name", template_name: "Template Name"})); "Basic Test")]
   #[test_case("[|]", Ok(("", BlockInfo { name: "", template_name: ""})); "Empty Test")]
   #[test_case("[Markdown Block|]", Ok(("", BlockInfo { name: "Markdown Block", template_name: ""})); "Empty Template")]
   #[test_case("[|Template Name]", Ok(("", BlockInfo { name: "", template_name: "Template Name"})); "Empty Block Name")]
   fn block_name_test(input: &str, output: IResult<&str, BlockInfo>) {
       assert_eq!(block_header.parse(input), output);
   }

   #[test_case("{{Block Name}}", Ok(("", BlockContent::Embed("Block Name"))); "Embed Basic Test")]
   #[test_case("{ Block Name }", Err(Err::Error(Error::new("{ Block Name }", error::ErrorKind::Tag))); "Embed Fail on Single ")]
   fn block_embed_test(input: &str, output: IResult<&str, BlockContent>) {
       assert_eq!(block_embed.parse(input), output);
   }

   #[test_case("{Block Name}", Ok(("", BlockContent::Link("Block Name") )); "Link Basic Test")]
   #[test_case("{{ Block Name }}", Err(Err::Error(Error::new("{ Block Name }}", error::ErrorKind::IsNot))); "Link Fail on Double ")]
   fn block_link_test(input: &str, output: IResult<&str, BlockContent>) {
       assert_eq!(block_link.parse(input), output);
   }

   #[test_case("# Heading", Ok(("", BlockContent::Markdown("# Heading") )); "Markdown Basic Test")]
   #[test_case("basic markdown {{ Block Name }}", Ok(("{{ Block Name }}", BlockContent::Markdown("basic markdown ") )); "Markdown Stop Test 1")]
   #[test_case("basic markdown @ ignored", Ok(("@ ignored", BlockContent::Markdown("basic markdown ") )); "Markdown Stop Test 2")]
   #[test_case("{{ Block Name }}", Err(Err::Error(Error::new("{{ Block Name }}", error::ErrorKind::IsNot))); "Empty Return Fail Test")]
   fn markdown_test(input: &str, output: IResult<&str, BlockContent>) {
       assert_eq!(markdown.parse(input), output);
   }

   #[test_case("# Heading {Block Name} {{ Block Name }}", Ok(("", vec![BlockContent::Markdown("# Heading"), BlockContent::Link("Block Name"), BlockContent::Embed(" Block Name ")])); "Block Contenet Basic Test")]
   #[test_case("", Err(Err::Error(Error::new("", error::ErrorKind::IsNot))); "Empty Return Fail Test")]
   fn block_content_test(input: &str, output: IResult<&str, Vec<BlockContent>>) {
       assert_eq!(block_content.parse(input), output);
   }


   #[test_case(
    "@[Block Name|Template Name] # Heading {Block Name}{{ Block Name }}@",
    Ok(
        (
            "",
            MarkdownBlock {
                info: BlockInfo {name: "Block Name", template_name: "Template Name"},
                content: vec![BlockContent::Markdown("# Heading "), BlockContent::Link("Block Name"), BlockContent::Embed(" Block Name ")]
            }
       )
    ); "Markdown Block Basic Test")]

   #[test_case("", Err(Err::Error(Error::new("", error::ErrorKind::Tag))); "Empty Return Fail Test")]
   fn markdown_block_test(input: &str, output: IResult<&str, MarkdownBlock>) {
       let config = LanguageConfig {
        line_start: "@".to_string(),
        multiline_start: "@".to_string(),
        multiline_end: "@".to_string()
       };
       assert_eq!(markdown_block(input, config), output);
   }

}
