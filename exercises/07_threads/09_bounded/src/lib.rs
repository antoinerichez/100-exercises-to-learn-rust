use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, RecvError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
    response_capacity: usize,
}

impl TicketStoreClient {
    fn new(sender: SyncSender<Command>, response_capacity: usize) -> Self {
        Self {
            sender,
            response_capacity,
        }
    }

    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, RecvError> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel::<TicketId>(self.response_capacity);
        let command = Command::Insert {
            draft,
            response_channel: response_sender,
        };

        let _ = self.sender.send(command);
        response_receiver.recv()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, RecvError> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel::<Option<Ticket>>(self.response_capacity);
        let command = Command::Get {
            id,
            response_channel: response_sender,
        };

        let _ = self.sender.send(command);
        response_receiver.recv()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender, capacity)
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
