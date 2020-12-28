use std::io;

fn main() {
    println!("Example #3-5-3. Displaying Lyrics of a Christmas Carol \"The Twelve Days of Christmas\"");

    println!("Press any key to view the lyrics...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");
    
    let lyrics = ["On the first day of Christmas my true love gave to me
    A partridge in a pear tree", "
    On the second day of Christmas my true love gave to me
    Two turtle doves and a partridge in a pear tree", "
    On the third day of Christmas my true love gave to me
    Three French hens, two turtle doves and a partridge in a pear tree" ,"
    On the fourth day of Christmas my true love gave to me
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the fifth day of Christmas my true love gave to me
    Five gold rings, four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the sixth day of Christmas my true love gave to me
    Six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the seventh day of Christmas my true love gave to me
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the eighth day of Christmas my true love gave to me
    Eight maids a milking, seven swans a swimming
    Six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the ninth day of Christmas
    Nine ladies dancing, eight maids a milking
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the tenth day of Christmas my true love gave to me
    Ten lords a leaping, nine ladies dancing, eight maids a milking
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the eleventh day of Christmas my true love gave to me
    Eleven pipers piping, ten lords a leaping
    Nine ladies dancing, eight maids a milking
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree", "
    On the twelfth day of Christmas my true love gave to me
    Twelve drummers drumming, eleven pipers piping
    Ten lords a leaping, nine ladies dancing, eight maids a milking
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree"];

    let mut x = 0;
    while x < lyrics.len() {
        println!("{}", lyrics[x]);
        x = x + 1;
    }
}