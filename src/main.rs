use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use cursive::align::HAlign;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, SelectView, TextView};
use cursive::Cursive;

fn main() {
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    let content = include_str!("list");
    select.add_all_str(content.lines());

    select.set_on_submit(show_choice);

    let mut root = cursive::default();

    root.add_layer(
        Dialog::around(select.scrollable().fixed_size((30, 20))).title("Choose .gitignore"),
    );

    root.add_global_callback(Key::Esc, |s| s.quit());

    root.run();
}

fn show_choice(root: &mut Cursive, template: &str) {
    root.pop_layer();

    let text = format!(
        "Writing .gitignore for {} in a current directory!",
        template
    );

    download(template).unwrap();

    root.add_layer(Dialog::around(TextView::new(text)).button("Finish", |s| s.quit()));
}

#[tokio::main]
async fn download(template: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://gitignore.io/api/{}", template);
    let res = reqwest::get(&url).await?;
    let payload = res.text().await?;

    write_file(payload);

    Ok(())
}

fn write_file(payload: String) {
    let path = Path::new(
        &env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    )
    .join(".gitignore");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    match file.write_all(payload.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
