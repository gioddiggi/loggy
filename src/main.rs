use loggy::file::FileLogger;
use loggy::logger::Logger;
use loggy::level::Level;
use loggy::formatter::JsonFormatter;
// TODO: File is just for test, must provide examples dir later on
fn main() -> std::io::Result<()> {

    let logger : Logger<FileLogger,JsonFormatter>;

    let open_file_result = FileLogger::new("loggy.log");

    match open_file_result {
        Ok(v)=> {logger = Logger::new(Level::Debug, v, JsonFormatter{});}
        Err(e) => { panic!("Could not open file : {:?}", e)}
    }

    let name = "Giovanni";
    logger.info(format!("User {} logged in", name));

    Ok(())
}