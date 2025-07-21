fn main() {
    let gifts = [
        "patridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dacing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];

    let counters = [
        "a",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve"
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eight",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    println!();
    println!();
    println!();

    println!("On the {}, day of Christmas,", days[0]);
    println!("my true love sent to me");
    println!("{} {}", counters[0], gifts[0]);

    println!();

    for index in 1..=11 {
        println!("On the {}, day of Christmas,", days[index]);
        println!("my true love sent to me");

        for reverse_index in (0..=index).rev() {
            if reverse_index == 0 {
                println!("And {} {}", counters[reverse_index], gifts[reverse_index]);
            } else {
                println!("{} {}", counters[reverse_index], gifts[reverse_index]);
            }
        }

        println!();
    };

    println!();
    println!();
    println!();
}