use anubis::{
    common::{Block, BlockInfo},
    db::*,
};

#[test]
fn test_block_db() {
    let test_db = AnubisDatabase::new(None);
    assert!(test_db.is_ok());

    let mut test_db = test_db.unwrap();
    let test_header = "block_name";
    let block = test_db.get_block(test_header);
    assert_eq!(block, None);

    let block = Block {
        info: BlockInfo {
            name: String::new(),
            template_name: String::new(),
        },
        content: vec![],
    };

    test_db
        .block_db
        .insert(test_header.to_string(), block.clone());

    let block_retrieved = test_db.get_block(test_header);
    assert_eq!(block_retrieved, Some(&block));
}
