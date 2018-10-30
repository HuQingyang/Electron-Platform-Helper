
use toml;
use std::{env, fs::File};
use std::path::{Path, PathBuf};

pub fn is_path_exist(path: &str) -> bool {
  Path::new(path).exists()
}

pub fn path_buf_to_string(path: PathBuf) -> String {
  path
    .to_str()
    .unwrap()
    .to_owned()
}

pub fn read_file_to_string(path: PathBuf) -> String {
  let mut f = File::open(path)
    .expect(&format!("file \"{}\" not found", &path));
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  contents
}

// config
#[derive(Deserialize)]
struct Config {
  pub target: String,
  pub runtime: String,
  pub installed: bool
}

pub fn get_config() -> Config {
  let current_path = env::current_exe().unwrap();
  let config_path = Path::new(&current_path)
    .with_file_name("ElectronPlatform.toml");
  println!("{}", &config_path);
  let values: Config = toml::from_str(
    &read_file_to_string(config_path)
  ).unwrap();
  return values;
}

// Handle HTML
fn inline_style(s: &str) -> String {
  format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
  format!(r#"<script type="text/javascript">{}</script>"#, s)
}

pub fn generate_html(styles: Vec<&str>, scripts: Vec<&str>) -> String {
  let inline_styles = styles.into_iter()
    .map(inline_style)
    .collect::<Vec<String>>()
    .join("\n");
  let inline_scripts = scripts.into_iter()
    .map(inline_script)
    .collect::<Vec<String>>()
    .join("\n");

  format!(r#"
    <!doctype html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport"
              content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>Document</title>
        {styles}
    </head>
    <body>
        <div id="root"></div>
        {scripts}
    </body>
    </html>"#,
    styles = inline_styles,
    scripts = inline_scripts
  )
}
