use crate::cbl::cbi::ClientBufferItem;

mod cbi;

pub struct ClientBufferList {
    pub first: Option<ClientBufferItem>,
    pub last: Option<ClientBufferItem>,
}

impl ClientBufferList {
    pub fn new() -> ClientBufferList {
        ClientBufferList {
            first: None,
            last: None,
        }
    }

    pub fn search(&self, id: u32) -> Result<ClientBufferItem, ()> {
        let mut last = self.first;
        loop {
            if last == None {
                return Err(());
            }
            let mut cbi = last.unwrap();
            if cbi.id == id {
                return Ok(cbi);
            }
            last = cbi.next;
        }
    }

    pub fn clean_timeouts(&mut self) {
        let mut last = self.first;
        loop {
            if last == None {
                return;
            }
            let mut cbi = last.unwrap();
            if cbi.is_overdue() {
                if cbi.previous == None {
                    self.first = cbi.next;
                    if cbi.next != None {
                        cbi.next.previous = None;
                    }
                } else {
                    cbi.previous.next = cbi.next;
                    if cbi.next != None {
                        cbi.next = cbi.previous;
                    }
                }
                if cbi == self.last.unwrap() {
                    self.last = cbi.previous;
                }
            }
            last = cbi.next;
        }
    }
}