pub fn count_and_disect_words (string_sentence: &String) -> (i32, i32, Vec<String>){
    let mut amount_of_words = 0;
    let mut index_last = 0;
    let mut list_of_words: Vec<String> = [].to_vec();
    // made to prevent 2 spaces count as a separate word
    let mut last_character_index = 0;
    // for fun :)
    // if string_sentence == "" {
    //     println!("why?")
    // }
    // prints ALL characters
    // for character in string_sentence.chars(){
    //     println!("{}", character)
    // }
    let mut detected_word: String = String::new();
    // word detection, that's it
    for (i, character) in string_sentence.chars().into_iter().enumerate() {
        if (character != ' ') && (character != '\n') && (character != '\t'){
            detected_word.push(character);
        }
        last_character_index = if (character != ' ') && (character != '\n') && (character != '-') {
            i
        } else {
            last_character_index
        };
        if ((character == ' ') /*|| (character == ',')*/)&& (i <= last_character_index + 1) {
            amount_of_words = amount_of_words + 1;
            // println!("{}. {}",amount_of_words, detected_word);
            list_of_words.push(detected_word);
            detected_word = String::new();
        }
        index_last = i;
    }
    // print what cant return, return what can
    // println!("{}. {}",&amount_of_words + 1, &detected_word);
    list_of_words.push(detected_word);
 (amount_of_words+1, index_last.try_into().unwrap(), list_of_words)
}
