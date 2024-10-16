use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
use std::fs::File;

mod config;
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MarketData {
    data_type: [u8; 2],                  // B6
    information_type: [u8; 2],           // 03
    market_type: u8,                     // 4
    issue_code: [u8; 12],                // ISIN code
    issue_seq_no: [u8; 3],               // Issue seq.-no.
    market_status_type: [u8; 2],         // Market Status Type
    total_bid_quote_volume: [u8; 7],     // Total bid quote volume
    best_bid_price_1st: [u8; 5],         // Best bid price (1st)
    best_bid_quantity_1st: [u8; 7],      // Best bid quantity (1st)
    best_bid_price_2nd: [u8; 5],         // Best bid price (2nd)
    best_bid_quantity_2nd: [u8; 7],      // Best bid quantity (2nd)
    best_bid_price_3rd: [u8; 5],         // Best bid price (3rd)
    best_bid_quantity_3rd: [u8; 7],      // Best bid quantity (3rd)
    best_bid_price_4th: [u8; 5],         // Best bid price (4th)
    best_bid_quantity_4th: [u8; 7],      // Best bid quantity (4th)
    best_bid_price_5th: [u8; 5],         // Best bid price (5th)
    best_bid_quantity_5th: [u8; 7],      // Best bid quantity (5th)
    total_ask_quote_volume: [u8; 7],     // Total ask quote volume
    best_ask_price_1st: [u8; 5],         // Best ask price (1st)
    best_ask_quantity_1st: [u8; 7],      // Best ask quantity (1st)
    best_ask_price_2nd: [u8; 5],         // Best ask price (2nd)
    best_ask_quantity_2nd: [u8; 7],      // Best ask quantity (2nd)
    best_ask_price_3rd: [u8; 5],         // Best ask price (3rd)
    best_ask_quantity_3rd: [u8; 7],      // Best ask quantity (3rd)
    best_ask_price_4th: [u8; 5],         // Best ask price (4th)
    best_ask_quantity_4th: [u8; 7],      // Best ask quantity (4th)
    best_ask_price_5th: [u8; 5],         // Best ask price (5th)
    best_ask_quantity_5th: [u8; 7],      // Best ask quantity (5th)
    no_of_best_bid_valid_quote: [u8; 5], // No. of best bid valid quote (total)
    no_of_best_bid_quote_1st: [u8; 4],   // No. of best bid quote (1st)
    no_of_best_bid_quote_2nd: [u8; 4],   // No. of best bid quote (2nd)
    no_of_best_bid_quote_3rd: [u8; 4],   // No. of best bid quote (3rd)
    no_of_best_bid_quote_4th: [u8; 4],   // No. of best bid quote (4th)
    no_of_best_bid_quote_5th: [u8; 4],   // No. of best bid quote (5th)
    no_of_best_ask_valid_quote: [u8; 5], // No. of best ask valid quote (total)
    no_of_best_ask_quote_1st: [u8; 4],   // No. of best ask quote (1st)
    no_of_best_ask_quote_2nd: [u8; 4],   // No. of best ask quote (2nd)
    no_of_best_ask_quote_3rd: [u8; 4],   // No. of best ask quote (3rd)
    no_of_best_ask_quote_4th: [u8; 4],   // No. of best ask quote (4th)
    no_of_best_ask_quote_5th: [u8; 4],   // No. of best ask quote (5th)
    quote_accept_time: [u8; 8],          // HHMMSSuu
    end_of_message: u8,                  // 0xff
}

#[test]
fn parse_market_data() {
    let encoded: [u8; 215] = [
        0x42, 0x36, 0x30, 0x33, 0x34, 0x4b, 0x52, 0x34, 0x33, 0x30, 0x31, 0x46, 0x33, 0x32, 0x35,
        0x37, 0x30, 0x30, 0x30, 0x34, 0x34, 0x30, 0x30, 0x30, 0x31, 0x31, 0x31, 0x32, 0x33, 0x30,
        0x30, 0x32, 0x32, 0x37, 0x30, 0x30, 0x30, 0x30, 0x31, 0x32, 0x37, 0x30, 0x30, 0x32, 0x32,
        0x36, 0x30, 0x30, 0x30, 0x30, 0x31, 0x31, 0x32, 0x30, 0x30, 0x32, 0x32, 0x35, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x34, 0x33, 0x30, 0x30, 0x32, 0x32, 0x34, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x35, 0x34, 0x30, 0x30, 0x32, 0x32, 0x33, 0x30, 0x30, 0x30, 0x30, 0x30, 0x34, 0x32, 0x30,
        0x30, 0x30, 0x36, 0x37, 0x37, 0x35, 0x30, 0x30, 0x32, 0x32, 0x38, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x38, 0x31, 0x30, 0x30, 0x32, 0x32, 0x39, 0x30, 0x30, 0x30, 0x30, 0x31, 0x35, 0x31,
        0x30, 0x30, 0x32, 0x33, 0x30, 0x30, 0x30, 0x30, 0x30, 0x31, 0x37, 0x36, 0x30, 0x30, 0x32,
        0x33, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x35, 0x33, 0x30, 0x30, 0x32, 0x33, 0x32, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x35, 0x30, 0x30, 0x32, 0x38, 0x32, 0x30, 0x30, 0x30, 0x34,
        0x30, 0x30, 0x30, 0x32, 0x30, 0x30, 0x30, 0x32, 0x30, 0x30, 0x30, 0x33, 0x30, 0x30, 0x30,
        0x34, 0x30, 0x30, 0x33, 0x30, 0x35, 0x30, 0x30, 0x30, 0x37, 0x30, 0x30, 0x30, 0x35, 0x30,
        0x30, 0x31, 0x34, 0x30, 0x30, 0x30, 0x31, 0x30, 0x30, 0x30, 0x31, 0x30, 0x39, 0x30, 0x30,
        0x32, 0x39, 0x34, 0x32, 0xff,
    ];
    let decoded: MarketData = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(decoded.data_type, [0x42, 0x36]);
    assert_eq!(decoded.information_type, [0x30, 0x33]);
    assert_eq!(decoded.end_of_message, 0xff);
}
