![https://crates.io/crates/timeln](https://img.shields.io/crates/v/timeln.svg)

# timeln

*annotate your stdout with time information*

Do you like debugging with a copious amount of print statements? Do you often find yourself adding timers around sections of your code to see how long they take to execute? Do you wish you could see the time elapsed between key patterns of your program's output? If so, then timeln is for you!

![timeln example](https://raw.githubusercontent.com/EthanJamesLew/timeln/main/docs/img/fib_example.png)

Timeln is a command-line utility written in Rust that time tracks patterns appearing in your scripts' print statements, creating "timeline" of your command line applications and scripts.

The utility reads from the standard input (stdin), line by line, and measures the time elapsed from the start of the program as well as the delta time between subsequent lines. This makes it perfect for timing your scripts and programs, or for (hacky) profiling where time-consuming operations are taking place.

Features:
* **Line-by-Line Timing**: Reads data line-by-line from standard input (stdin) and calculates/display the time elapsed and the delta time between subsequent lines.
* **Regex Matching**: Allows timing between regex matches instead of lines, adding the ability to focus on specific patterns in the incoming data stream.
* **Colorized Output**: Adds an option to colorize output, with time and delta time stamps in green for enhanced readability.
* **Regex Highlighting**: When colorization is enabled, regex matches are highlighted in red for easy identification.
The name "Timeln" is a pun combining the concepts of "println" and "timeline", reflecting its function of printing time-stamped lines as a timeline of your program's execution. Utilizing the powerful StructOpt and regex libraries, Timeln ensures straightforward usage via CLI and powerful regular expression capabilities.

## Install from Crates.io

The program is installed on crates.io under [timeln](https://crates.io/crates/timeln)
```shell
cargo install timeln
```

## Install Binaries

See the releases on the [GitHub page](https://github.com/EthanJamesLew/timeln/releases).

## Install from Source
To install Timeln, you first need to have Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone the Timeln repository and build the project using cargo, the Rust package manager:
```shell
git clone https://github.com/EthanJamesLew/timeln.git
cd timeln
cargo build --release
```
The `timeln` binary will now be available in the `target/release` directory. You can move it to a directory on your PATH for easy access:
```shell
mv target/release/timeln <DIR IN PATH>
```

## Usage

Using Timeln is straightforward. Simply pipe the output of a command into `timeln` (`-c` colorizes the output)
```shell
python your_script.py | timeln -c
```
If you want to time between regex matches instead of lines, use the -r option followed by your regex pattern:
```shell
python your_script.py | timeln -r "your_regex_pattern"
```
In this mode, Timeln will only display the lines that match the given regex pattern and will calculate time elapsed and delta time based on these matching lines.

When colorization is enabled, regex matches will be highlighted in red for easy identification.

## Disclaimer

Let's have a heart-to-heart for a sec. Timeln is pretty cool, right? You're timing stuff, watching those millisecond deltas roll by, feeling like a hacker in a Hollywood movie. But wait! Before we get carried away, let's remember something crucial: **Timeln is a tool, not a lifestyle**.

While there's a certain retro charm in squinting at print statements like it's the 1970s, remember that we're in the 21st century and have some awesome modern debugging tools at our disposal! It's kinda like using a carrier pigeon to deliver a message when you've got a perfectly good smartphone in your pocket. Charming? Sure. Effective? Maybe not so much.

In other words, don't fall into the "println debugging" trap. It's like using a bicycle to compete in a Formula 1 race - you might eventually get to the finish line, but it probably won't be a podium finish.

So, as you bask in the green glow of Timeln's output, remember: this tool is just a part of your developer's utility belt, not the belt itself. Use it wisely, use it well, and for goodness sake, don't forget about your debugger.

Happy coding, and remember: print responsibly!
