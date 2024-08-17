use std::io::Stderr;

use ratatui::prelude::CrosstermBackend;
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};
use tokio_util::sync::CancellationToken;


#[derive(Debug,Clone)]
pub enum Event {
}

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<Stderr>>,
    pub task: JoinHandle<()>,
    pub cancellation_token: CancellationToken,
    pub event_reciver: UnboundedReceiver<Event>,
}