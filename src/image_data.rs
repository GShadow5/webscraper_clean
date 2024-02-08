use chrono::{DateTime, Local};

pub struct ImageBox {
    pub url: String,
    pub date: DateTime<Local>,
    pub title: String,
    pub hash: String,
    pub blacklisted: bool,
}

impl std::fmt::Display for ImageBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{\n\turl: {}\n\tdate: {}\n\ttitle: {}\n\thash: {}\n\tblacklisted: {}\n}}",
            self.url,
            self.date.date_naive(),
            self.title,
            self.hash,
            self.blacklisted
        )
    }
}

impl PartialEq for ImageBox {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Clone for ImageBox {
    fn clone(&self) -> Self {
        ImageBox {
            url: self.url.clone(),
            date: self.date.clone(),
            title: self.title.clone(),
            hash: self.hash.clone(),
            blacklisted: self.blacklisted.clone(),
        }
    }
}
