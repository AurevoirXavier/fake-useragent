extern crate reqwest;
extern crate select;

const API: &'static str = "http://www.useragentstring.com/pages/useragentstring.php?name=";

mod browser;

// --- custom ---
use browser::Browsers;

enum Kind<'a> {
//    Crawlers,
    Browsers(Browsers<'a>),
//    MobileBrowsers,
//    Consoles,
//    OfflineBrowsers,
//    EmailClients,
//    LinkCheckers,
//    EmailCollectors,
//    Validators,
//    FeedReaders,
//    Libraries,
//    CloudPlatforms,
//    Others,
}

struct UserAgent<'a> {
    cache: bool,
    path: &'a str,
    user_agents: Vec<String>,
    kinds: Vec<Kind<'a>>,
}

impl<'a> UserAgent<'a> {
    fn new() -> UserAgent<'a> {
        UserAgent {
            cache: false,
            path: ".",
            user_agents: vec![],
            kinds: vec![],
        }
    }

    fn get(url: &str) -> String {
        let client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .gzip(true)
            .build()
            .unwrap();

        loop {
            match client.get(url).send() {
                Ok(mut resp) => match resp.text() {
                    Ok(html) => return html,
                    Err(e) => println!("{:?}", e)
                }
                Err(e) => println!("{:?}", e)
            }
        }
    }

    fn parse(html: String) -> Vec<String> {
        // --- external ---
        use select::{
            document::Document,
            predicate::{Attr, Name, Predicate},
        };

        let mut user_agents = vec![];
        let document = Document::from(html.as_str());
        for node in document.find(Attr("id", "liste").descendant(Name("ul"))) {
            for user_agent in node.find(Name("li").descendant(Name("a"))) {
                user_agents.push(user_agent.text());
            }
        }

        user_agents
    }

    fn cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    fn load_cache(mut self, path: &str) -> Self {
        // --- std ---
        use std::{
            fs::File,
            io::Read,
        };

        let mut file = File::open(path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        self.user_agents = s.lines().map(|user_agent| user_agent.to_owned()).collect();
        self
    }

    fn path(mut self, path: &'a str) -> Self {
        self.path = path;
        self
    }

    fn kind(mut self, kind: Kind<'a>) -> Self {
        self.kinds = vec![kind];
        self
    }

    fn kinds(mut self, kinds: Vec<Kind<'a>>) -> Self {
        self.kinds = kinds;
        self
    }

    fn store(&self) {
        // --- std ---
        use std::{
            fs::File,
            io::Write,
        };

        let mut file = File::create(format!("{}/user_agents", self.path.trim_end_matches('/'))).unwrap();
        file.write_all(self.user_agents.join("\n").as_bytes()).unwrap();
        file.sync_all().unwrap();
    }

    fn fetch(mut self) -> Self {
        for kind in &self.kinds {
            // --- custom ---
            use Kind::*;

            match kind {
//                Crawlers => unimplemented!(),
                Browsers(browsers) => for browser in browsers.iter() {
                    println!("{}", browser);
                    let html = UserAgent::get(&format!("{}{}", API, browser));
                    let user_agents = UserAgent::parse(html);

                    self.user_agents.extend_from_slice(&user_agents);
                }
//                MobileBrowsers => unimplemented!(),
//                Consoles => unimplemented!(),
//                OfflineBrowsers => unimplemented!(),
//                EmailClients => unimplemented!(),
//                LinkCheckers => unimplemented!(),
//                EmailCollectors => unimplemented!(),
//                Validators => unimplemented!(),
//                FeedReaders => unimplemented!(),
//                Libraries => unimplemented!(),
//                CloudPlatforms => unimplemented!(),
//                Others => unimplemented!(),
            };
        }

        if self.cache { self.store(); }

        self
    }

    fn random(&self) -> &str {
        // --- external ---
        use rand::{
            thread_rng,
            Rng,
        };

        &self.user_agents[thread_rng().gen_range(0, self.user_agents.len())]
    }
}

fn main() {
    let useragent = UserAgent::new()
//        .load_cache("./user_agents");
        .kind(Kind::Browsers(Browsers::new().set_all()))
        .cache(true)
        .fetch();

    println!("{}", useragent.random());
}
