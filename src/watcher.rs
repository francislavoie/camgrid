use notify::{raw_watcher, RawEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::TryRecvError::Empty;
use std::sync::mpsc::{channel, Receiver};

pub struct Watch {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::RawEvent>,
}

impl Watch {
    pub fn new() -> Watch {
        let (tx, rx) = channel();

        let watcher = raw_watcher(tx).unwrap();

        Watch {
            watcher: watcher,
            rx: rx,
        }
    }

    pub fn watch(&mut self, path: &String) {
        self.watcher.watch(path, RecursiveMode::Recursive).unwrap();
    }

    pub fn unwatch(&mut self, path: &String) {
        self.watcher.unwatch(path).unwrap();
    }

    pub fn recv(&self) -> Option<RawEvent> {
        match self.rx.try_recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
            }
            Ok(event) => {
                println!("broken event: {:?}", event);
            }
            Err(e) => {
                if e != Empty {
                    println!("watch error: {:?}", e)
                }
            }
        }
        None
    }
}
