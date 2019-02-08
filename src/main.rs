extern crate reqwest;
extern crate select;

const USER_AGENTS_PAGE_PREFIX: &'static str = "http://www.useragentstring.com/pages/useragentstring.php?name=";

enum Browser {
    ABrowse,
    AcooBrowser,
    AmericaOnlineBrowser,
    AmigaVoyager,
    AOL,
    Arora,
    AvantBrowser,
    Beonex,
    BonEcho,
    Browzar,
    Camino,
    Charon,
    Cheshire,
    Chimera,
    Chrome,
    ChromePlus,
    Classilla,
    CometBird,
    ComodoDragon,
    Conkeror,
    CrazyBrowser,
    Cyberdog,
    DeepnetExplorer,
    DeskBrowse,
    Dillo,
    Dooble,
    Edge,
    ElementBrowser,
    Elinks,
    EnigmaBrowser,
    EnigmaFox,
    Epiphany,
    Escape,
    Firebird,
    Firefox,
    FirewebNavigator,
    Flock,
    Fluid,
    Galaxy,
    Galeon,
    GranParadiso,
    GreenBrowser,
    Hana,
    HotJava,
    IBMWebExplorer,
    IBrowse,
    ICab,
    Iceape,
    IceCat,
    Iceweasel,
    INetBrowser,
    InternetExplorer,
    IRider,
    Iron,
    KMeleon,
    KNinja,
    Kapiko,
    Kazehakase,
    KindleBrowser,
    KKman,
    KMLite,
    Konqueror,
    LeechCraft,
    Links,
    Lobo,
    Lolifox,
    Lorentz,
    Lunascape,
    Lynx,
    Madfox,
    Maxthon,
    Midori,
    Minefield,
    Mozilla,
    Myibrow,
    MyIE2,
    Namoroka,
    Navscape,
    NCSAMosaic,
    NetNewsWire,
    NetPositive,
    Netscape,
    NetSurf,
    OmniWeb,
    Opera,
    Orca,
    Oregano,
    OsbBrowser,
    Palemoon,
    Phoenix,
    Pogo,
    Prism,
    QtWebInternetBrowser,
    Rekonq,
    Retawq,
    RockMelt,
    Safari,
    SeaMonkey,
    Shiira,
    Shiretoko,
    Sleipnir,
    SlimBrowser,
    Stainless,
    Sundance,
    Sunrise,
    Surf,
    Sylera,
    TencentTraveler,
    TenFourFox,
    TheWorldBrowser,
    Uzbl,
    Vimprobable,
    Vonkeror,
    W3M,
    WeltweitimnetzBrowser,
    WorldWideWeb,
    Wyzo,
}

impl Browser {
    fn api(&self) -> String {
        // --- custom ---
        use Browser::*;

        String::from(USER_AGENTS_PAGE_PREFIX) + match self {
            ABrowse => "ABrowse",
            AcooBrowser => "Acoo+Browser",
            AmericaOnlineBrowser => "America+Online+Browser",
            AmigaVoyager => "AmigaVoyager",
            AOL => "AOL",
            Arora => "Arora",
            AvantBrowser => "Avant+Browser",
            Beonex => "Beonex",
            BonEcho => "BonEcho",
            Browzar => "Browzar",
            Camino => "Camino",
            Charon => "Charon",
            Cheshire => "Cheshire",
            Chimera => "Chimera",
            Chrome => "Chrome",
            ChromePlus => "ChromePlus",
            Classilla => "Classilla",
            CometBird => "CometBird",
            ComodoDragon => "Comodo_Dragon",
            Conkeror => "Conkeror",
            CrazyBrowser => "Crazy+Browser",
            Cyberdog => "Cyberdog",
            DeepnetExplorer => "Deepnet+Explorer",
            DeskBrowse => "DeskBrowse",
            Dillo => "Dillo",
            Dooble => "Dooble",
            Edge => "Edge",
            ElementBrowser => "Element+Browser",
            Elinks => "Elinks",
            EnigmaBrowser => "Enigma+Browser",
            EnigmaFox => "EnigmaFox",
            Epiphany => "Epiphany",
            Escape => "Escape",
            Firebird => "Firebird",
            Firefox => "Firefox",
            FirewebNavigator => "Fireweb+Navigator",
            Flock => "Flock",
            Fluid => "Fluid",
            Galaxy => "Galaxy",
            Galeon => "Galeon",
            GranParadiso => "GranParadiso",
            GreenBrowser => "GreenBrowser",
            Hana => "Hana",
            HotJava => "HotJava",
            IBMWebExplorer => "IBM+WebExplorer",
            IBrowse => "IBrowse",
            ICab => "iCab",
            Iceape => "Iceape",
            IceCat => "IceCat",
            Iceweasel => "Iceweasel",
            INetBrowser => "iNet+Browser",
            InternetExplorer => "Internet+Explorer",
            IRider => "iRider",
            Iron => "Iron",
            KMeleon => "K-Meleon",
            KNinja => "K-Ninja",
            Kapiko => "Kapiko",
            Kazehakase => "Kazehakase",
            KindleBrowser => "Kindle+Browser",
            KKman => "KKman",
            KMLite => "KMLite",
            Konqueror => "Konqueror",
            LeechCraft => "LeechCraft",
            Links => "Links",
            Lobo => "Lobo",
            Lolifox => "lolifox",
            Lorentz => "Lorentz",
            Lunascape => "Lunascape",
            Lynx => "Lynx",
            Madfox => "Madfox",
            Maxthon => "Maxthon",
            Midori => "Midori",
            Minefield => "Minefield",
            Mozilla => "Mozilla",
            Myibrow => "myibrow",
            MyIE2 => "MyIE2",
            Namoroka => "Namoroka",
            Navscape => "Navscape",
            NCSAMosaic => "NCSA_Mosaic",
            NetNewsWire => "NetNewsWire",
            NetPositive => "NetPositive",
            Netscape => "Netscape",
            NetSurf => "NetSurf",
            OmniWeb => "OmniWeb",
            Opera => "Opera",
            Orca => "Orca",
            Oregano => "Oregano",
            OsbBrowser => "osb-browser",
            Palemoon => "Palemoon",
            Phoenix => "Phoenix",
            Pogo => "Pogo",
            Prism => "Prism",
            QtWebInternetBrowser => "QtWeb+Internet+Browser",
            Rekonq => "Rekonq",
            Retawq => "retawq",
            RockMelt => "RockMelt",
            Safari => "Safari",
            SeaMonkey => "SeaMonkey",
            Shiira => "Shiira",
            Shiretoko => "Shiretoko",
            Sleipnir => "Sleipnir",
            SlimBrowser => "SlimBrowser",
            Stainless => "Stainless",
            Sundance => "Sundance",
            Sunrise => "Sunrise",
            Surf => "surf",
            Sylera => "Sylera",
            TencentTraveler => "Tencent+Traveler",
            TenFourFox => "TenFourFox",
            TheWorldBrowser => "theWorld+Browser",
            Uzbl => "uzbl",
            Vimprobable => "Vimprobable",
            Vonkeror => "Vonkeror",
            W3M => "w3m",
            WeltweitimnetzBrowser => "WeltweitimnetzBrowser",
            WorldWideWeb => "WorldWideWeb",
            Wyzo => "Wyzo",
        }
    }
}

