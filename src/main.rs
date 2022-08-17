/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and 
“ay” is added, so “first” becomes “irst-fay.” 
Words that start with a vowel have “hay” added to the end instead 
(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/
/* Rust book assignment 5 */
fn main(){
    // String-to-pig_latin string converter
    let input = String::from("jason");
    let mut converted = String::new();
    let mut ned = Vec::new();
    push_to_array(&input,  &mut ned);
    //println!("{}",is_consonant(&mut ned));
    //mod_string(&mut ned, &mut converted);
    //println!("{}{:?}", converted,ned);
    pig_latin(&mut converted,  &mut ned)



   

}
fn is_consonant(arr : &mut Vec<String>) -> bool{
    let vowels = ['a','e','i','o','u','A','E','I','O','U'];
    for i in vowels{
        if i.to_string() ==  arr[0]{
            return false
        }
    }
    return true
}
fn push_to_array(str: &String, arr: &mut Vec<String>){
    for i in str.chars(){
        arr.push(i.to_string())
    }
}
fn mod_string(arr:&mut Vec<String>, str: &mut String){
    for i in 1..=arr.len() - 1{
        str.push_str(&arr[i]);
    }
}
fn pig_latin(str: &mut String, arr:&mut Vec<String>){
    let mut temp = String::new(); 
    if is_consonant(arr){
        mod_string(arr, str);
        println!("{}",str.to_owned() +"-"+ &arr[0] +"ay")
    }else{
      
        for i in arr{
            temp.push_str(i)
        }
         println!("{}", temp.to_owned() + "-" + "hay");
    }
  
}