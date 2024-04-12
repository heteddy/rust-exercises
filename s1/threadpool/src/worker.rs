use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, panicking, JoinHandle};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    worker_id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let t = thread::spawn(move || loop {
            let j = receiver.lock().unwrap().recv();
            match j {
                Ok(job) => {
                    println!("{} received job", id);
                    job();
                }
                Err(e) => {
                    eprintln!("receive message error,maybe close sender {:?}", e);
                    break; //
                }
            }
        });
        Worker {
            worker_id: id,
            thread: Some(t),
        }
    }
}
