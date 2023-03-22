use std::{collections::HashMap, fs, path::PathBuf};

pub mod battery;

fn parse_uevent(uevent: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in uevent.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        let key = parts[0].trim().to_string();
        let val = parts[1].trim().to_string();
        map.insert(key, val);
    }
    map
}

#[derive(Debug)]
struct SysfsObject {
    uevent_path: PathBuf,
    uevent_data: HashMap<String, String>,
}

impl SysfsObject {
    fn new(path: impl Into<PathBuf>) -> Self {
        let mut path_buf: PathBuf = path.into();
        path_buf = path_buf.join("uevent");
        let mut obj = Self {
            uevent_path: path_buf,
            uevent_data: HashMap::new(),
        };
        obj.refresh();
        obj
    }

    pub fn refresh(&mut self) {
        let Ok(data_str) = fs::read_to_string(&self.uevent_path) else {
            return
        };

        self.uevent_data = parse_uevent(data_str);
    }

    pub fn value(&self, name: &str) -> Option<&'_ String> {
        self.uevent_data.get(name)
    }
}
