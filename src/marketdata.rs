use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MarketData {
    pub data_type: [u8; 2],                  // B6
    pub information_type: [u8; 2],           // 03
    pub market_type: u8,                     // 4
    pub issue_code: [u8; 12],                // ISIN code
    pub issue_seq_no: [u8; 3],               // Issue seq.-no.
    pub market_status_type: [u8; 2],         // Market Status Type
    pub total_bid_quote_volume: [u8; 7],     // Total bid quote volume
    pub best_bid_price_1: [u8; 5],         // Best bid price (1st)
    pub best_bid_quantity_1: [u8; 7],      // Best bid quantity (1st)
    pub best_bid_price_2: [u8; 5],         // Best bid price (2nd)
    pub best_bid_quantity_2: [u8; 7],      // Best bid quantity (2nd)
    pub best_bid_price_3: [u8; 5],         // Best bid price (3rd)
    pub best_bid_quantity_3: [u8; 7],      // Best bid quantity (3rd)
    pub best_bid_price_4: [u8; 5],         // Best bid price (4th)
    pub best_bid_quantity_4: [u8; 7],      // Best bid quantity (4th)
    pub best_bid_price_5: [u8; 5],         // Best bid price (5th)
    pub best_bid_quantity_5: [u8; 7],      // Best bid quantity (5th)
    pub total_ask_quote_volume: [u8; 7],     // Total ask quote volume
    pub best_ask_price_1: [u8; 5],         // Best ask price (1st)
    pub best_ask_quantity_1: [u8; 7],      // Best ask quantity (1st)
    pub best_ask_price_2: [u8; 5],         // Best ask price (2nd)
    pub best_ask_quantity_2: [u8; 7],      // Best ask quantity (2nd)
    pub best_ask_price_3: [u8; 5],         // Best ask price (3rd)
    pub best_ask_quantity_3: [u8; 7],      // Best ask quantity (3rd)
    pub best_ask_price_4: [u8; 5],         // Best ask price (4th)
    pub best_ask_quantity_4: [u8; 7],      // Best ask quantity (4th)
    pub best_ask_price_5: [u8; 5],         // Best ask price (5th)
    pub best_ask_quantity_5: [u8; 7],      // Best ask quantity (5th)
    pub no_of_best_bid_valid_quote: [u8; 5], // No. of best bid valid quote (total)
    pub no_of_best_bid_quote_1: [u8; 4],   // No. of best bid quote (1st)
    pub no_of_best_bid_quote_2: [u8; 4],   // No. of best bid quote (2nd)
    pub no_of_best_bid_quote_3: [u8; 4],   // No. of best bid quote (3rd)
    pub no_of_best_bid_quote_4: [u8; 4],   // No. of best bid quote (4th)
    pub no_of_best_bid_quote_5: [u8; 4],   // No. of best bid quote (5th)
    pub no_of_best_ask_valid_quote: [u8; 5], // No. of best ask valid quote (total)
    pub no_of_best_ask_quote_1: [u8; 4],   // No. of best ask quote (1st)
    pub no_of_best_ask_quote_2: [u8; 4],   // No. of best ask quote (2nd)
    pub no_of_best_ask_quote_3: [u8; 4],   // No. of best ask quote (3rd)
    pub no_of_best_ask_quote_4: [u8; 4],   // No. of best ask quote (4th)
    pub no_of_best_ask_quote_5: [u8; 4],   // No. of best ask quote (5th)
    pub quote_accept_time: [u8; 8],          // HHMMSSuu
    pub end_of_message: u8,                  // 0xff
}

impl MarketData {
    pub fn issue_code(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.issue_code)
    }
    pub fn data_type(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.data_type)
    }
    pub fn quote_accept_time(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.quote_accept_time)
    }
    pub fn best_bid_price_1(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_price_1)
    }
    pub fn best_bid_quantity_1(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_quantity_1)
    }
    pub fn best_bid_price_2(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_price_2)
    }
    pub fn best_bid_quantity_2(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_quantity_2)
    }
    pub fn best_bid_price_3(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_price_3)
    }
    pub fn best_bid_quantity_3(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_quantity_3)
    }
    pub fn best_bid_price_4(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_price_4)
    }
    pub fn best_bid_quantity_4(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_quantity_4)
    }
    pub fn best_bid_price_5(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_price_5)
    }
    pub fn best_bid_quantity_5(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.best_bid_quantity_5)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum MDError {
    #[error("Data format not supported")]
    DataTypeUnknown,
    #[error("Data of size {0} not supported")]
    DataSizeUnsupported(usize),
    #[error("bincode::ErrorKind {0}")]
    BinCode(#[from] Box<bincode::ErrorKind>),
}

impl TryFrom<&[u8]> for MarketData {
    type Error = MDError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 257 {
            return Err(MDError::DataSizeUnsupported(value.len()));
        }
        let decoded: MarketData = bincode::deserialize(&value[42..257])?;
        if decoded.data_type != [0x42, 0x36] {
            return Err(MDError::DataTypeUnknown);
        };
        if decoded.information_type != [0x30, 0x33] {
            return Err(MDError::DataTypeUnknown);
        }
        if decoded.end_of_message != 0xff {
            return Err(MDError::DataTypeUnknown);
        }
        Ok(decoded)
    }
}
#[test]
fn parse_market_data_from_clean_bytes() {
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

#[test]
fn try_from_for_full_udp() {
    let full_udp_packet_found_in_example_files: &[u8] = &[
        1, 0, 94, 37, 54, 61, 0, 18, 68, 200, 56, 10, 8, 0, 69, 0, 0, 243, 11, 241, 0, 0, 59, 17,
        145, 136, 192, 166, 1, 120, 233, 37, 54, 61, 141, 203, 60, 155, 0, 223, 40, 57, 66, 54, 48,
        51, 52, 75, 82, 52, 50, 48, 49, 70, 51, 50, 55, 50, 49, 48, 48, 50, 52, 48, 48, 48, 50, 48,
        55, 48, 54, 48, 48, 49, 52, 50, 48, 48, 48, 48, 48, 48, 53, 48, 48, 49, 52, 49, 48, 48, 48,
        48, 48, 55, 53, 48, 48, 49, 52, 48, 48, 48, 48, 48, 54, 54, 56, 48, 48, 49, 51, 57, 48, 48,
        48, 48, 50, 52, 54, 48, 48, 49, 51, 56, 48, 48, 48, 48, 53, 49, 57, 48, 48, 48, 52, 53, 49,
        53, 48, 48, 49, 52, 51, 48, 48, 48, 48, 48, 51, 57, 48, 48, 49, 52, 52, 48, 48, 48, 48, 48,
        54, 50, 48, 48, 49, 52, 53, 48, 48, 48, 48, 48, 55, 56, 48, 48, 49, 52, 54, 48, 48, 48, 48,
        48, 55, 52, 48, 48, 49, 52, 55, 48, 48, 48, 48, 48, 55, 53, 48, 48, 55, 55, 56, 48, 48, 48,
        52, 48, 48, 48, 56, 48, 48, 50, 49, 48, 48, 49, 52, 48, 48, 53, 48, 48, 48, 51, 48, 55, 48,
        48, 48, 52, 48, 48, 48, 52, 48, 48, 49, 48, 48, 48, 48, 51, 48, 48, 48, 51, 48, 57, 48, 48,
        50, 57, 57, 55, 255,
    ];

    let md: Result<MarketData, _> = full_udp_packet_found_in_example_files.try_into();
    assert!(md.is_ok())
}
