
extern crate irc;
extern crate rand;

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
    "\u{1}ACTION rolls over and goes back to sleep\u{1}",
    "\u{1}ACTION laughs\u{1}",
];


fn random_response() -> &'static str {
    // TODO: special responses (e.g. format!("{}: {}", src, msg))
    let index = rand::random::<usize>() % RESPONSES.len();
    RESPONSES[index]
}

fn respond<'a>(src: Option<&'a str>, dst: &'a str, msg: &str) 
        -> Option<(&'a str, &'static str)> {
    // `src` is always the person who spoke
    // `dst` can be 'slackbot' or '#test'
    println!("SHOULD I RESPOND TO  '{:?}'  WHO SAID  '{}'  TO '{}'", src, msg, dst);
    // TODO: caps insensitivity?
    match (src, dst, msg.contains(USERNAME)) {
        // don't loop
        (Some(USERNAME), _, _) => None,
        // PM
        (Some(src), USERNAME, _) => Some((src, random_response())), 
        // mention
        (_, ref chan, true) if CHANNELS.contains(chan) => Some((dst, random_response())),
        _ => None,
    }

}

fn run(cfg: Config) -> Result<(),irc::error::Error> { 
    let server = IrcServer::from_config(cfg)?;
    server.identify()?;

    server.for_each_incoming(|message| { 
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if let Some((recip, resp)) = respond(message.source_nickname(), target, msg) {
                println!("SENT: `{}` to `{}` in response to `{:?}`", resp, recip, message);
                match server.send_privmsg(recip, resp) {
                    Err(e) => println!("Failed to respond: {:?}", e),
                    _ => {},
                };
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
