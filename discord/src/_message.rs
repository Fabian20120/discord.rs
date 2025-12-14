use super::_user;

struct Message<'a> {
    pub activity: (),
    pub application: (),
    pub application_id: (),
    pub attachments: (),
    pub author: _user::User,
    pub call: (),
    pub channel: (),
    pub channel_mentions: (),
    pub clean_content: (),
    pub components: (),
    pub content: &'a str,
    pub created_at: u64,
    pub edited_at: u64,
    pub embeds: (),
    pub flags: (),
    pub guild: (),
    pub id: u64,
    pub interaction: (),
    pub interaction_metadata: (),
    pub jump_url: (),
    pub mention_everyone: (),
    pub mentions: (),
    pub message_snapshots: (),
    pub nonce: (),
    pub pinned: (),
    pub pinned_at: (),
    pub poll: (),
    pub position: (),
    pub purchase_notification: (),
    pub raw_channel_mentions: (),
    pub raw_mentions: (),
    pub raw_role_mentions: (),
    pub reactions: (),
    pub reference: (),
    pub role_mentions: (),
    pub role_subscription: (),
    pub stickers: (),
    pub system_content: (),
    pub thread: (),
    pub tts: (),
    pub _type: (),
    pub webhook_id: (),
}

impl <'a>Message<'a> {
    pub fn add_files(_files: ()) {

    }

    pub fn add_reaction(_reaction: ()) {

    }

    pub fn clear_reaction(_reaction: ()) {

    }

    pub fn clear_reactions() {

    }

    pub fn create_thread() {

    }

    pub fn delete() -> bool {
        return true;
    }

    pub fn edit(_content: &'a str, _embeds: (), _view: ()) {

    }

    pub fn end_poll() {

    }

    pub fn fetch() {

    }

    pub fn fetch_thread() {

    }

    pub fn forward() {

    }

    pub fn is_system() -> bool {
        true
    }
    
    pub fn pin() {

    }

    pub fn publish() {

    }

    pub fn remove_attachments() {

    }

    pub fn remove_action() {

    }

    pub fn reply() {

    }

    pub fn to_reference() {

    }

    pub fn unpin() {
        
    }
}