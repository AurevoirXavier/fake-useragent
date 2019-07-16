macro_rules! set_unset {
    ($self_:ident, $setter:ident, $unsetter:ident, $name:expr) => {
        pub fn $setter(mut $self_) -> Self {
            $self_.0.insert($name);
            $self_
        }

        pub fn $unsetter(mut $self_) -> Self {
            $self_.0.remove($name);
            $self_
        }
    };
}

pub mod browser;

// --- custom ---
use super::UserAgentsBuilder;

pub trait UserAgentString {
    fn to_vec(&self) -> Vec<&&str>;

    fn fetch(&self, thread: u32, user_agents: &mut Vec<String>) {
        for drivers_chunk in self.to_vec().chunks(thread as usize) {
            let mut handles = vec![];

            for driver in drivers_chunk {
                let url = format!("http://www.useragentstring.com/pages/useragentstring.php?name={}", driver);
                handles.push(std::thread::spawn(move || UserAgentsBuilder::parse(UserAgentsBuilder::get(&url))));
            }

            for handle in handles { user_agents.extend_from_slice(&handle.join().unwrap()); }
        }
    }
}
