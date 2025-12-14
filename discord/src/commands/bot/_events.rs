use super::super::super::_user;

pub enum Event {
    Ready,
    MessageCreate(MessageCreateEvent),
}

pub struct MessageCreateEvent {

}