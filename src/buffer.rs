use std::fs::File;
use std::vec::Vec;
use diff;

pub enum BufferFile {
    File(File),
    NoneGiven,
}

pub struct FileBuffer {
    file: BufferFile,
    diff: diff::Diff,
}

impl FileBuffer {
    pub fn new(diff: diff::Diff, file: BufferFile) -> FileBuffer {
        FileBuffer{
            file: file,
            diff: diff,
        }
    }

    pub fn lines(&self) -> Result<Vec<String>, String> {
        let mut lines: Vec<String> = Vec::new();
        match self.file {
            BufferFile::File(_) => Err("Not using files yet".to_string()),
            BufferFile::NoneGiven => {
                for entry in self.diff.entries.clone() {
                    match entry.difference {
                        diff::Difference::Minus => {
                            if (lines.len() as i32) < entry.line_num {
                                return Err("Diff held minus entry for line that didn't exist".to_string())
                            }
                            lines.remove((entry.line_num - 1) as usize);
                        },
                        diff::Difference::Plus => {
                            if (lines.len() as i32) < entry.line_num + 1 {
                                return Err("Diff entry for line that doesn't exist".to_string())
                            }
                            lines.insert((entry.line_num - 1) as usize, entry.line.clone());
                        },
                    }
                }
                Ok(lines)
            }
        }
    }
}
