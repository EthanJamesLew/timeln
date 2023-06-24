use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "timeln",
    about = "A utility that times lines/regex from stdin."
)]
pub struct TimelnOpt {
    #[structopt(short = "c", long = "color")]
    pub color: bool,
    #[structopt(short = "r", long = "regex")]
    pub regex: Option<String>,
    #[structopt(short = "p", long = "plot")]
    pub plot: bool,
}
