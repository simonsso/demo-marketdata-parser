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
                    PcapBlockOwned::LegacyHeader(_hdr) => {}
                    PcapBlockOwned::Legacy(b) => {
                        let pkt_time = b.ts_usec;
                        if let Ok(md) = MarketData::try_from(b.data) {
                            // Should print:
                            // <pkt-time> <accept-time> <issue-code> <bqty5>@<bprice5> ... <bqty1>@<bprice1> <aqty1>@<aprice1> ... <aqty5>@<aprice5>

                            let s = format!(
                                "{} {} {} {}@{} {}@{} {}@{} {}@{} {}@{} {}@{} {}@{} {}@{} {}@{} {}@{}",
                                pkt_time,
                                md.quote_accept_time(),
                                md.issue_code(),
                                md.best_bid_quantity_5(),
                                md.best_bid_price_5(),
                                md.best_bid_quantity_4(),
                                md.best_bid_price_4(),
                                md.best_bid_quantity_3(),
                                md.best_bid_price_3(),
                                md.best_bid_quantity_2(),
                                md.best_bid_price_2(),
                                md.best_bid_quantity_1(),
                                md.best_bid_price_1(),
                                md.best_ask_quantity_1(),
                                md.best_ask_price_1(),
                                md.best_ask_quantity_2(),
                                md.best_ask_price_2(),
                                md.best_ask_quantity_3(),
                                md.best_ask_price_3(),
                                md.best_ask_quantity_4(),
                                md.best_ask_price_4(),
                                md.best_ask_quantity_5(),
                                md.best_ask_price_5()
                            );
                            println!("{s}");
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
