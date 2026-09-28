use std::thread;
use std::sync::{Arc, Mutex, mpsc};
pub struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}    

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread:: spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message{
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
    
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
pub struct Config {
    pub max_workers: usize,
    pub max_requests: usize,
}
const DEFAULT_MAX_WORKERS: usize = 4;
const DEFAULT_MAX_REQUESTS: usize = 100;
impl Config{

    /// Builds a [`Config`] from command-line arguments.
    ///
    /// `args[0]` is the program name and is ignored. `args[1]` is the maximum
    /// number of worker threads, and `args[2]` is the maximum number of
    /// connections to accept.
    ///
    /// A missing argument panics. A value that cannot be parsed, or that is at
    /// or below the floor, is replaced by that floor: 4 workers and 100
    /// requests. This function does not return an error.
    pub fn build_from_args(args: &[String]) -> Config{
        let args_length = args.len();
        let max_workers = if args_length > 1 { args[1].parse().unwrap_or(DEFAULT_MAX_WORKERS) } else { DEFAULT_MAX_WORKERS };
        let max_requests = if args_length > 2 { args[2].parse().unwrap_or(DEFAULT_MAX_REQUESTS) } else { DEFAULT_MAX_REQUESTS };
        Config {
            max_workers: if max_workers > 0 { max_workers } else { DEFAULT_MAX_WORKERS },
            max_requests: if max_requests > 0 { max_requests } else { DEFAULT_MAX_REQUESTS },
        }
    }
}