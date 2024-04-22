// --- std ---
use std::collections::HashSet;

const BROWSERS: [&str; 117] = [
    "ABrowse",
    "Acoo+Browser",
    "America+Online+Browser",
    "AmigaVoyager",
    "AOL",
    "Arora",
    "Avant+Browser",
    "Beonex",
    "BonEcho",
    "Browzar",
    "Camino",
    "Charon",
    "Cheshire",
    "Chimera",
    "Chrome",
    "ChromePlus",
    "Classilla",
    "CometBird",
    "Comodo_Dragon",
    "Conkeror",
    "Crazy+Browser",
    "Cyberdog",
    "Deepnet+Explorer",
    "DeskBrowse",
    "Dillo",
    "Dooble",
    "Edge",
    "Element+Browser",
    "Elinks",
    "Enigma+Browser",
    "EnigmaFox",
    "Epiphany",
    "Escape",
    "Firebird",
    "Firefox",
    "Fireweb+Navigator",
    "Flock",
    "Fluid",
    "Galaxy",
    "Galeon",
    "GranParadiso",
    "GreenBrowser",
    "Hana",
    "HotJava",
    "IBM+WebExplorer",
    "IBrowse",
    "iCab",
    "Iceape",
    "IceCat",
    "Iceweasel",
    "iNet+Browser",
    "Internet+Explorer",
    "iRider",
    "Iron",
    "K-Meleon",
    "K-Ninja",
    "Kapiko",
    "Kazehakase",
    "Kindle+Browser",
    "KKman",
    "KMLite",
    "Konqueror",
    "LeechCraft",
    "Links",
    "Lobo",
    "lolifox",
    "Lorentz",
    "Lunascape",
    "Lynx",
    "Madfox",
    "Maxthon",
    "Midori",
    "Minefield",
    "Mozilla",
    "myibrow",
    "MyIE2",
    "Namoroka",
    "Navscape",
    "NCSA_Mosaic",
    "NetNewsWire",
    "NetPositive",
    "Netscape",
    "NetSurf",
    "OmniWeb",
    "Opera",
    "Orca",
    "Oregano",
    "osb-browser",
    "Palemoon",
    "Phoenix",
    "Pogo",
    "Prism",
    "QtWeb+Internet+Browser",
    "Rekonq",
    "retawq",
    "RockMelt",
    "Safari",
    "SeaMonkey",
    "Shiira",
    "Shiretoko",
    "Sleipnir",
    "SlimBrowser",
    "Stainless",
    "Sundance",
    "Sunrise",
    "surf",
    "Sylera",
    "Tencent+Traveler",
    "TenFourFox",
    "theWorld+Browser",
    "uzbl",
    "Vimprobable",
    "Vonkeror",
    "w3m",
    "WeltweitimnetzBrowser",
    "WorldWideWeb",
    "Wyzo",
];

pub struct Browsers<'a>(HashSet<&'a str>);

