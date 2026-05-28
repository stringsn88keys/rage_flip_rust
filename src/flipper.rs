const FORWARD: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=!@#$%^&*()_+ ";
const BACKSIDEDOWN: &str = " +‾()*⅋^%$#{@¡}=-068𝘓95ߤ↋↊⇂zʎxʍʌnʇsɹbdouɯʅʞɾᴉɥƃⅎǝpɔqɐZ⅄XϺɅՈꓕSꓤꝹԀONꟽ⅂ꓘᒋIH⅁ᖵƎᗡϽꓭ∀";

pub fn flip(word: &str) -> String {
    let upsidedown: String = BACKSIDEDOWN.chars().rev().collect();
    let upsidedown_chars: Vec<char> = upsidedown.chars().collect();

    let mut upsidedownmap = std::collections::HashMap::new();
    for (index, char) in FORWARD.chars().enumerate() {
        if let Some(&mapped) = upsidedown_chars.get(index) {
            upsidedownmap.insert(char, mapped);
        }
    }

    let mut flipped = String::new();
    for char in word.chars() {
        let mapped_char = upsidedownmap.get(&char).copied().unwrap_or(char);
        flipped.insert(0, mapped_char);
    }

    flipped
}

pub fn rage_flip(text: &str) -> String {
    format!("(ノಠ益ಠ)ノ彡┻{}┻", flip(text))
}

pub fn table_flip(text: &str) -> String {
    format!("(╯°□°)╯︵┻{} ┻", flip(text))
}
