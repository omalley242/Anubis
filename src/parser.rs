use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisParserInterface {
    fn new(path: Option<&PathBuf>) -> Self;
}

pub struct AnubisParser {}

impl AnubisParserInterface for AnubisParser {
    fn new(path: Option<&PathBuf>) -> Self {
        AnubisParser {}
    }
}

/*@[ BlockName | Template | Template2{templateArgs} ]

# Markdown Heading
{{Block Embed | Template3 }}
{ Block Reference }

*/
struct BlockHeader;
struct CommentStart;
struct CommentEnd;
struct LineComment;
struct AnubisSymbol;


struct Block; // BlockStart BlockInteral* BlockEnd
struct BlockStart; // comment (anubis start, header, blockcontent?)
struct BlockEnd; // comment ( blockcontent?, anubis end)
struct BlockInternal; // codecontent | comment(blockcontent)
/*
```
@*/
enum CommentedBlockContentList {
    Inline(LineComment, BlockContentList),
    Multiline(CommentStart, BlockContentList, CommentEnd),
}

enum BlockContentList {
    Join(BlockContent, Box<BlockContentList>),
    Base(BlockContent),
}

enum BlockContent {}
