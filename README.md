# Parse market data interview task
Print all quotes of type "B6034" from a dump of packages.

## Execute
```
Usage: parse_market_data [OPTIONS] <DUMP_FILE>

Arguments:
  <DUMP_FILE>  PCAP dump filename

Options:
  -r             Sort on accepted time
  -h, --help     Print help
  -V, --version  Print version
```

For instance like this where mdf-kospi200.20110216-0.pcap is the unpacked provided example file.

```
cargo run -- mdf-kospi200.20110216-0.pcap
```


## Known limitations
* This code will not preform well when sorting a large capture, since it store everything in memory and sorts it afterwards. One possible improvement is to use two threads and pipe the data to another printer process which will take advantage of the 3 second window and only sort and store the last 3 seconds of data. To implement this I would create a reference data set with current implementation and compare it to the optimized for correctness.

* It was not obvious to how ts_sec and ts_usec were defined the name indicate time = ts_sec * 10^6 + ts_usec but the implementation from the crate fetches them from higher and lower part of a 64 bit word.

* Any data in bids, asks or quantity fields not a number will be printed as 0

* Panics and unwraps: I generally consider `unwrap`s harmful, in this case I used the example code from pcap crate and have not yet investigated the impacts of this. 