use anyhow;
use vergen::{Config, vergen};

fn main() -> Result<(), anyhow::Error>{
  // Generate the default 'cargo:' instruction output
  vergen(Config::default())
}
