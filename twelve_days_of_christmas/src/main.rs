fn main() {
    const LYRICS: [&str; 12] = ["drummers drumming", "pipers piping", "lords a-leaping", "ladies dancing", "maids a-milking", "swans a-swimming", "geese a-laying", "gold rings",
    "calling birds", "French hens", "turtle doves", "partridge in a pear tree"];

    for current_paragraph in 1..=12 {
        println!("\nOn the {current_paragraph} day of Christmas,
my true love sent to me");
        for counter in 1..=current_paragraph {
            println!("{} {}", current_paragraph - counter + 1, LYRICS[LYRICS.len() + counter -1 - current_paragraph])
        }
    }
}
