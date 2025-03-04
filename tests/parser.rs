// ======= Header Tests ========

/*[BlockName]*/

#[test]
fn base_header() {}

/*[BlockName|Template]*/

#[test]
fn header_template() {}

/*[BlockName|Template|Template2]*/

#[test]
fn header_multi_template() {}

/*[BlockName|Template{arg1: True}]*/

#[test]
fn header_template_args() {}

/*[BlockName|Template{arg1: True}|Template2{arg2: True}]*/

#[test]
fn header_multi_template_args() {}

// ======= Header Test End =======
// ======= Block Content Tests ========

/*# Markdown Heading*/

#[test]
fn markdown_content() {}

/*{BlockName}*/

#[test]
fn link_content() {}

/*{{BlockName}}*/

#[test]
fn embed_content() {}

#[test]
fn code_content() {}

/*{{BlockName|Template}}*/

#[test]
fn embed_content_template() {}

/*{{BlockName|Template{args: True}}}*/

#[test]
fn embed_content_template_args() {}

// ======= Block Content Tests End =======
