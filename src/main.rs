extern crate serenity;

use serenity::client::Client;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    // Configure the client with your Discord bot token in the environment
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token);

    client.on_message(|_, message| {
        // Configure minecraft_service_name in the environment
        let service = env::var("MINECRAFT_SERVICE_NAME")
            .expect("Expected a minecraft systemd service in the environment");
        // start minecraft server by systemd
        if message.content == "!start" {
            message.channel_id.say("サービス開始します...").unwrap();
            // excute unix command
            let mut process_start = Command::new("bash")
                    .arg("-c")
                    .arg(format!("{} {}", "sudo systemctl start", &service))
                    .stdout(Stdio::piped())
                    .spawn().expect("failed to run systemctl start");
            process_start.wait().unwrap();
            message.channel_id.say("だん! 開始しました:punch:").unwrap();
        }
        // stop minecraft server by systemd
        if message.content == "!stop" {
            message.channel_id.say("サービス停止します...").unwrap();
            // excute unix command
            let mut process_start = Command::new("bash")
                    .arg("-c")
                    .arg(format!("{} {}", "sudo systemctl stop", &service))
                    .stdout(Stdio::piped())
                    .spawn().expect("failed to run systemctl start");
            process_start.wait().unwrap();
            message.channel_id.say("だん！ 停止しました:punch:").unwrap();
        }
    });

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
