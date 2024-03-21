
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// function to print the lyrics of the song
fn print_lyrics (day: &str, gift: &str) {
    println!("On the {} day of Christmas my true love sent to me", day);
    println!("{}", gift);
    println!("")
}

fn main() {
    print_lyrics("first", "A partridge in a pear tree");
    print_lyrics("second", "Two turtle doves, and");
    print_lyrics("third", "Three French hens");
    print_lyrics("fourth", "Four calling birds");
    print_lyrics("fifth", "Five golden rings");
    print_lyrics("sixth", "Six geese a-laying");
    print_lyrics("seventh", "Seven swans a-swimming");
    print_lyrics("eighth", "Eight maids a-milking");
    print_lyrics("ninth", "Nine ladies dancing");
    print_lyrics("tenth", "Ten lords a-leaping");
    print_lyrics("eleventh", "Eleven pipers piping");
    print_lyrics("twelfth", "Twelve drummers drumming");
}
