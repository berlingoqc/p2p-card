use game::logic::players::MyPlayerConfiguration;

#[cfg(not(feature = "web"))]
use clap::Parser;

#[cfg(not(feature = "web"))]
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

#[cfg(not(feature = "web"))]
pub fn load_my_player_config() -> Result<MyPlayerConfiguration, ()> {

    let args = Args::parse();


    Ok(MyPlayerConfiguration {
        name: args.user_name,
        wallet_path: args.wall_path,
    })
}

#[cfg(feature = "web")]
pub fn load_my_player_config() -> Result<MyPlayerConfiguration, ()> {
    use wasm_bindgen::prelude::*;

    // Access the global `window` object
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Get the DOM element by ID
    let element = document.get_element_by_id("bevy-canvas")
        .expect("Element not found");
    
    // Cast it to `HtmlElement` to access custom properties
    let html_element = element.dyn_into::<web_sys::HtmlElement>().unwrap();
    
    // Access custom property
    let custom_property = html_element
        .get_attribute("username")
        .unwrap_or_else(|| "default-value".to_string());
    
    println!("Custom property: {}", custom_property);

    Ok(MyPlayerConfiguration {
        name: "berlingoqc".to_string(),
        wallet_path: "".to_string(),
    })
}