use indicatif::{ProgressBar, ProgressStyle};
use std::env;

pub fn new(total: u64, message: &str) -> ProgressBar {
    let hide_progress = env::var("HIDE_PROGRESS").is_ok();
    let bar = if hide_progress {
        ProgressBar::hidden()
    } else {
        let bar = ProgressBar::new(total);
        bar.set_message(message.to_owned());
        bar.set_style(ProgressStyle::default_bar()
            .template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7} | Elapsed time: {elapsed} | Estimated time:{eta}").unwrap()
            .progress_chars("=>-"));
        bar
    };

    return bar;
}
