use loggy::{Logger, Level};
use loggy::formatter::{JsonFormatter, PlainTextFormatter};
use loggy::sink::Console;


// TODO: File is just for test, must provide examples dir later on
fn main() -> std::io::Result<()> {

    let mut logger : Logger<Console,PlainTextFormatter>;
    logger = Logger::new(Level::Debug, Console{}, PlainTextFormatter);

    let name = "Giovanni";
    logger.info(format!("User {} logged in", name));
    logger.add_extra("service", "login-service");
    logger.info(format!("{} calling /login ", name));
    logger.remove_extra("service");
    logger.info(format!("{} calling /logout ", name));

    Ok(())
}