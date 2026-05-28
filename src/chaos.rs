use std::fs;
use std::path::PathBuf;
use rand::Rng;

const DEFAULT_CHAOS_LEVEL: u32 = 10;

fn chaos_level_file() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".chaos_level.txt")
}

pub fn read_chaos_level() -> u32 {
    let path = chaos_level_file();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            let level: u32 = content.trim().parse().unwrap_or(DEFAULT_CHAOS_LEVEL);
            return if level > 0 { level } else { DEFAULT_CHAOS_LEVEL };
        }
    }
    DEFAULT_CHAOS_LEVEL
}

pub fn write_chaos_level(level: u32) {
    let path = chaos_level_file();
    if let Err(e) = fs::write(&path, level.to_string()) {
        eprintln!("Error writing chaos level: {}", e);
    }
}

pub fn set_chaos_level(instruction: &str) -> Result<u32, String> {
    let current_level = read_chaos_level();
    let new_level = match instruction {
        "more" => current_level + 1,
        "less" => current_level.saturating_sub(1).max(1),
        _ => {
            let parsed: u32 = instruction.parse().map_err(|_| {
                "Error: Chaos level must be a positive number".to_string()
            })?;
            if parsed <= 0 {
                return Err("Error: Chaos level must be a positive number".to_string());
            }
            parsed
        }
    };
    write_chaos_level(new_level);
    Ok(new_level)
}

pub fn process(text: &str, chaos_level: Option<u32>) -> String {
    let level = chaos_level.unwrap_or_else(read_chaos_level);
    let mut result = String::new();
    
    for c in text.chars() {
        result.push(c);
        let count: u32 = rand::thread_rng().gen_range(1..=level);
        for _ in 0..count {
            let codepoint: u32 = rand::thread_rng().gen_range(0x300..=0x36F);
            if let Some(ch) = char::from_u32(codepoint) {
                result.push(ch);
            }
        }
    }
    
    result
}
