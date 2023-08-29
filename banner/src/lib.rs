use std::collections::HashMap;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", l_h.chars().next().unwrap()),
            long_hand: format!("--{}", l_h),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, String>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        if let Some(func) = self.flags.get(&flag) {
            match func(argv[0], argv[1]) {
                Ok(result) => result,
                Err(err) => err,
            }
        } else {
            "flag not found".to_string()
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, String> {
    let a: f32 = a.parse().map_err(|_| "invalid float literal".to_string())?;
    let b: f32 = b.parse().map_err(|_| "invalid float literal".to_string())?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, String> {
    let a: f32 = a.parse().map_err(|_| "invalid float literal".to_string())?;
    let b: f32 = b.parse().map_err(|_| "invalid float literal".to_string())?;
    Ok((a % b).to_string())
}