
extern crate irc;
extern crate rand;

use std::borrow::Cow;
use std::default::Default;
use irc::client::prelude::*;


//const SERVER_ADDR: &'static str = "irc.freenode.org";
const SERVER_ADDR: &'static str = "irc.fyrechat.net";
const USERNAME: &'static str = "slackbot";
const CHANNELS: &'static [&'static str] = &["#test", "#flood"];
const RESPONSES: &'static [&'static str] = &[
    "fuck off",
    "do it yourself",
    "you're not my supervisor",
    "/me rolls over and goes back to sleep",
    "/me laughs",
];


fn random_response() -> &'static str {
    let index = rand::random::<usize>() % RESPONSES.len();
    RESPONSES[index]
}

fn respond(src: &str, dst: &str, msg: &str) -> Option<Cow<'static, str>> {
    /*
    if target == USERNAME {
        Some(Cow::Borrowed(random_response()))
    } else if msg.contains(USERNAME) {
        let resp = random_response();
        if let Some(src_nick) = src {
            if msg.starts_with(USERNAME) {
                Some(Cow::Owned(format!("{}: {}", src_nick, resp)))
            } else {
                Some(Cow::Borrowed(resp))
            }
        } else {
            Some(Cow::Borrowed(resp))
        }
    } else {
        None
    }
    */
    let resp = random_response();
    println!("New message: FROM '{}', TO '{}', MSG: '{}'", src, dst, msg);
    match (src, dst, msg.find(USERNAME)) {
        // don't respond to our own messgae
        (USERNAME, _, _) => None,
        // PM from someone
        (_, USERNAME, _) => Some(Cow::Borrowed(resp)),
        // message started with USERNAME
        (_, chan, Some(0)) if CHANNELS.contains(&chan) => 
            Some(Cow::Owned(format!("{}: {}", src, resp))),
        // message sorta started with USERNAME: ' /USERNAME' or something
        (_, chan, Some(i)) if i<3 && CHANNELS.contains(&chan) &&
                msg.trim_left().starts_with(|c| c=='/'||c=='\\') => 
            Some(Cow::Owned(format!("{}: {}", src, resp))),
        // someone said my name
        (_, chan, Some(_)) if CHANNELS.contains(&chan) => 
            Some(Cow::Borrowed(resp)),
        _ => None,
    }
}

fn run(cfg: Config) -> Result<(),irc::error::Error> { 
    let server = IrcServer::from_config(cfg)?;
    server.identify()?;

    server.for_each_incoming(|message| { 
        //println!("SAW:  `{:?}`", message);
        //if let (Command::PRIVMSG(ref target, ref msg), Some(src)) = (message.command, message.source_nickname()) {
        if let Command::PRIVMSG(ref dst, ref msg) = message.command {
            if let Some(src) = message.source_nickname() {
                if let Some(resp) = respond(src, dst, msg) {
                    println!("SENT: `{:?}`  in response to `{:?}`", resp, message);
                    match server.send_privmsg(dst, &resp) {
                        Err(e) => println!("Failed to respond: {:?}", e),
                        _ => {},
                    };
                }
            }
        }
    })
}

fn main() {
    let cfg = Config {
        nickname: Some(USERNAME.to_owned()),
        server:   Some(SERVER_ADDR.to_owned()),
        channels: Some(CHANNELS.iter().map(|&r| String::from(r)).collect()),
        .. Default::default()
    };

    run(cfg).unwrap();

}
