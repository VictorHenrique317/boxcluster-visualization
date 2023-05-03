use indicatif::{ProgressBar, ProgressStyle};

pub fn new(total: u64, message: &str) -> ProgressBar{
    let bar = ProgressBar::new(total);
        bar.set_message(message.to_owned());
        bar.set_style(ProgressStyle::with_template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7}| Elapsed time: {elapsed} | Estimated time:{eta}")
            .unwrap()
            .progress_chars("=>-"));
    return bar;
}