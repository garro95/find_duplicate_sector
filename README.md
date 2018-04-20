# fds

This app with a bad name is just a quick experiment to write a Rust
application using the `quicli` crate.

It may be useful if you want to find the first duplicated sector in a partition,
for example to fix an error like [this](https://askubuntu.com/questions/327509/gparted-freeze-during-ntfs-partition-resize).
It should be faster and more ergonomc then the script suggested by the author of the post.

# Usage
To use the app you need to [install](https://www.rust-lang.org/en-US/install.html)
the Rust stable toolchain on your device and git or a zip archive manager.
Clone this repository or download and extract it.
Open a terminal in the directory where you cloned or extracted the files and do:
```
cargo run --release -- <partition_name> <ref_sector> <start_sector> <end_sector>
```
