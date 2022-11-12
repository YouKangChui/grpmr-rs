use teloxide::utils::command::BotCommand;
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Ban a user")]
    Ban,
    #[command(description = "Bans a user for sometime")]
  //  Tban,
//    #[command(description = "Unbans a user")]
    Unban,
    #[command(description = "Mute a user")]
    Mute,
    #[command(description = "Mutes user for some time")]
    Tmute,
    #[command(description = "Unmute a user")]
    Unmute,
    #[command(description = "Greeting a user who sends /start")]
    Start,
    #[command(description = "Helps with available commands")]
    Help,
    #[command(description = "Kick a user from the group")]
//    Kick,
//    #[command(description = "Sends info about a user")]
    Info,
    #[command(description = "Send's user's or chat's ID")]
    Id,
    #[command(description = "Kick yourself from a group")]
    Kickme,
    #[command(description = "Pins a message")]
    Pin,
    #[command(description = "Unpins a mentioned message")]
    Unpin,
    #[command(description = "Promotes a user")]
    Promote,
    #[command(description = "Demotes a user")]
    Demote,
    #[command(description = "Get's the invite link of the chat")]
    Invitelink,
    #[command(description = "Get's list of admins in a group")]
    Adminlist,
    #[command(description = "Delete bulk of messages in a group")]
    Purge,
    #[command(description = "Deletes a message")]
    Del,
    #[command(description = "Leaves a chat (Owner use only)")]
    Leavechat,
    #[command(description = "Urban Dictionary")]
    Ud,
    #[command(description = "Pastes text to dogbin")]
    Paste,
    #[command(description = "Echoes same thing basically")]
    Echo,
    #[command(description = "Changes chat permissons")]
    Lock,
    #[command(description = "Unlocks locked permissions in a chat")]
    Unlock,
    #[command(description = "Available locktypes")]
    Locktypes,
    #[command(description = "Get's list of chats bot is in")]
    Chatlist,
    #[command(description = "Gban a user")]
    Gban,
    #[command(description = "Ungban a user")]
    Ungban,
    #[command(description = "Turns Gbans for a group on/off")]
    Gbanstat,
    #[command(description = "Warns a user")]
    Warn,
    #[command(description = "Sets warn limit")]
    Warnlimit,
    #[command(description = "Sets warn mode")]
    Warnmode,
    #[command(description = "Resets user's warnings")]
    Resetwarns,
    #[command(description = "Counts the number of warnings")]
    Warns,
    #[command(description = "Disables a command")]
    Disable,
    #[command(description = "Enables a command")]
    Enable,
    #[command(description = "Add filter for a keyword")]
    Filter,
    #[command(description = "list filters")]
    Filters,
    #[command(description = "Stop the use of a filter")]
    Stop,
    #[command(description = "Add a blacklist word")]
    Addblacklist,
    #[command(description = "Remove a blacklist word")]
    Rmblacklist,
    #[command(description = "List current blacklists in the group")]
    Blacklists,
    #[command(description = "Set a blacklist mode")]
    Blacklistmode,
    #[command(description = "Set chat picture")]
    Setchatpic,
    #[command(description = "Set chat title")]
    Setchattitle,
    #[command(description = "Set a log channel")]
    Setlog,
    #[command(description = "Unset a log channel")]
    Unsetlog,
    #[command(description = "Report a user")]
    Report,
    #[command(description = "Report setting of a group")]
    Reports,
}
