
# Rust Webscraper
 An example webscraper written in Rust. This program was built to scrape images from a particular website. I have removed the website URL from the code because the code still has bugs, and I don't want to encourage anyone to scrape a website without permission. This was built as a learning exercise to understand how to scrape websites using Rust.

 I also used this project to learn about logging in Rust. I used log4rs to keep track of the program's progress, and the program logs both to the console and to a file. The level 


 ### Usage
This program is not meant to be run in its current state, however, you can still build and run it if you like.

1. Install Rust
2. Clone the repository
3. Run `cargo build` to build the program
4. Run `cargo run` to run the program

### Bugs and limitations
This program has a few bugs and limitations. The main limitation is that it only scrapes images from a single website. Additionally, the cooldown timer is not working as expected. The program is supposed to wait for a few seconds before making another request to the website, but it seems to be making requests far too quickly. I have not been able to figure out why this is happening, but I know it's because I did not implement the cooldown mutex code correctly. There are probably other issues, but those are the only ones I can remember off the top of my head. This is an older project, and I have not worked on it in a while.

### License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

