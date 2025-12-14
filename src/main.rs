use discord;
use serde_json::json;

#[tokio::main]
async fn main() {
    let intents = discord::Intents::all();
    let mut bot = discord::commands::bot::new(intents);

    println!("Bot initialized:");

    // Add a test slash command
    bot.add_slash_command(
        "ping",
        "Responds with pong",
        vec![],
        |_interaction, ctx| {
            tokio::spawn(async move {
                println!("Ping command received!");
                ctx.respond("Pong!").await;
            });
        },
    );

    // Add another test slash command with options
    bot.add_slash_command(
        "greet",
        "Greets a user",
        vec![json!({
            "name": "user",
            "description": "The user to greet",
            "type": 6,
            "required": true
        })],
        |_interaction, ctx| {
            tokio::spawn(async move {
                println!("Greet command received!");
                ctx.defer().await;
                
                // Extract the User object from the context's interaction data
                if let Some(user) = ctx.get_user(&ctx.interaction_data) {
                    let message = format!("{} has been greeted! 👋", user.mention());
                    ctx.followup(&message).await;
                } else {
                    ctx.respond("No user was provided!").await;
                }
            });
        },
    );

    let token = "MTQ0NjYzODQ3NjkyODg3NjcwNg.GAu6N8.c-uhN3a7t13Jr9EjCuHNBIPKpEjXJBG7P6ZmSk".to_string();

    bot.run(token).await;
}
