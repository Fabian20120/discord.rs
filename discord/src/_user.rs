use super::Colour;
use time::OffsetDateTime;
#[derive(Debug, Clone)]
pub struct User {
    pub accent_color: Colour,
    pub accent_colour: Colour,
    pub avatar: (),
    pub avatar_decoration: (),
    pub avatar_decoration_sku_id: (),
    pub banner: (),
    pub bot: bool,
    pub collectibles: (),
    pub color: Colour,
    pub colour: Colour,
    pub created_at: i64,
}