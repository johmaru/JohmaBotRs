mod commands;


use poise::{async_trait, PrefixFrameworkOptions, serenity_prelude as serenity};
use poise::serenity_prelude::EventHandler;
use songbird::SerenityInit;

struct Data{}
struct Handler;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a,Data,Error>;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[poise::command(slash_command, prefix_command)]
async fn greet(
    ctx: Context<'_>,
    #[description = "The user to greet"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| &ctx.author());
    let response = format!("Hello, {}!", user.name);
    ctx.say(response).await?;
    Ok(()) }

#[poise::command(slash_command, prefix_command)]
async fn lol_latest_patch(ctx: Context<'_>,

) -> Result<(), Error>
{
    let client = reqwest::Client::new();
    let response: Vec<String> = client.get("https://ddragon.leagueoflegends.com/api/versions.json").send().await?.json().await?;
    let patch = response[0].clone();
    ctx.say(patch).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn patch_note(ctx: Context<'_>,

) -> Result<(), Error>
{
    let client = reqwest::Client::new();
    let response: Vec<String> = client.get("https://ddragon.leagueoflegends.com/api/versions.json").send().await?.json().await?;
    let patch = response[0].clone();
    let latest_patch = &patch[0..4];
   let x = latest_patch.replace(".", "-");
    let url = format!("https://na.leagueoflegends.com/en-us/news/game-updates/patch-{}-notes/", x);
    ctx.say(url).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("~".to_string()),
                ..Default::default()
            },
            commands: vec![greet(), lol_latest_patch(), patch_note()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands
                ).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::Client::builder(&token, serenity::GatewayIntents::non_privileged())
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await;
        client.unwrap().start().await.unwrap();
}