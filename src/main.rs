use dotenv::dotenv;
use poise::{serenity_prelude as serenity, CreateReply};
use std::env;
use tokio::time::{sleep, Duration};

mod satisfactory;

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
    let server_status = satisfactory::get_status(address, token).await?;

    let reply = CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Status")
            .description(format!(
                "{} of {} currently online.",
                server_status.online, server_status.max
            ))
            .color(0x168064),
    );
    ctx.send(reply).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Ignore dotenv errors and try to continue anyway
    let _ = dotenv();

    let discord_token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let satisfactory_server = env::var("SATISFACTORY_SERVER").expect("missing SATISFACTORY_SERVER");
    let satisfactory_token = env::var("SATISFACTORY_TOKEN").expect("missing SATISFACTORY_TOKEN");

    let data = Data {
        satisfactory_server,
        satisfactory_token,
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![status()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            // Spawn the tasks that polls and updates the status in the background
            tokio::task::spawn(poll_status(ctx.clone(), data.clone()));
            // Register the discord commands with discord globally
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data.clone())
            })
        })
        .build();

    let mut client =
        serenity::ClientBuilder::new(discord_token, serenity::GatewayIntents::non_privileged())
            .framework(framework)
            .await
            .unwrap();

    client.start().await.unwrap()
}

async fn poll_status(ctx: serenity::Context, data: Data) {
    loop {
        let discord_status =
            match satisfactory::get_status(&data.satisfactory_server, &data.satisfactory_token)
                .await
            {
                Err(_) => "an offline server".to_string(),
                Ok(satisfactory::Players { online, max }) => format!("{online} of {max} players"),
            };
        ctx.set_presence(
            Some(serenity::ActivityData::watching(discord_status)),
            serenity::OnlineStatus::Online,
        );
        sleep(Duration::from_secs(60)).await;
    }
}
