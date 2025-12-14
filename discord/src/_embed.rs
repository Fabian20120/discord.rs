use super::_user::User;
use super::_colour::Colour;

#[derive(Debug, Clone)]
pub struct Embed<'a> {
    author: User,
    colour: Colour,
    description: &'a str,
    fields: (),
    flags: (),
    footer: &'a str,
    image: (),
    provider: (),
    thumbnail: (),
    timestamp: u64,
    title: &'a str,
    _type: (),
    url: &'a str,
    video: (),
}

impl <'a>Embed<'a> {
    pub fn from_dict() {

    }

    pub fn add_field() {

    }

    pub fn clear_fields() {

    }

    pub fn copy() {

    }

    pub fn insert_field_at() {

    }

    pub fn remove_author() {

    }

    pub fn remove_field() {

    }

    pub fn remove_footer() {

    }

    pub fn set_author() {

    }

    pub fn set_field_at() {

    }

    pub fn set_footer() {

    }

    pub fn set_image() {

    }

    pub fn set_thumbnail() {

    }

    pub fn to_dict() {

    }
}