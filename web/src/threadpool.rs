use std::{
    sync::mpsc::{Sender, channel},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    thread_count: i8,
    // Can't exceed thread count.
    thread_sender_queue: Vec<(JoinHandle<()>, Sender<Job>)>,
    current_thread_num: i8,
}

impl ThreadPool {
    pub fn new(thread_count: Option<i8>) -> Self {
        let th_count = thread_count.unwrap_or(8);
        let mut thread_sender_queue = vec![];

        for _ in 0..th_count {
            let (tx, rx) = channel::<Job>();
            let thread_join = thread::spawn(move || {
                while let Ok(receiver) = rx.recv() {
                    receiver();
                }
            });

            thread_sender_queue.push((thread_join, tx));
        }

        ThreadPool {
            thread_count: th_count,
            thread_sender_queue,
            current_thread_num: 0,
        }
    }

    pub fn execute(&mut self, arg: Job) {
        self.thread_sender_queue[self.current_thread_num as usize]
            .1
            .send(arg)
            .unwrap();

        self.current_thread_num = (self.current_thread_num + 1) % self.thread_count;
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for (thread_join, sender) in self.thread_sender_queue.drain(..) {
            drop(sender);
            thread_join.join().unwrap();
        }
    }
}