enum Kind {
    Crawlers,
    Browsers(Option<Browser>),
    MobileBrowsers,
    Consoles,
    OfflineBrowsers,
    EmailClients,
    LinkCheckers,
    EmailCollectors,
    Validators,
    FeedReaders,
    Libraries,
    CloudPlatforms,
    Others,
}

struct UserAgent<'a> {
    cache: bool,
    path: &'a str,
    amount: Option<u32>,
    user_agents: Vec<String>,
    kinds: Option<Vec<Kind>>,
}

impl<'a> UserAgent<'a> {
    fn new() -> UserAgent<'a> {
        UserAgent {
            cache: false,
            path: ".",
            amount: None,
            user_agents: vec![],
            kinds: None,
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

    fn parse_user_agents_page(html: String) -> Vec<String> {
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

    fn parse_list_page(html: String) -> Vec<&str> {
        unimplemented!()
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

    fn amount(mut self, amount: u32) -> Self {
        self.amount = Some(amount);
        self
    }

    fn kind(mut self, kind: Kind) -> Self {
        self.kinds = Some(vec![kind]);
        self
    }

    fn kinds(mut self, kinds: Vec<Kind>) -> Self {
        self.kinds = Some(kinds);
        self
    }

    fn fetch(mut self) -> Self {
        if let Some(kinds) = &self.kinds {
            for kind in kinds {
                // --- custom ---
                use Kind::*;

                let url = match kind {
                    Crawlers => unimplemented!(),
                    Browsers(browser) => if let Some(browser) = browser { browser.api() } else {

                    }
                    MobileBrowsers => unimplemented!(),
                    Consoles => unimplemented!(),
                    OfflineBrowsers => unimplemented!(),
                    EmailClients => unimplemented!(),
                    LinkCheckers => unimplemented!(),
                    EmailCollectors => unimplemented!(),
                    Validators => unimplemented!(),
                    FeedReaders => unimplemented!(),
                    Libraries => unimplemented!(),
                    CloudPlatforms => unimplemented!(),
                    Others => unimplemented!(),
                };
                let html = UserAgent::get(&url);
                let user_agents = UserAgent::parse_user_agents_page(html);

                self.user_agents.extend_from_slice(&user_agents);
            }
        } else {
            let html = UserAgent::get("http://www.useragentstring.com/pages/useragentstring.php");
            let user_agent_page = UserAgent::parse_lit_page(&html);
        }

        if self.cache {
            // --- std ---
            use std::{
                fs::File,
                io::Write,
            };

            let mut file = File::create(format!("{}/user_agents", self.path.trim_end_matches('/'))).unwrap();
            file.write_all(self.user_agents.join("\n").as_bytes()).unwrap();
            file.sync_all().unwrap();
        }

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
    let mut useragent = UserAgent::new()
//        .load_cache("./user_agents");
        .kind(Kind::Browsers(Some(Browser::Chrome)))
        .cache(true)
        .fetch();

    println!("{}", useragent.random());
}
