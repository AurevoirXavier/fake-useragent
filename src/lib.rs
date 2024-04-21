use reqwest;
use select;

pub use user_agent_string::browser::Browsers;

mod user_agent_string;

// --- custom ---
use user_agent_string::UserAgentString;

pub struct UserAgents(Vec<String>);

impl UserAgents {
    pub fn new() -> UserAgents {
        // --- std ---
        use std::{
            env::current_dir,
            fs::File,
            io::Read,
            path::Path,
        };

        let path = format!("{}/user_agents", current_dir().unwrap().to_str().unwrap());
        let path = Path::new(&path);
        if path.is_file() {
            let mut file = File::open(path).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();

            UserAgents(s.lines().map(|user_agent| user_agent.to_owned()).collect())
        } else { UserAgentsBuilder::new().all_browsers().build() }
    }

    pub fn from_cache(path: &str) -> UserAgents {
        // --- std ---
        use std::{
            fs::File,
            io::Read,
        };

        let mut file = File::open(path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        UserAgents(s.lines().map(|user_agent| user_agent.to_owned()).collect())
    }

    pub fn random(&self) -> &str {
        // --- external ---
        use rand::{
            thread_rng,
            Rng,
        };

        &self.0[thread_rng().gen_range(0, self.0.len())]
    }
}

pub struct UserAgentsBuilder<'a> {
    cache: bool,
    thread: u32,
    dir: String,
    user_agents: Vec<String>,
    kinds: [Option<Box<dyn UserAgentString + 'a>>; 13],
}

macro_rules! set_unset {
    ($self_:ident, $setter:ident, $unsetter:ident, $i:expr) => {
        pub fn $setter(mut $self_, browsers: Browsers<'a>) -> Self {
            $self_.kinds[$i] = Some(Box::new(browsers));
            $self_
        }

        pub fn $unsetter(mut $self_) -> Self {
            $self_.kinds[$i] = None;
            $self_
        }
    };
}

impl<'a> UserAgentsBuilder<'a> {
    pub fn new() -> UserAgentsBuilder<'a> {
        UserAgentsBuilder {
            cache: true,
            thread: 20,
            dir: std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            user_agents: vec![],
            kinds: [None, None, None, None, None, None, None, None, None, None, None, None, None],
        }
    }

    pub fn all_browsers(mut self) -> Self {
        self.kinds[1] = Some(Box::new(Browsers::new().set_all()));
        self
    }
    set_unset!(self, set_browsers, unset_browsers, 1);


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

    pub fn cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    pub fn thread(mut self, num: u32) -> Self {
        if num >= 382 { self.thread = 382; } else if num > 0 { self.thread = num; }
        self
    }

    pub fn dir(mut self, path: &str) -> Self {
        self.dir = path.trim_end_matches('/').to_owned();
        self
    }

    fn store(&self) {
        // --- std ---
        use std::{
            fs::File,
            io::Write,
        };

        let mut file = File::create(format!("{}/user_agents", self.dir)).unwrap();
        file.write_all(self.user_agents.join("\n").as_bytes()).unwrap();
        file.sync_all().unwrap();
    }

    pub fn build(mut self) -> UserAgents {
        for user_agent_string in self.kinds.iter() {
            if let Some(user_agent_string) = user_agent_string {
                user_agent_string.fetch(self.thread, &mut self.user_agents);
            }
        }

        if self.cache { self.store(); }

        UserAgents(self.user_agents)
    }
}
