use std::{fs, io::Read};

pub struct FileLineIterator {
    file: Option<fs::File>,
    buf: String,
    newline: Option<usize>,
}

impl FileLineIterator {
    pub fn new(path: &str) -> Self {
        FileLineIterator {
            file: Some(fs::File::open(path).unwrap()),
            buf: String::new(),
            newline: None,
        }
    }
}

impl Iterator for FileLineIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let file = self.file.as_mut()?;
        while self.newline.is_none() {
            {
                let len = (self.buf.len() + 0x6ffusize) & !0x3ffusize;
                if self.buf.capacity() < len {
                    self.buf.reserve(len - self.buf.len());
                }
            }
            unsafe {
                let vec = self.buf.as_mut_vec();
                let old_len = vec.len();
                let spare = &mut *(vec.spare_capacity_mut() as *mut _ as *mut [u8]);
                let read = file.read(spare).unwrap();
                if read == 0 {
                    self.file = None;
                    if old_len != 0 {
                        return Some(std::mem::take(&mut self.buf));
                    } else {
                        return None;
                    }
                }
                let new_len = old_len + read;
                let slice = std::str::from_utf8(&mut spare[..read]);
                let slice = match slice {
                    Ok(ok) => ok,
                    Err(_err) => panic!("File contains non-valid UTF-8"),
                };
                self.newline = slice.find('\n').map(|nl| nl + old_len);
                vec.set_len(new_len);
            }
        }
        let nl = unsafe{ self.newline.unwrap_unchecked() };
        let mut line = self.buf.split_off(nl);
        std::mem::swap(&mut line, &mut self.buf);
        self.buf.remove(0);
        self.newline = self.buf.find('\n');
        if line.ends_with('\r') {
            line.pop();
        }
        Some(line)
    }
}
