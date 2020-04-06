
use std::io::{LineWriter, stderr, Write};
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};

struct MyLogger {
    out_stream: Mutex<Box<dyn Write + Send>>
}


impl MyLogger {
    fn init(outfile: &str) -> Arc<MyLogger> {
        let logfile = match OpenOptions::new().create(true).truncate(true).write(true).open(outfile) {
            Ok(f)  => Box::new(LineWriter::new(f)) as Box<dyn Write + Send>,
            Err(e) => {
                println!("Filed to create output file: {}", e );
                Box::new(LineWriter::new(stderr())) as Box<dyn Write + Send>
            }
        };


         let logger = MyLogger {
                out_stream: Mutex::new(logfile)
         };
         return  std::sync::Arc::new(logger);
    }

    fn log(& self, msg: &str) {
        let mut guard = self.out_stream.lock().unwrap();
        guard.write_all(msg.as_ref()).expect("error at logging");
    }
}

fn main() {
    let logger = &mut MyLogger::init("file_name.txt");
    logger.log("En el thread main\n");





}