pub struct Substitution {
    pub name: &'static str,
    pub text: &'static str,
}

pub const SUBSTITUTIONS: &[Substitution] = &[
    Substitution { name: "bugeyes", text: "(⊙_◎)" },
    Substitution { name: "cmd-", text: "⌘-" },
    Substitution { name: "cmd", text: "⌘" },
    Substitution { name: "cntl", text: "⌃" },
    Substitution { name: "disapproval", text: "ಠ_ಠ" },
    Substitution { name: "dogshrug", text: "¯\\_🐶_/¯" },
    Substitution { name: "duck-flip", text: "(╯°□°)╯︵ ┻(duckflip)┻" },
    Substitution { name: "facepalm", text: "(－‸ლ)" },
    Substitution { name: "flip", text: "(╯°□°)╯︵ ┻━┻" },
    Substitution { name: "fu", text: "t(-__-t)" },
    Substitution { name: "heresatable", text: "┬─┬﻿ ノ( ゜-゜ノ)" },
    Substitution { name: "javaflip", text: "(╯°□°)╯︵ ┻ɐʌɐɾ┻" },
    Substitution { name: "kungfuhamster", text: "    ()__()\n    / o o\\   ;\n   |'=Y=';-/\n   { \\  / }\n    mmm mmm   " },
    Substitution { name: "noevil", text: "🙈🙉🙊" },
    Substitution { name: "omw", text: "On my way!" },
    Substitution { name: "optn", text: "⌥" },
    Substitution { name: "rage", text: "ಠ益ಠ" },
    Substitution { name: "rageflip", text: "(ノಠ益ಠ)ノ彡┻━┻" },
    Substitution { name: "rock", text: "\\m/ (>_<) \\m/" },
    Substitution { name: "shft", text: "⇧" },
    Substitution { name: "shift", text: "⇧" },
    Substitution { name: "shrug", text: "¯\\_(ツ)_/¯" },
    Substitution { name: "shrugtable", text: "┻━┻ ︵ ¯\\(ツ)/¯ ︵ ┻━┻" },
    Substitution { name: "unsee", text: "♨_♨" },
    Substitution { name: "yuno", text: "ლ(ಠ益ಠლ)" },
];

pub fn process(substitution_name: &str) -> Option<&'static str> {
    let name = substitution_name.to_lowercase();
    SUBSTITUTIONS
        .iter()
        .find(|s| s.name == name)
        .map(|s| s.text)
}

pub fn list_substitutions() -> String {
    let mut output = vec!["Available text substitutions:".to_string()];
    for sub in SUBSTITUTIONS {
        let display_text = if sub.text.len() > 50 {
            format!("{}...", &sub.text[..50])
        } else {
            sub.text.to_string()
        };
        output.push(format!(
            "  {:15} - {}",
            sub.name, display_text
        ));
    }
    output.join("\n")
}

pub fn substitution_exists(name: &str) -> bool {
    let name = name.to_lowercase();
    SUBSTITUTIONS.iter().any(|s| s.name == name)
}

pub fn substitution_names() -> Vec<&'static str> {
    SUBSTITUTIONS.iter().map(|s| s.name).collect()
}
