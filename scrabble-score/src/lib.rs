/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut sum = 0;
	//let words = word.replace("ß", "");
	for letter in word.to_lowercase().chars() {
		sum += match letter {
			'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
            'd'|'g' => 2,
            'b'|'c'|'m'|'p' => 3,
            'f'|'h'|'v'|'w'|'y' => 4,
            'k' => 5,
            'j'|'x' => 8,
            'q'|'z' => 10,
            _ => 0,
		}
	}
	
	sum
}
