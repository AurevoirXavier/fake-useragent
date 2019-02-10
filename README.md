## Intro

**fake-useragent**

Inspired by python module [fake-useragent](https://github.com/hellysmile/fake-useragent)

Data from [useragentstring.com](http://useragentstring.com) (if failed to initialize, please make sure that you can access to this website)

## Example

```rust
extern crate fake_useragent;

fn main() {
    // quick start
    {
        use fake_useragent::UserAgents;
        
        let user_agents = UserAgents::new();
        println!("{}", user_agents.random());
    }
    
    // from cache
    {
        use fake_useragent::UserAgents;
        
        let user_agents = UserAgents::from_cache("/tmp/user_agents");
        println!("{}", user_agents.random());
    }
    
    // customize
    {
        use fake_useragent::{Browser, UserAgentsBuilder};
        
        let user_agent = UserAgentsBuilder::new()
            .cache(false)                 // specify save to file or not, default true
            .dir("/tmp")                  // specify store path, default dir `./`; default filename `user_agents`
            .thread(20)                   // specify fetch thread, default 20
            .set_browsers(Browsers::new() // specify browsers
                .set_chrome()
                .set_firefox()
                .set_safari())
            .build();
        println!("{}", user_agent.random());        
    }
}
```

## TODO

```rust
// selectable
let user_agents = UserAgents::new();
user_agents.chrome();
...

// more type
let user_agent = UserAgentsBuilder::new()
    .set_browsers(...)
    .set_crawlers(...)
    .set_link_checkers(...)
    .set_mobile_browsers(...)
    .build();
```
