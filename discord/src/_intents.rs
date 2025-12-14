#[derive(Debug, Clone)]
pub struct Intents {
    pub auto_moderation: bool,
    pub auto_moderation_configuration: bool,
    pub auto_moderation_execution: bool,
    pub bans: bool,
    pub dm_messages: bool,
    pub dm_polls: bool,
    pub dm_reactions: bool,
    pub dm_typing: bool,
    pub emojis: bool,
    pub emojis_and_stickers: bool,
    pub expressions: bool,
    pub guild_messages: bool,
    pub guild_polls: bool,
    pub guild_reactions: bool,
    pub guild_scheduled_events: bool,
    pub guild_typing: bool,
    pub guilds: bool,
    pub integrations: bool,
    pub invites: bool,
    pub members: bool,
    pub message_content: bool,
    pub messages: bool,
    pub moderation: bool,
    pub polls: bool,
    pub presences: bool,
    pub reactions: bool,
    pub typing: bool,
    pub value: bool,
    pub voice_states: bool,
    pub webhooks: bool,
}

impl Intents {
    /// Returns an Intents object where all fields are enabled (true)
    pub fn all() -> Self {
        Self {
            auto_moderation: true,
            auto_moderation_configuration: true,
            auto_moderation_execution: true,
            bans: true,
            dm_messages: true,
            dm_polls: true,
            dm_reactions: true,
            dm_typing: true,
            emojis: true,
            emojis_and_stickers: true,
            expressions: true,
            guild_messages: true,
            guild_polls: true,
            guild_reactions: true,
            guild_scheduled_events: true,
            guild_typing: true,
            guilds: true,
            integrations: true,
            invites: true,
            members: true,
            message_content: true,
            messages: true,
            moderation: true,
            polls: true,
            presences: true,
            reactions: true,
            typing: true,
            value: true,
            voice_states: true,
            webhooks: true,
        }
    }

    /// Discord default (members, presences, message_content disabled)
    pub fn default() -> Self {
        Self {
            auto_moderation: true,
            auto_moderation_configuration: true,
            auto_moderation_execution: true,
            bans: true,
            dm_messages: true,
            dm_polls: true,
            dm_reactions: true,
            dm_typing: true,
            emojis: true,
            emojis_and_stickers: true,
            expressions: true,
            guild_messages: true,
            guild_polls: true,
            guild_reactions: true,
            guild_scheduled_events: true,
            guild_typing: true,
            guilds: true,
            integrations: true,
            invites: true,
            members: false,
            message_content: false,
            messages: true,
            moderation: true,
            polls: true,
            presences: false,
            reactions: true,
            typing: true,
            value: true,
            voice_states: true,
            webhooks: true,
        }
    }

    /// All disabled
    pub fn none() -> Self {
        Self {
            auto_moderation: false,
            auto_moderation_configuration: false,
            auto_moderation_execution: false,
            bans: false,
            dm_messages: false,
            dm_polls: false,
            dm_reactions: false,
            dm_typing: false,
            emojis: false,
            emojis_and_stickers: false,
            expressions: false,
            guild_messages: false,
            guild_polls: false,
            guild_reactions: false,
            guild_scheduled_events: false,
            guild_typing: false,
            guilds: false,
            integrations: false,
            invites: false,
            members: false,
            message_content: false,
            messages: false,
            moderation: false,
            polls: false,
            presences: false,
            reactions: false,
            typing: false,
            value: false,
            voice_states: false,
            webhooks: false,
        }
    }

    // ----------------------------
    // Convert to Discord bitmask
    // ----------------------------
    pub fn to_bitmask(&self) -> u32 {
        let mut v = 0;

        macro_rules! set_bit {
            ($field:expr, $bit:expr) => {
                if $field {
                    v |= 1 << $bit;
                }
            };
        }

        // Mapping based on Discord docs (v10)
        set_bit!(self.guilds, 0);
        set_bit!(self.members, 1);
        set_bit!(self.bans, 2);
        set_bit!(self.emojis_and_stickers, 3);
        set_bit!(self.integrations, 4);
        set_bit!(self.webhooks, 5);
        set_bit!(self.invites, 6);
        set_bit!(self.voice_states, 7);
        set_bit!(self.presences, 8);
        set_bit!(self.messages, 9);
        set_bit!(self.dm_messages, 12);
        set_bit!(self.dm_reactions, 13);
        set_bit!(self.dm_typing, 14);
        set_bit!(self.message_content, 15);

        // newer intents (some bots may not need these)
        set_bit!(self.auto_moderation_configuration, 20);
        set_bit!(self.auto_moderation_execution, 21);
        set_bit!(self.guild_scheduled_events, 16);

        v
    }

    // ----------------------------
    // Useful builder helpers
    // ----------------------------
    pub fn disable_members(mut self) -> Self {
        self.members = false;
        self
    }

    pub fn disable_message_content(mut self) -> Self {
        self.message_content = false;
        self
    }

    pub fn disable_presences(mut self) -> Self {
        self.presences = false;
        self
    }

    pub fn enable_message_content(mut self) -> Self {
        self.message_content = true;
        self
    }

    pub fn enable_members(mut self) -> Self {
        self.members = true;
        self
    }
}