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

    pub fn search(&self, ip: u128, id: u32) -> Option<ClientBufferItem> {
        let mut last = self.first;
        loop {
            if last == None {
                return None;
            }
            let mut cbi = last.unwrap();
            if cbi.ip == ip && cbi.id == id {
                return Ok(cbi);
            }
            last = cbi.next;
        }
    }

    // TODO: clean up function
}