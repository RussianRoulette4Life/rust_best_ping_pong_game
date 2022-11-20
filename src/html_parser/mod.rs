pub mod word_detection;
pub fn parse_html(html: &String, stopping_tag: &str) -> (Vec<String>, Vec<Vec<String>>){
    let mut start_iter:bool=false;
    let mut tags:String = String::new();
    let mut string_of_text = String::new();
    let mut count = 0;
    let mut all_properties: Vec<Vec<String>> = vec![];
    for character in html.chars(){
        if character == '<'{
                start_iter = true;
        }
        if start_iter {
            tags.push(character);
        } else {
            string_of_text.push(character);
        }
        if character == '>'{
            start_iter = false;
            if tags == format!("</{stopping_tag}>") {
                let string_final:String = String::from((string_of_text.replace('\n', " ")).trim());
                return (word_detection::count_and_disect_words(&string_final).2, all_properties)
            } else {
                let mut disacted_tags = word_detection::count_and_disect_words(&tags).2;
                let mut all_tile_properties: Vec<String> = vec![];
                // count+=1;
                // println!("{count}.{:#?}", &disacted_tags);
                // println!("{}", &disacted_tags[disacted_tags.len() - 1]);
                disacted_tags.remove(0);
                for mut property in disacted_tags {
                    let equal_sign_index = property.find('=').expect("FAILED TO GET n OUT OF ENUM Option(n) at html_parser.rs");
                    property = property.replace('>', "");
                    let property_final: String = String::from(property.get(equal_sign_index + 2..property.len() - 1).expect("yes"));
                    all_tile_properties.push(property_final)
                }
                all_properties.push(all_tile_properties);
                tags = String::new();
            }
        }
    }
    (vec![], vec![])
}
pub fn count_passed_in_tags(html: &String, tag_name: &str) -> u32 {
    let mut start_iter:bool = false;
    let stopping_tag = format!("</{tag_name}>");
    let mut tags = String::new();
    let mut num_of_tags:u32 = 0; 
    for character in html.chars(){
        if character == '<'{
                start_iter = true;
        }
        if start_iter {
            tags.push(character);
        }
        if character == '>' {
            if tags == stopping_tag {
                num_of_tags += 1;
            } else {
                tags = String::new()
            }
        }
    }
    return num_of_tags
}
pub fn return_html_up_to_a_tag(html:&String, tag_number:u32) -> String {
    let mut start_iter:bool=false;
    let mut tags:String = String::new();
    let mut string_of_text = String::new();
    let mut num_of_tags: u32 = 0;
    for character in html.chars(){
        if character == '<'{
                start_iter = true;
        }
        if start_iter {
            tags.push(character);
        }
        string_of_text.push(character);
        if character == '>'{
            start_iter = false;
            if tags == "</tr>"{
                num_of_tags+=1;
                if tag_number == num_of_tags {
                    return string_of_text
                } else {
                    string_of_text = String::new();
                }
            } else {
                tags = String::new();
            }
        }
    }
    String::from("must've been an error")
}
