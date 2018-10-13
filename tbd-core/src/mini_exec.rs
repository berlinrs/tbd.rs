use std::future::Future;
use std::pin::Pin;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{sync_channel, SyncSender, SendError, Receiver};
use std::task::{
    self,
    local_waker_from_nonlocal,
    Poll,
    Wake,
};
use std::time::Duration;

use futures::future::FutureObj;
use futures::task::{Spawn, SpawnError};

static TIMEOUT: Duration = Duration::from_millis(100);

pub struct Executor {
    task_sender: SyncSender<Arc<Task>>,
    task_receiver: Receiver<Arc<Task>>,
}

impl<'a> Spawn for &'a Executor {
    fn spawn_obj(&mut self, future: FutureObj<'static, ()>)
        -> Result<(), SpawnError>
    {
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).map_err(|SendError(_)| {
            SpawnError::shutdown()
        })
    }
}

struct Task {
    future: Mutex<Option<FutureObj<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl Wake for Task {
    fn wake(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        let _ = arc_self.task_sender.send(cloned);
    }
}

impl Executor {
    pub fn new() -> Self {
        let (task_sender, task_receiver) = sync_channel(1000);
        Executor { task_sender, task_receiver }
    }

    pub fn run(&self) {
        let mut executor = &*self;
        while let Ok(task) = self.task_receiver.recv_timeout(TIMEOUT) {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Should we use the ref version here? might be nice to start
                // w/o futures crate at first to show that it can be done,
                // and just mention that there's a simple function to avoid
                // the clone if anyone asks?
                let waker = local_waker_from_nonlocal(task.clone());
                if let Poll::Pending = Pin::new(&mut future).poll(&waker) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}