use std::io;
use std::io::Write;

#[cfg(feature = "use_http")]
use reqwest::blocking::get;

#[cfg(feature = "use_http")]
use std::io::Read;

const BITMAP: &str = "\
....................................................................
   **************   *  *** **  *      ******************************
  ********************* ** ** *  * ****************************** *
 **      *****************       ******************************
          *************          **  * **** ** ************** *
           *********            *******   **************** * *
            ********           ***************************  *
   *        * **** ***         *************** ******  ** *
               ****  *         ***************   *** ***  *
                 ******         *************    **   **  *
                 ********        *************    *  ** ***
                   ********         ********          * *** ****
                   *********         ******  *        **** ** * **
                   *********         ****** * *           *** *   *
                     ******          ***** **             *****   *
                     *****            **** *            ********
                    *****             ****              *********
                    ****              **                 *******   *
                    ***                                       *    *
                    **     *                    *
....................................................................\
";

fn main() {
    println!("Bitmap Message, by Al Sweigart al@inventwithpython.com");
    println!("Enter the message to display with the bitmap.");

    let msg = read_stdin(None);

    if msg == "" {
        println!("No message - exiting.");
        return;
    }

    for line in BITMAP.split("\n") {
        for (i, bit) in line.chars().enumerate() {
            if bit == ' ' {
                print!(" ")
            } else {
                let current_char = msg
                    .chars()
                    .nth(i % msg.len())
                    .expect("No char at location.");

                print!("{}", current_char);
            }
        }
        println!()
    }
}

fn read_stdin(msg: Option<&str>) -> String {
    match msg {
        Some(s) => print!("{}", s),
        None => print!("> "),
    }

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed input read line.");

    input = input.trim().to_string();
    input
}

#[cfg(feature = "use_http")]
fn read_bitmap_from_url() -> String {
    const BITMAP_URL: &str = "https://inventwithpython.com/bitmapworld.txt";
    let mut response = get(BITMAP_URL).expect("Could not read URL.");
    let mut content = String::new();

    response
        .read_to_string(&mut content)
        .expect("Failed to read response text.");

    content
}
