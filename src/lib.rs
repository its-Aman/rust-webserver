use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    thread::JoinHandle,
};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    thread: Option<JoinHandle<()>>,
    id: usize,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending terminate message to all workers");
        
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker id={} ", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, String> {
        if (size > 0) {
            let mut workers = Vec::with_capacity(size);

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(String::from("size can't be less then 0"))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker: I got the id={} job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Workekr {} was told to terminate", id);
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
