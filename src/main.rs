use dotenv::dotenv;
use poise::serenity_prelude::{self, model::gateway::Activity};
use tokio::time::{sleep, Duration};

mod satisfactory;
use satisfactory::{get_status, Players};

// Error type to be returned by methods
type Error = Box<dyn std::error::Error + Send + Sync>;

// Data shared by command invocations
#[derive(Clone)]
struct Data {
    satisfactory_server: String,
    satisfactory_token: String,
}
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let address = &ctx.data().satisfactory_server;
    let token = &ctx.data().satisfactory_token;
    let server_status = get_status(address, token).await?;

    ctx.send(|f| {
        f.embed(|e| {
            e.title("Status")
                .description(format!(
                    "{} of {} currently online.",
                    server_status.online, server_status.max
                ))
                .color(0x168064);
            e
        })
    })
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Ignore dotenv errors and try to continue anyway
    let _ = dotenv();

    let discord_token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let satisfactory_server =
        std::env::var("SATISFACTORY_SERVER").expect("missing SATISFACTORY_SERVER");
    let satisfactory_token =
        std::env::var("SATISFACTORY_TOKEN").expect("missing SATISFACTORY_TOKEN");

    let data = Data {
        satisfactory_server,
        satisfactory_token,
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![status()],
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            // Spawn the tasks that polls and updates the status in the background
            tokio::task::spawn(poll_status(ctx.clone(), data.clone()));
            // Register the discord commands with discord globally
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data.clone())
            })
        });

    let bot = framework.build().await.unwrap();

    bot.start().await.unwrap();
}

async fn poll_status(ctx: poise::serenity_prelude::Context, data: Data) {
    loop {
        let discord_status =
            match get_status(&data.satisfactory_server, &data.satisfactory_token).await {
                Err(_) => "an offline server".to_string(),
                Ok(Players { online, max }) => format!("{online} of {max} players"),
            };
        ctx.set_presence(
            Some(Activity::watching(discord_status)),
            serenity_prelude::OnlineStatus::Online,
        )
        .await;
        sleep(Duration::from_secs(60)).await;
    }
}
