use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

pub struct Worker {
    id:usize,
    thread:JoinHandle<()>,
}

impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>> ) -> Self {
        let thread = thread::spawn( move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker {id, thread}
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(connections:usize) -> Self {
        assert!(connections > 0);
        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        // list of workers(threads), that will pickup requests from the channels
        let mut workers: Vec<Worker> = Vec::with_capacity(connections);
        for i in 0..connections {
            // create worker that will create new thread with a id
            // let thread = thread::spawn(|| {});
            // threads.append(thread)

            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.insert(i, worker);
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F) -> ()
    where
        F: Send + 'static + FnOnce(),
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}