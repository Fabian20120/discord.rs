use super::super::super::_user;

#[derive(Debug, Clone)]
pub enum Event {
    Ready,
    MessageCreate(MessageCreateEvent),
}

#[derive(Debug, Clone)]
pub struct MessageCreateEvent {

}