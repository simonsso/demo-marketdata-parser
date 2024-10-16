use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
use std::fs::File;

mod config;
mod marketdata;
use clap::Parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::parse();
    let file = File::open(config.dump_file)?;
    let mut reader = LegacyPcapReader::new(65536, file)?;

    loop {
        match reader.next() {
            Ok((offset, block)) => {
                println!("got new block");
                match block {
                    PcapBlockOwned::LegacyHeader(_hdr) => {
                        // save hdr.network (linktype)
                    }
                    PcapBlockOwned::Legacy(b) => {
                        // use linktype to parse b.data()
                        println!("Hello {:02x?}", b.data)
                    }
                    PcapBlockOwned::NG(_) => unreachable!(),
                }
                reader.consume(offset);
            }
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete(_)) => {
                reader.refill().unwrap();
            }
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }
    Ok(())
}
