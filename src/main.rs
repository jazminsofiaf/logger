
use std::io::{LineWriter, stderr,stdout, Write};
use std::fs::OpenOptions;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;


struct MyLogger {
    out_stream: Box<dyn Write + Send>
}


impl MyLogger {
    fn new() -> MyLogger {
        let default_logger = MyLogger {
            out_stream:  Box::new(LineWriter::new(stdout())) as Box<dyn Write + Send>
        };
        return  default_logger;
    }

    fn set_output_file(&mut self, outfile: &str) {
        let logfile = match OpenOptions::new().create(true).truncate(true).write(true).open(outfile) {
            Ok(f)  => Box::new(LineWriter::new(f)) as Box<dyn Write + Send>,
            Err(e) => {
                println!("Filed to create output file: {}", e );
                Box::new(LineWriter::new(stderr())) as Box<dyn Write + Send>
            }
        };

        self.out_stream = logfile;
    }

    fn log(&mut self, msg: &str) {
        self.out_stream.write_all(msg.as_ref()).expect("error at logging");
    }
}

lazy_static! {
    static ref GLOBAL_LOGGER : Mutex<MyLogger> = Mutex::new(MyLogger::new());
}

fn main() {

    let mut guard = GLOBAL_LOGGER.lock().unwrap();
    guard.set_output_file("file_name.txt");
    drop(guard);

    let handle;
    {
        handle = thread::spawn(move || {
            for i in 1..5 {
                let mut guard = GLOBAL_LOGGER.lock().unwrap();
                guard.log(format!("hi number {} from the spawned thread!\n", i).as_str());
                drop(guard);
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    for i in 1..5 {
        let mut guard = GLOBAL_LOGGER.lock().unwrap();
        guard.log(format!("hi number {} from the main thread!\n", i).as_str());
        drop(guard);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}