impl<'a> Default for Browsers<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Browsers<'a> {
    pub fn new() -> Browsers<'a> {
        Browsers(HashSet::new())
    }

    set_unset!(self, set_a_browser, unset_a_browser, "ABrowse");
    set_unset!(self, set_acoo_browser, unset_acoo_browser, "Acoo+Browser");
    set_unset!(
        self,
        set_america_online_browser,
        unset_america_online_browser,
        "America+Online+Browser"
    );
    set_unset!(self, set_amiga_voyager, unset_amiga_voyager, "AmigaVoyager");
    set_unset!(self, set_aol, unset_aol, "AOL");
    set_unset!(self, set_arora, unset_arora, "Arora");
    set_unset!(
        self,
        set_avant_browser,
        unset_avant_browser,
        "Avant+Browser"
    );
    set_unset!(self, set_beonex, unset_beonex, "Beonex");
    set_unset!(self, set_bon_echo, unset_bon_echo, "BonEcho");
    set_unset!(self, set_browzar, unset_browzar, "Browzar");
    set_unset!(self, set_camino, unset_camino, "Camino");
    set_unset!(self, set_charon, unset_charon, "Charon");
    set_unset!(self, set_cheshire, unset_cheshire, "Cheshire");
    set_unset!(self, set_chimera, unset_chimera, "Chimera");
    set_unset!(self, set_chrome, unset_chrome, "Chrome");
    set_unset!(self, set_chrome_plus, unset_chrome_plus, "ChromePlus");
    set_unset!(self, set_classilla, unset_classilla, "Classilla");
    set_unset!(self, set_comet_bird, unset_comet_bird, "CometBird");
    set_unset!(
        self,
        set_comodo_dragon,
        unset_comodo_dragon,
        "Comodo_Dragon"
    );
    set_unset!(self, set_conkeror, unset_conkeror, "Conkeror");
    set_unset!(
        self,
        set_crazy_browser,
        unset_crazy_browser,
        "Crazy+Browser"
    );
    set_unset!(self, set_cyberdog, unset_cyberdog, "Cyberdog");
    set_unset!(
        self,
        set_deepnet_explorer,
        unset_deepnet_explorer,
        "Deepnet+Explorer"
    );
    set_unset!(self, set_desk_browse, unset_desk_browse, "DeskBrowse");
    set_unset!(self, set_dillo, unset_dillo, "Dillo");
    set_unset!(self, set_dooble, unset_dooble, "Dooble");
    set_unset!(self, set_edge, unset_edge, "Edge");
    set_unset!(
        self,
        set_element_browser,
        unset_element_browser,
        "Element+Browser"
    );
    set_unset!(self, set_elinks, unset_elinks, "Elinks");
    set_unset!(
        self,
        set_enigma_browser,
        unset_enigma_browser,
        "Enigma+Browser"
    );
    set_unset!(self, set_enigma_fox, unset_enigma_fox, "EnigmaFox");
    set_unset!(self, set_epiphany, unset_epiphany, "Epiphany");
    set_unset!(self, set_escape, unset_escape, "Escape");
    set_unset!(self, set_firebird, unset_firebird, "Firebird");
    set_unset!(self, set_firefox, unset_firefox, "Firefox");
    set_unset!(
        self,
        set_fireweb_navigator,
        unset_fireweb_navigator,
        "Fireweb+Navigator"
    );
    set_unset!(self, set_flock, unset_flock, "Flock");
    set_unset!(self, set_fluid, unset_fluid, "Fluid");
    set_unset!(self, set_galaxy, unset_galaxy, "Galaxy");
    set_unset!(self, set_galeon, unset_galeon, "Galeon");
    set_unset!(self, set_gran_paradiso, unset_gran_paradiso, "GranParadiso");
    set_unset!(self, set_green_browser, unset_green_browser, "GreenBrowser");
    set_unset!(self, set_hana, unset_hana, "Hana");
    set_unset!(self, set_hot_java, unset_hot_java, "HotJava");
    set_unset!(
        self,
        set_ibm_web_explorer,
        unset_ibm_web_explorer,
        "IBM+WebExplorer"
    );
    set_unset!(self, set_i_browse, unset_i_browse, "IBrowse");
    set_unset!(self, set_i_cab, unset_i_cab, "iCab");
    set_unset!(self, set_iceape, unset_iceape, "Iceape");
    set_unset!(self, set_ice_cat, unset_ice_cat, "IceCat");
    set_unset!(self, set_iceweasel, unset_iceweasel, "Iceweasel");
    set_unset!(self, set_i_net_browser, unset_i_net_browser, "iNet+Browser");
    set_unset!(
        self,
        set_internet_explorer,
        unset_internet_explorer,
        "Internet+Explorer"
    );
    set_unset!(self, set_i_rider, unset_i_rider, "iRider");
    set_unset!(self, set_iron, unset_iron, "Iron");
    set_unset!(self, set_k_meleon, unset_k_meleon, "K-Meleon");
    set_unset!(self, set_k_ninja, unset_k_ninja, "K-Ninja");
    set_unset!(self, set_kapiko, unset_kapiko, "Kapiko");
    set_unset!(self, set_kazehakase, unset_kazehakase, "Kazehakase");
    set_unset!(
        self,
        set_kindle_browser,
        unset_kindle_browser,
        "Kindle+Browser"
    );
    set_unset!(self, set_kk_man, unset_kk_man, "KKman");
    set_unset!(self, set_km_lite, unset_km_lite, "KMLite");
    set_unset!(self, set_kongueror, unset_kongueror, "Konqueror");
    set_unset!(self, set_leech_craft, unset_leech_craft, "LeechCraft");
    set_unset!(self, set_links, unset_links, "Links");
    set_unset!(self, set_lobo, unset_lobo, "Lobo");
    set_unset!(self, set_lolifox, unset_lolifox, "lolifox");
    set_unset!(self, set_lorentz, unset_lorentz, "Lorentz");
    set_unset!(self, set_lunascape, unset_lunascape, "Lunascape");
    set_unset!(self, set_lynx, unset_lynx, "Lynx");
    set_unset!(self, set_madfox, unset_madfox, "Madfox");
    set_unset!(self, set_maxthon, unset_maxthon, "Maxthon");
    set_unset!(self, set_midori, unset_midori, "Midori");
    set_unset!(self, set_minefield, unset_minefield, "Minefield");
    set_unset!(self, set_mozilla, unset_mozilla, "Mozilla");
    set_unset!(self, set_myibrow, unset_myibrow, "myibrow");
    set_unset!(self, set_my_ie2, unset_my_ie2, "MyIE2");
    set_unset!(self, set_namoroka, unset_namoroka, "Namoroka");
    set_unset!(self, set_navscape, unset_navscape, "Navscape");
    set_unset!(self, set_ncsa_mosaic, unset_ncsa_mosaic, "NCSA_Mosaic");
    set_unset!(self, set_net_news_wire, unset_net_news_wire, "NetNewsWire");
    set_unset!(self, set_net_positive, unset_net_positive, "NetPositive");
    set_unset!(self, set_net_scape, unset_net_scape, "Netscape");
    set_unset!(self, set_net_surf, unset_net_surf, "NetSurf");
    set_unset!(self, set_omni_web, unset_omni_web, "OmniWeb");
    set_unset!(self, set_opera, unset_opera, "Opera");
    set_unset!(self, set_orca, unset_orca, "Orca");
    set_unset!(self, set_oregano, unset_oregano, "Oregano");
    set_unset!(self, set_osb_browser, unset_osb_browser, "osb-browser");
    set_unset!(self, set_palemoon, unset_palemoon, "Palemoon");
    set_unset!(self, set_phoenix, unset_phoenix, "Phoenix");
    set_unset!(self, set_pogo, unset_pogo, "Pogo");
    set_unset!(self, set_prism, unset_prism, "Prism");
    set_unset!(
        self,
        set_qt_web_internet_browser,
        unset_qt_web_internet_browser,
        "QtWeb+Internet+Browser"
    );
    set_unset!(self, set_rekona, unset_rekona, "Rekonq");
    set_unset!(self, set_retawq, unset_retawq, "retawq");
    set_unset!(self, set_rock_melt, unset_rock_melt, "RockMelt");
    set_unset!(self, set_safari, unset_safari, "Safari");
    set_unset!(self, set_sea_monkey, unset_sea_monkey, "SeaMonkey");
    set_unset!(self, set_shiira, unset_shiira, "Shiira");
    set_unset!(self, set_shiretoko, unset_shiretoko, "Shiretoko");
    set_unset!(self, set_sleipnir, unset_sleipnir, "Sleipnir");
    set_unset!(self, set_slim_browser, unset_slim_browser, "SlimBrowser");
    set_unset!(self, set_stainless, unset_stainless, "Stainless");
    set_unset!(self, set_sundance, unset_sundance, "Sundance");
    set_unset!(self, set_surf, unset_surf, "surf");
    set_unset!(self, set_sylera, unset_sylera, "Sylera");
    set_unset!(
        self,
        set_tecent_traveler,
        unset_tecent_traveler,
        "Tencent+Traveler"
    );
    set_unset!(self, set_ten_four_fox, unset_ten_four_fox, "TenFourFox");
    set_unset!(
        self,
        set_the_world_browser,
        unset_the_world_browser,
        "theWorld+Browser"
    );
    set_unset!(self, set_uzbl, unset_uzbl, "uzbl");
    set_unset!(self, set_vimprobable, unset_vimprobable, "Vimprobable");
    set_unset!(self, set_vonkeror, unset_vonkeror, "Vonkeror");
    set_unset!(self, set_w3m, unset_w3m, "w3m");
    set_unset!(
        self,
        set_weltweitimnetz_browser,
        unset_weltweitimnetz_browser,
        "WeltweitimnetzBrowser"
    );
    set_unset!(
        self,
        set_world_wide_web,
        unset_world_wide_web,
        "WorldWideWeb"
    );
    set_unset!(self, set_wyzo, unset_wyzo, "Wyzo");
    pub fn set_all(mut self) -> Self {
        self.0 = BROWSERS.iter().cloned().collect();
        self
    }
    pub fn unset_all(mut self) -> Self {
        self.0 = HashSet::new();
        self
    }
}

impl<'a> super::UserAgentString for Browsers<'a> {
    fn to_vec(&self) -> Vec<&&str> {
        self.0.iter().collect()
    }
}
