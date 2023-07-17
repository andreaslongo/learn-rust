// Print the lyrics to the Christmas carol "The Twelve Days of Christmas".
fn main() {
    let gifts = [
        "",
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for day in 1..13 {
        println!("\nOn the {} day of Christmas my true love sent to me", day);
        let mut day_index = day;
        while day_index > 0 {
            println!("{} {}", day_index, gifts[day_index]);
            day_index -= 1;
        }
    }
}
