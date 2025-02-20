use crate::common::{collect_all_files, read_file, remove_ignored_files, Anubis};
use crate::parser_core::file_parser;
use nom::Parser;
use std::path::Path;
use std::{collections::HashSet, path::PathBuf};

pub trait AnubisParser {
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn parse_files(
        &mut self,
        file_list: HashSet<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn parse_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

impl AnubisParser for Anubis {
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_list = collect_all_files();
        let ignore_glob = self.config.generate_ignore_glob()?;
        remove_ignored_files(&mut file_list, ignore_glob);
        self.parse_files(file_list)?;
        self.database.save("./anubis.db")?;
        Ok(())
    }

    fn parse_files(
        &mut self,
        file_list: HashSet<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        file_list.iter().try_for_each(|file| self.parse_file(file))
    }

    fn parse_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let lang_config = self.config.get_language_config(file_path)?;
        let file_contents = read_file(file_path)?;
        if let Ok(result) = file_parser(lang_config).parse(&file_contents) {
            self.database.insert_blocks(result.1, lang_config);
        }
        Ok(())
    }
}
