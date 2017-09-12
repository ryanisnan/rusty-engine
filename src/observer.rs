use std::rc::Rc;

pub struct Event {
    pub id: u8
}

pub trait Observer {
    fn notify(&self, object: &Observable, event: &Event) -> ();
}

pub trait Observable {
    fn watch(&mut self, observer: Rc<Observer>) -> ();
}
