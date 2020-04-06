
use std::io::{LineWriter, stderr, Write};
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    let handle;
    {
        let logger = logger.clone();
        handle = thread::spawn(move || {
            for i in 1..5 {
                logger.log(format!("hi number {} from the spawned thread!\n", i).as_str());
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    for i in 1..5 {
        logger.log(format!("hi number {} from the main thread!\n", i).as_str());
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}