extern crate fake_useragent;

// --- external ---
use fake_useragent::{Browsers, UserAgents, UserAgentsBuilder};

fn main() {
    let user_agent = UserAgentsBuilder::new()
        .cache()
        .dir()
        .thread()
        .set_browsers(Browsers::new()
            .set_chrome()
            .set_firefox())
        .build();
    let user_agent = UserAgents::new();
    println!("{}", user_agent.random());
}
