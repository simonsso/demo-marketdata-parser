use marketdata::MarketData;
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
                match block {
                    PcapBlockOwned::LegacyHeader(_hdr) => {
                        // save hdr.network (linktype)
                    }
                    PcapBlockOwned::Legacy(b) => {
                        // use linktype to parse b.data()
                        if let Ok(md) = MarketData::try_from(b.data) {
                            println!("Hello {} {}", md.data_type(), md.isin())
                        }
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
