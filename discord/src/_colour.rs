use scan_fmt::scan_fmt;

#[derive(Debug, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub value: u32,
}

macro_rules! make_colour {
    ($r:expr, $g:expr, $b:expr) => {
        Self {
            r: $r,
            g: $g,
            b: $b,
            value: rgb_to_raw($r, $g, $b)
        }
    };
}

fn rgb_to_raw<'a>(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

impl Colour {
    // --------------- Convertion ---------------
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b, value: rgb_to_raw(r, g, b) }
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r1, g1, b1) = match h {
            h if (0.0..60.0).contains(&h)    => (c, x, 0.0),
            h if (60.0..120.0).contains(&h)  => (x, c, 0.0),
            h if (120.0..180.0).contains(&h) => (0.0, c, x),
            h if (180.0..240.0).contains(&h) => (0.0, x, c),
            h if (240.0..300.0).contains(&h) => (x, 0.0, c),
            _ => (c, 0.0, x), // 300..360
        };

        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;

        Colour { r, g, b, value: (rgb_to_raw(r, g, b)) }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim();

        // ----- rgb() -----
        if s.to_lowercase().starts_with("rgb(") && s.ends_with(")") {
            let inner = &s[4..s.len()-1];
            let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
            if parts.len() != 3 {
                return Err("Invalid rgb format".into());
            }

            let mut rgb = [0u8; 3];
            for (i, part) in parts.iter().enumerate() {
                rgb[i] = if part.ends_with('%') {
                    let val: f32 = part[..part.len() -1].parse().map_err(|_| "Invalid percent")?;
                    ((val / 100.0) * 255.0).round() as u8
                } else {
                    part.parse::<u8>().map_err(|_| "Invalid number")?
                };
            }

            return Ok(Self {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
                value: rgb_to_raw(rgb[0], rgb[1], rgb[2]),
            });
        }

        // ----- 0x..., #..., 0x#... -----
        let hex = s
            .trim_start_matches("0x")
            .trim_start_matches("#")
            .trim_start_matches("0x#");

        let hex_num = u32::from_str_radix(hex, 16).map_err(|_| "Invalid hex")?;

        let colour = if hex.len() <= 3 {
            // 3-digit shortcut
            let r = ((hex_num >> 8) & 0xF) as u8 * 17;
            let g = ((hex_num >> 4) & 0xF) as u8 * 17;
            let b = (hex_num & 0xF) as u8 * 17;
            Self { r, g, b, value: rgb_to_raw(r, g, b) }
        } else if hex.len() == 6 {
            // 6-digit hex
            let r = ((hex_num >> 16) & 0xFF) as u8;
            let g = ((hex_num >> 8) & 0xFF) as u8;
            let b = (hex_num & 0xFF) as u8;
            Self { r, g, b, value: rgb_to_raw(r, g, b) }
        } else {
            return Err("Invalid hex lenght".into());
        };

        Ok(colour)
    }

    // --------------- Factory Colours ---------------
    pub fn default() -> Self { make_colour!(0, 0, 0) }
    pub fn teal() -> Self { make_colour!(26, 188, 156) }
    pub fn dark_teal() -> Self { make_colour!(17, 128, 106) }
    pub fn brand_green() -> Self { make_colour!(87, 242, 135) }
    pub fn green() -> Self { make_colour!(46, 204, 113) }
    pub fn dark_green() -> Self { make_colour!(31, 139, 76) }
    pub fn blue() -> Self { make_colour!(52, 152, 219) }
    pub fn dark_blue() -> Self { make_colour!(32, 102, 148) }
    pub fn pruple() -> Self { make_colour!(155, 89, 182) }
    pub fn dark_purple() -> Self { make_colour!(113, 54, 138) }
    pub fn magenta() -> Self { make_colour!(233, 30, 99) }
    pub fn dark_magenta() -> Self { make_colour!(173, 20, 87) }
    pub fn gold() -> Self { make_colour!(241, 196, 15) }
    pub fn dark_gold() -> Self { make_colour!(194, 124, 14) }
    pub fn orange() -> Self { make_colour!(230, 126, 34) }
    pub fn dark_orange() -> Self { make_colour!(163, 67, 0) }
    pub fn brand_red() -> Self { make_colour!(237, 66, 69) }
    pub fn red() -> Self { make_colour!(231, 76, 60) }
    pub fn dark_red() -> Self { make_colour!(153, 45, 34) }
    pub fn lighter_grey() -> Self { make_colour!(149, 165, 166) }
    pub fn light_gray() -> Self { make_colour!(151, 156, 159) }
    pub fn darker_gray() -> Self { make_colour!(84, 110, 122) }
    pub fn og_burple() -> Self { make_colour!(114, 137, 218) }
    pub fn burple() -> Self { make_colour!(88, 101, 242) }
    pub fn greyple() -> Self { make_colour!(153, 170, 181) }
    pub fn ash_theme() -> Self { make_colour!(46, 46, 52) }
    pub fn dark_theme() -> Self { make_colour!(26, 26, 30) }
    pub fn onyx_theme() -> Self { make_colour!(7, 7, 9) }
    pub fn light_theme() -> Self { make_colour!(251, 251, 251) }
    pub fn fucksia() -> Self { make_colour!(235, 69, 158) }
    pub fn yellow() -> Self { make_colour!(254, 231, 92) }
    pub fn ash_embed() -> Self { make_colour!(55, 55, 62) }
    pub fn dark_embed() -> Self { make_colour!(36, 36, 41) }
    pub fn onyx_embed() -> Self { make_colour!(19, 20, 22) }
    pub fn light_embed() -> Self { make_colour!(255, 255, 255) }
    pub fn pink() -> Self { make_colour!(235, 69, 159) }
}