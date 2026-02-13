use loggy::level::Level;
use loggy::logger::Logger;


// TODO: File is just for test, must provide examples dir later on
fn main(){
    let logger = Logger::new(Level::Debug);
    logger.debug("Hello loggy");
}
