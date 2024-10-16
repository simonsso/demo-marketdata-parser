use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// PCAP dump filename
    pub dump_file: String,
    /// Sort on accepted time
    #[clap(short = 'r')]
    pub sort_on_accepted_time: bool,
}
