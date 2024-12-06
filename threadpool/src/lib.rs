use std::sync;

pub struct Threadpool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: sync::mpsc::Sender<Box<dyn Fn() + Send>>,
}

impl Threadpool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = sync::mpsc::channel::<Box<dyn Fn() + Send + 'static>>();
        let receiver = sync::Arc::new(sync::Mutex::new(receiver));

        let mut _handles = vec![];
        for _ in 0..num_threads {
            let local_receiver = receiver.clone();
            let handle = std::thread::spawn(move || {
                let work = match local_receiver.lock().unwrap().recv() {
                    Ok(work) => work,
                    Err(_) => return,
                };
                work();
            });
            _handles.push(handle);
        }

        Self { _handles, sender }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use super::Threadpool;

        let pool = Threadpool::new(3);

        pool.execute(|| println!("hello, from thread"));
        pool.execute(|| println!("hello, from thread"));
    }
}
