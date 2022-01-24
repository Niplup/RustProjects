fn main() {
    let phrase = String::from("Hey there bud");
    let phraset = String::from("bud");

    println!("{}", first_word_no_slice(&phrase));
    println!("{}", first_word_no_slice(&phraset));

    println!("{}", first_word_slice(&phrase));
    println!("{}", first_word_slice(&phraset));
}

fn first_word_slice(phrase: &str) -> &str {
    for (i,c) in phrase.chars().enumerate() {
        //The [..i] notation means from 0 to i
        if c == ' ' {return &phrase[..i];}
    }
    //Returns entirety of slice [..]
    &phrase[..]
}

fn first_word_no_slice(phrase: &String) -> String {
    let mut ret = String::new();
    for c in phrase.chars() {
        if c == ' ' {break;}
        ret.push(c);
    }
    ret
}


