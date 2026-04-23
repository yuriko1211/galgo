use magnus::{function, prelude::*, Error, Ruby};
use rand::prelude::*;
use rand::rng;

fn change_galgo(message: String) -> String {
    format!(" {message} {}", random_gal_emoji())
}

fn random_gal_emoji() -> String {
    let mut emojis = [
        "(*•̀ᴗ•́*)و ̑̑",
        "(⌒_⌒)",
        "( ˘ω˘ )",
        "٩(˘◡˘)۶",
        "(˶′◡‵˶)"
    ];
    emojis.shuffle(&mut rng());
    emojis[0].to_string()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Galgo")?;
    module.define_singleton_method("change_galgo", function!(change_galgo, 1))?;
    Ok(())
}
