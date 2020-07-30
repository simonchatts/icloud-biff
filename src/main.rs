#[async_std::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    match &args[..] {
        [_prog_name, config_fname] => icloud_biff::run(config_fname).await,
        _ => {
            eprintln!("Usage: icloud-biff config.json");
            std::process::exit(1)
        }
    }
}
