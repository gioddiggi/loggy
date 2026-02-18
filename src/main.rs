use loggy::sink::Console;
use loggy::logger::Logger;
use loggy::Level;
use loggy::formatter::JsonFormatter;
// TODO: File is just for test, must provide examples dir later on
fn main() -> std::io::Result<()> {

    let logger : Logger<Console,JsonFormatter>;

    logger = Logger::new(Level::Debug, Console{}, JsonFormatter{});

    let name = "Giovanni";
    logger.info(format!("User {} logged in", name));

    Ok(())
}