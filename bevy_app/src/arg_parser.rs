use game::logic::players::MyPlayerConfiguration;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    user_name: String,

    #[arg(short, long)]
    wall_path: String,

    #[arg(short, long, default_value_t = 1)]
    seed: u64,
}


pub fn load_my_player_config() -> Result<MyPlayerConfiguration, ()> {

    let args = Args::parse();


    Ok(MyPlayerConfiguration {
        name: args.user_name,
        wallet_path: args.wall_path,
    })
}

