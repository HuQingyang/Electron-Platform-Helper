use std::env;
use std::path::{Path};
use std::process::Command;
use model::Platform;
use utils::{path_buf_to_string, is_path_exist};


fn get_current_platform() -> Platform {
  if cfg!(target_os = "macos") {
    return Platform::DARWIN;
  } else if cfg!(target_os = "windows") {
    if cfg!(target_pointer_width = "32") {
      return Platform::WIN32;
    } else if cfg!(target_pointer_width = "64") {
      return Platform::WIN64;
    }
  } else if cfg!(target_os = "linux") {
    if cfg!(target_pointer_width = "32") {
      return Platform::LINUX32;
    } else if cfg!(target_pointer_width = "64") {
      return Platform::LINUX64;
    }
  }
  Platform::UNKNOWN
}

fn platform_to_str(platform: Platform) -> String {
  match platform {
    Platform::DARWIN => String::from("darwin"),
    Platform::WIN32 => String::from("win32"),
    Platform::WIN64 => String::from("win64"),
    Platform::LINUX32 => String::from("linux32"),
    Platform::LINUX64 => String::from("linux64"),
    Platform::UNKNOWN => String::from("unknown")
  }
}

pub fn get_valid_runtime_path(version: &str) -> Option<Path> {

}

pub fn is_runtime_exist(target: &str) -> bool {
  let home_path = path_buf_to_string(env::home_dir().unwrap());
  let platform_path = path_buf_to_string(
    Path::new(&home_path)
      .join(Path::new(".electron-platform"))
  );
  if !is_path_exist(&platform_path) {
    return false;
  }

  let runtime_path = path_buf_to_string(
    Path::new(&platform_path)
      .join(Path::new(
        &format!("runtime/{}/{}",
           platform_to_str(platform), version
        )
      ))
  );
  is_path_exist(&runtime_path)
}

pub fn open_app_bin() {
  let current_path = env::current_exe().unwrap();
  if cfg!(target_os = "windows") {
    // TODO: Windows
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .spawn()
      .expect("failed to execute process")
  } else {
    let bin_path = path_buf_to_string(
      Path::new(&current_path)
        .with_file_name("App")
    );
    println!("current: {}", &path_buf_to_string(current_path.clone()));
    println!("bin: {}", &bin_path);
    Command::new(&bin_path)
      .spawn()
      .expect("failed to execute process")
  };
}