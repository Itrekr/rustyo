# About

Rustyo is a simple radio that uses mpv to play radio streams from publically available radiostations.

# Installation

## With cargo

Make sure .cargo/bin is in your path and run:

```
cargo install rustyo
```

## Manual installation

Clone the repo and navigate to it with:

```
git clone https://github.com/itrekr/rustyo
cd rustyo
```

Then run cargo build to compile, after which the binary 'rustyo' can be found in the target/release folder:

```
cargo build --release
```

Linux users can then move the binary to their path of choice.

# Usage

Once installed you can list the radiostations with the 'list' command like so:

```
rustyo list
```

Pick any of the stations and run it with:

```
rustyo stationname
```

To stop the radiostation you can run either of the following commands:

```
rustyo none
killall mpv
```


# Adding stations

Any stream that can be played with 'mpv' should function. In the resources folder you find the stations.csv file which contains all the names and the streaming url's of the different stations. All you have to do to add a station is add a newline with the name and the corresponding streaming url seperated by a comma. After that, recompile the project like so:

```
cargo clean
cargo build --release
```

Don't forget to then move the new binary to your path of choice again, as that won't update automatically.

# Feedback and improvements

Feel free to submit issues, additional stations or improvements on GitHub.
