#![feature(link_args)]

extern crate fsevent;
use std::sync::mpsc::channel;
use std::thread::Thread;

#[allow(dead_code)]
fn main() {
  let (sender, receiver) = channel::<fsevent::Event>();

  let fsevent = fsevent::FsEvent::new(sender);
  fsevent.append_path("../../");
  let observing_thread = fsevent.observe();

  loop {
    select! (
      val = receiver.recv() => {
        println!("{:?}", val);
      }
    )
  }
}
