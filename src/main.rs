use loggy::level::Level;
use loggy::logger::Logger;


// TODO: File is just for test, must provide examples dir later on
fn main(){
    let logger = Logger::new(Level::Debug);
    let name  = "Giovanni";
    logger.info(format!("User {} logged in", name));
}
