use clap::{App, AppSettings, Arg, SubCommand};

pub fn create_app<'a>() -> App<'a, 'a> {
    App::new(crate_name!())
        .setting(AppSettings::SubcommandsNegateReqs)
        .version("1.0")
        .author("Shaun Chong")
        .about("Commands resolver")
        .arg(Arg::with_name(""))
}