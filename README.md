# A DNS Server in Rust
This is my repo which follows the DNS server guide found [here](https://github.com/EmilHernvall/dnsguide) by [Emil Hernvall](https://github.com/EmilHernvall).

**THIS CODE IS JUST ME LEARNING RUST. IT'S NOWHERE NEAR PRODUCTION WORTHY. USE AT YOUR OWN RISK, YADDA YADDA ETC.**

The original guide puts all the code into one `main.rs` file, so I refactored it into the rust mod system hiearchy.

You can check out their [hermes](https://github.com/EmilHernvall/hermes) repo, if you want to see what a polished finished result looks like!

# License 📃
The original repo doesn't specify a license, so I'm using the [MIT license](LICENSE); which the `hermes` repo also uses.

# Building 🛠
You'll need `cargo` and a stable rust toolchain, both easily obtainable by going to [rustup.rs](https://rustup.rs/) and following the instructions there.

Once you have `cargo` and the rust toolchain installed, then simply checkout this repository, and in that same directory run `cargo build`. You'll find the program under `./target/debug/`.

# Running 🦀
You can simply execute `cargo run` in the main repo directory after you clone it.
This program doesn't terminate. It loops forever, so you'll need to either CTRL+C or send a SIGHUP to kill it.