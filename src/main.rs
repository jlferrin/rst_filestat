use std::env;
use rst_filestat::Config;
use rst_filestat::StatFile;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);
    let statfile = StatFile::new(&config.filename);

    statfile.print();
    statfile.println();

}

