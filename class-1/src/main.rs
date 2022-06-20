use std::io;
use regex::Regex;

/*Description: check if sub_arr is child of org_arr
* @sub_arr: child array
* @org_arr: parent array
*/
fn bai1_kiemtramang(sub_arr: &[i32], org_arr: &[i32]) -> bool {
    //Get length of sub_arr array;
    let sub_arr_length: usize = sub_arr.len();
    let mut result = false;
    //In org_arr array, loop from first position to org_arr.len - sub_arr.len
    for (i, _element) in org_arr[0..(org_arr.len() - sub_arr_length)].iter().enumerate() {
        //Compare sub_arr with splice of org_arr
        //If true, return true result
        //If false, go to next position of org_arr
        if sub_arr == &org_arr[i..i + sub_arr_length]  {
            result = true;
            return result;
        }
        //println!("{}",sub_arr == &org_arr[i..i + sub_arr_length]);
    }

    return result;
}

fn main() {
    //-------------Class 1, Bai tap 1---------------
    //Parent array
    println!("Bai tap 1:");
    let org_arr = [1,2,3,5,6,8,10,11];
    println!("Parent array : {:?}", org_arr);
    //Child array
    let sub_arr = [6,8,10];
    println!("Child array : {:?}", sub_arr);
    //Print check child result
    println!("Is sub_arr a child of org_arr: {}",bai1_kiemtramang(&sub_arr, &org_arr));

    //-------------Class 1, Bai tap 2---------------
    println!("Bai tap 2:");
    //Phrase
    let str_phrase = "This is a regular paragraph with the default style of Normal.This is a regular paragraph with the default style of Normal.This is a regular paragraph with the default style of Normal.This is a regular paragraph with the default style of Normal.This is a regular paragraph with the default style of Normal.";
    println!("String phrase : {}", str_phrase);
    let mut keyword = String::new();

    //Input phrase by keyboard
    println!("Enter the keyword want to count: ");
    io::stdin()
        .read_line(&mut keyword)
        .expect("Cannot read input");

    //Remove /n at the end of phrase
    keyword.pop();
    println!("keyword: {}", &keyword);

    //Count keyword in phrase
    let phrase_count = str_phrase.matches(&keyword).count();
    println!("Count : {}", phrase_count);

    //Count keyword in phrase case-insensitive with regex
    let regex_format = Regex::new(format!(r"(?i:{})", keyword).as_str()).unwrap();
    let case_insensitive_phrase_count = regex_format.captures_iter(str_phrase).count();
    println!("Case-insensitvie count : {}", case_insensitive_phrase_count);
}