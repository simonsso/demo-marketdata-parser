fn main() {
    println!("Hello, world!");
}


struct MarketData {
    data_type: [u8; 2],                // B6
    information_type: [u8; 2],         // 03
    market_type: u8,                   // 4
    issue_code: [u8; 12],              // ISIN code
    issue_seq_no: [u8; 3],             // Issue seq.-no.
    market_status_type: [u8; 2],       // Market Status Type
    total_bid_quote_volume: [u8; 7],   // Total bid quote volume
    best_bid_price_1st: [u8; 5],       // Best bid price (1st)
    best_bid_quantity_1st: [u8; 7],    // Best bid quantity (1st)
    best_bid_price_2nd: [u8; 5],       // Best bid price (2nd)
    best_bid_quantity_2nd: [u8; 7],    // Best bid quantity (2nd)
    best_bid_price_3rd: [u8; 5],       // Best bid price (3rd)
    best_bid_quantity_3rd: [u8; 7],    // Best bid quantity (3rd)
    best_bid_price_4th: [u8; 5],       // Best bid price (4th)
    best_bid_quantity_4th: [u8; 7],    // Best bid quantity (4th)
    best_bid_price_5th: [u8; 5],       // Best bid price (5th)
    best_bid_quantity_5th: [u8; 7],    // Best bid quantity (5th)
    total_ask_quote_volume: [u8; 7],   // Total ask quote volume
    best_ask_price_1st: [u8; 5],       // Best ask price (1st)
    best_ask_quantity_1st: [u8; 7],    // Best ask quantity (1st)
    best_ask_price_2nd: [u8; 5],       // Best ask price (2nd)
    best_ask_quantity_2nd: [u8; 7],    // Best ask quantity (2nd)
    best_ask_price_3rd: [u8; 5],       // Best ask price (3rd)
    best_ask_quantity_3rd: [u8; 7],    // Best ask quantity (3rd)
    best_ask_price_4th: [u8; 5],       // Best ask price (4th)
    best_ask_quantity_4th: [u8; 7],    // Best ask quantity (4th)
    best_ask_price_5th: [u8; 5],       // Best ask price (5th)
    best_ask_quantity_5th: [u8; 7],    // Best ask quantity (5th)
    no_of_best_bid_valid_quote: [u8; 5], // No. of best bid valid quote (total)
    no_of_best_bid_quote_1st: [u8; 4],  // No. of best bid quote (1st)
    no_of_best_bid_quote_2nd: [u8; 4],  // No. of best bid quote (2nd)
    no_of_best_bid_quote_3rd: [u8; 4],  // No. of best bid quote (3rd)
    no_of_best_bid_quote_4th: [u8; 4],  // No. of best bid quote (4th)
    no_of_best_bid_quote_5th: [u8; 4],  // No. of best bid quote (5th)
    no_of_best_ask_valid_quote: [u8; 5], // No. of best ask valid quote (total)
    no_of_best_ask_quote_1st: [u8; 4],  // No. of best ask quote (1st)
    no_of_best_ask_quote_2nd: [u8; 4],  // No. of best ask quote (2nd)
    no_of_best_ask_quote_3rd: [u8; 4],  // No. of best ask quote (3rd)
    no_of_best_ask_quote_4th: [u8; 4],  // No. of best ask quote (4th)
    no_of_best_ask_quote_5th: [u8; 4],  // No. of best ask quote (5th)
    quote_accept_time: [u8; 8],         // HHMMSSuu
    end_of_message: u8,                 // 0xff
}
