use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short = "c", long)]
    pub city: Option<String>,
}

impl Args {
    fn city() -> Option<String> {
        let args: Args = Args::from_args();
        args.city
    }

    pub fn get_city() -> String {
        let city = Args::city();

        match city {
            Some(value) => value,
            None => {
                println!("No city was found. Using default: Vancouver");
                String::from("Vancouver")
            }
        }
    }
}
