use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::worker;

pub struct ThreadPool {
    number: usize,
    workers: Vec<worker::Worker>,
    sender: Option<mpsc::Sender<worker::Job>>,
}

impl ThreadPool {
    pub fn new(number: usize) -> Self {
        if number == 0 {
            panic!("线程数量异常,{:?}", number);
        }
        let (sender, receiver) = mpsc::channel();
        let r = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(number);
        
        for i in 0..number {
            workers.push(worker::Worker::new(i, r.clone()));
        }
        ThreadPool {
            number,
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); // todo 这里为什么要box  因为是dyn trait
        self.sender.as_ref().unwrap().send(job).unwrap()
    }

    pub fn execute2(&self, job: worker::Job) {
        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); //先删除sender
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
