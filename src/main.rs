use loggy::level::Level;
use loggy::logger::Logger;


fn main(){
    let logger = Logger::new(Level::Debug);
    logger.debug("Hello loggy");
}
