use loggy::logger::Logger;
use loggy::console::ConsoleLogger;
use loggy::level::Level;

// TODO: File is just for test, must provide examples dir later on
fn main(){
    let console_logger = ConsoleLogger{};
    let logger = Logger::new(Level::Debug, console_logger);
    let name  = "Giovanni";
    logger.info(format!("User {} logged in", name));
}
 