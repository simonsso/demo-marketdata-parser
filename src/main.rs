use marketdata::{MarketData, MarketDataPacket};
use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
use std::collections::VecDeque;
use std::fs::File;

mod config;
mod marketdata;
use clap::Parser;

const THREE_SECONDS: u64 = 3 << 32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::parse();
    let file = File::open(config.dump_file)?;
    let mut reader = LegacyPcapReader::new(65536, file)?;
    let mut sortmap: VecDeque<MarketDataPacket> = VecDeque::new();

    loop {
        match reader.next() {
            Ok((offset, block)) => {
                match block {
                    PcapBlockOwned::LegacyHeader(_hdr) => {}
                    PcapBlockOwned::Legacy(b) => {
                        if let Ok(md) = MarketData::try_from(b.data) {
                            // Should print:
                            // <pkt-time> <accept-time> <issue-code> <bqty5>@<bprice5> ... <bqty1>@<bprice1> <aqty1>@<aprice1> ... <aqty5>@<aprice5>

                            let pkt_time = b.ts_usec as u64 + ((b.ts_sec as u64) << 32);
                            let market_data_packet = MarketDataPacket::new(pkt_time, md);

                            if config.sort_on_accepted_time {
                                let quote_accept_time = market_data_packet.quote_accept_time;
                                let pos = sortmap
                                    .binary_search_by(|x| x.raw_cmp((quote_accept_time, pkt_time)));

                                let pos = match pos {
                                    Ok(p) => p,
                                    Err(p) => p,
                                };
                                sortmap.insert(pos, market_data_packet);

                                while let Some(x) = sortmap.front() {
                                    if x.pkt_time + THREE_SECONDS < pkt_time {
                                        if let Some(mdp) = sortmap.pop_front() {
                                            println!("{}", mdp.get_quote_data());
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                println!("{}", market_data_packet.get_quote_data());
                            }
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
    if config.sort_on_accepted_time {
        for s in sortmap {
            println!("{}", s.get_quote_data());
        }
    }
    Ok(())
}
