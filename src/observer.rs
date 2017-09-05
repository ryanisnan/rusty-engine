pub struct Event {
    pub id: u8
}

pub trait Observer {
    fn notify(&self, event: &Event) -> ();
}

pub trait Observable {
    fn watch(&self, observer: &Observer) -> ();
}
