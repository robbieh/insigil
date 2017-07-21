use state;
use toml;
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(Debug, Deserialize )]
pub struct Config {
    palette: Option<state::Palette>
}

const DEFAULT_PALETTE: state::Palette = state::Palette{
            background: [0.22, 0.16, 0.29, 1.0],
               primary: [0.01, 0.58, 0.31, 0.8],
             secondary: [0.15, 0.90, 0.15, 0.8],
             highlight: [0.79, 0.41, 0.83, 1.0],
            };


pub fn parse_palette_file(path: &str) -> Config {
    let mut toml_string = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error trying to open file {:?}", path);
            return Config{ palette: Some(DEFAULT_PALETTE) };
        }
    };

    file.read_to_string(&mut toml_string).unwrap_or_else(
        |e| panic!("Error trying to read file {}: {}", path, e));
    //let mut parser = Parser::new(&toml_string);
    //let toml = parser.parse();

    //if toml.is_none() {
    //    panic("Error parsing file {}", path);
    //}

    toml::from_str(&toml_string).unwrap()
}

pub fn read_palette() -> state::Palette {

    match env::home_dir() {
        None => DEFAULT_PALETTE,
        Some(path) => {
            parse_palette_file(
                path.join(".insigil.colors.toml").to_str().unwrap())
                .palette.unwrap()
        }
    }
    
}
