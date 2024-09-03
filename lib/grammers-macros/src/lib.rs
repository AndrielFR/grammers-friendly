#[macro_export]
macro_rules! command {
    ($command:expr) => {
        ::grammers_friendly::filters::CommandFilter::new("/", $command)
    };
    ($prefixes:expr, $command:expr) => {
        ::grammers_friendly::filters::CommandFilter::new($prefixes, $command)
    };
}
