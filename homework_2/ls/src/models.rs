pub struct FileModel {
    pub name: String,
    pub is_hidden: bool,
    pub is_directory: bool,
    pub size: FileSize,
    pub is_ro: bool,
}

pub struct FileSize {
    pub size: u64
}

impl FileSize {
    pub fn to_raw_str(&self) -> String {
        format!("{} B", self.size)
    }

    pub fn to_human_str(&self) -> String {
        if self.size < 1024 {
            self.to_raw_str()
        } else {
            let mut i: u8 = 0;
            let mut human_size: f64 = self.size as f64;
            loop {
                human_size /= 1024.0;
                i += 1;

                if human_size < 1024.0 {
                    break;
                }
            }
            format!("{:.2} {}", human_size, FileSize::get_size_label(i))
        }
    }

    fn get_size_label(size_order: u8) -> String {
        String::from(
            match size_order {
                0 => "B",
                1 => "KiB",
                2 => "MiB",
                3 => "GiB",
                4 => "TiB",
                5 => "EiB",
                _ => "B"
            }
        )
    }
}
