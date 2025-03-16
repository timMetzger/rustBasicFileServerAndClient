use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}


type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug,Clone)]
pub struct PoolCreationError;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.send(job).unwrap();
    }
    
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 0
        {
            return Err(PoolCreationError);
        }
        else
        {
            return Ok(Self::new(size));
        }
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            
            match job{
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker failed");
                }
            }
        });
        
        Worker { id, thread }
    }
}