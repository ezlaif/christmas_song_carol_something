/**
Prints the lyrics of an old christmas song
On the first day of Christmas
my true love sent to me
a partridge in a pear tree.

On the second day of Christmas,
my true love sent to me
Two turtle doves,
and a partridge in a pear tree.

On the third day of Christmas,
my true love sent to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fourth day of Christmas,
my true love sent to me
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
...
*/
fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
        "eleventh", "twelfth"
    ];
    const PRESENTS: [&str;12] = [
        "a partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds",
        "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking",
        "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"
    ];
    let mut index: i8 = 0;
    while index < (DAYS.len()) {
        println!("On the {} day of Christmas my true love sent to me", DAYS[index]);
        for inner_index in (0..(index + 1)).rev() {
            println!("{}", PRESENTS[inner_index]);
        }
        println!("\n");
        index += 1;
    }
}