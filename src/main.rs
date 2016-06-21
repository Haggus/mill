extern crate notify;

use std::sync::mpsc::channel;
use notify::{RecommendedWatcher, Watcher};

fn watch() -> notify::Result<()> {
    //create channel to get events
    let (tx, rx) = channel();

    //auto select best implementation for specific platform
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    //add path to be watched
    try!(watcher.watch("D:/CleanFolder"));
    try!(watcher.watch("D:/DirtyFolder"));

    loop {
        match rx.recv() {
            Ok(notify::Event{ path: Some(path), op: Ok(op) }) => {
                println!("{:?} {:?}", op, path);
            },
            Err(e) => {
                println!("watch error {}", e);
            },
            _ => ()
        }
    }
}

fn main() {
    if let Err(err) = watch() {
        println!("Error! {:?}", err)
    }
}
