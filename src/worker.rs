use std::path::PathBuf;

use super::AppWindow;
use futures::FutureExt;
use slint::{ComponentHandle, SharedString};
use tokio::{
    runtime::Handle,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

#[derive(Debug)]
pub enum Message {
    Quit,
    ShowOpenDialog,
    StartConvert,
    PackageSelected(SharedString),
}

pub struct Worker {
    pub channel: UnboundedSender<Message>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(ui: &AppWindow) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({
            let handle_weak = ui.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(worker_loop(r, handle_weak))
                    .unwrap()
            }
        });
        Self {
            channel,
            worker_thread,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        let _ = self.channel.send(Message::Quit);
        self.worker_thread.join()
    }
}

async fn worker_loop(
    mut r: UnboundedReceiver<Message>,
    handle: slint::Weak<AppWindow>,
) -> tokio::io::Result<()> {
    loop {
        let m = futures::select! {
            m = r.recv().fuse() => {
                match m {
                    None => {
                        return Ok(())
                    }
                    Some(m) => m,
                }
            }
        };

        match m {
            Message::ShowOpenDialog => {
                println!("message: show file picker...");

                let mut dialog = rfd::FileDialog::new();
                dialog = dialog.set_title("选择目录");
                let new_dir = match dialog.pick_folder() {
                    Some(new_dir) => new_dir.into(),
                    None => default_dir(),
                };

                handle
                    .clone()
                    .upgrade_in_event_loop(move |h| {
                        h.set_dir(new_dir.to_str().unwrap().into());
                    })
                    .unwrap();
            }
            Message::StartConvert => {
                // todo
                println!("message: start convert to json file...");
            }
            Message::Quit => {
                return Ok(());
            }
            _ => {
                println!("message: unknown message...")
            }
        }
    }

    Ok(())
}

fn default_dir() -> PathBuf {
    dunce::canonicalize(
        match std::env::args().skip(1).find(|s| !s.starts_with('-')) {
            Some(p) => p.into(),
            None => std::env::current_dir().unwrap_or_default(),
        },
    )
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dir1() {
        let p1 = default_dir();
        println!("{:#?}", p1.to_str());
    }
}
