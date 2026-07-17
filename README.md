# XCHECK

if you care about certain currencies' exchange rates, use `xcheck`

## Usage:
```console
cargo b --release
./target/release/xcheck --from GBP -t JPY   # see GBP/JPY rate for today
./target/release/xcheck stat 14 -f EUR      # or get a graph of EUR value for the past 14 days

sudo cp ./target/release/xcheck /usr/local/bin
```
