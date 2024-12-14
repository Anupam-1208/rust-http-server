use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

pub struct Worker {
    id:usize,
    thread:Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>> ) -> Self {
        let thread = thread::spawn( move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                // this will work when drop is called on the message, and to stop the loop this will catch err and stop reading from the channel
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {id, thread:Some(thread)}
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        ThreadPool { workers, sender:Some(sender) }
    }
    pub fn execute<F>(&self, f: F) -> ()
    where
        F: Send + 'static + FnOnce(),
    {
        println!("excuting called");
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker thread {}",worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

    }
}

