# Satisfactory Server Status Bot (SSSB)
This is a simple [discord](https://discord.com/) bot that displays the player status (how many are online and the max amount of players) 
for a [Satisfactory](https://www.satisfactorygame.com/) dedicated server by using the built in HTTPS API.

## Setup
1. Download the bot executable from the [releases page](https://github.com/goof-bug/satisfactory-server-status-bot/releases)
2. Make a `.env` file by copying the `.env.example`: `$ cp .env.example .env`
3. Fill in the values
   - The `DISCORD_TOKEN` variable you can get by making a bot at https://discord.com/developers/applications
   - The `SATISFACTORY_SERVER` variable is the host of the satisfactory server including the port (default is 7777), example: `satisfactory.example.org:7777`
   - The `SATISFACTORY_TOKEN` you need to generate by executing the `server.GenerateAPIToken` command in the satisfactory dedicated server console
4. Run the bot!
