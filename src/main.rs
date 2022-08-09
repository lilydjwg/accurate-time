#![feature(exclusive_range_pattern)]

use std::fs;
use chrono::{Local, DateTime};

fn render_tmux(dt: DateTime<Local>, ncpu: usize) -> String {
  let load_string = fs::read_to_string("/proc/loadavg").unwrap();
  let mut split = load_string.split(' ');
  let load1 = split.next().unwrap();
  let load5 = split.next().unwrap();
  let load15 = split.next().unwrap();

  let load: f32 = load1.parse().unwrap();
  let color = match (load / ncpu as f32 * 100.0).round() as u32 {
    0..25 => "green",
    25..50 => "white",
    50..75 => "blue",
    75..100 => "cyan",
    100..200 => "yellow",
    200..400 => "magenta",
    400.. => "red",
  };
  format!(
    "#[fg={}]{} {} {} #[fg=colour15]{}",
    color, 
    load1, load5, load15,
    dt.format("%Y-%m-%d %H:%M:%S"),
  )
}

fn wait_for_whole_seconds(secs: u64) {
  use std::time::{SystemTime, Duration};
  use std::thread::sleep;

  let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let mut to_wait = Duration::from_secs(1) - Duration::from_nanos(u64::from(dur.subsec_nanos()));
  if secs > 1 {
    to_wait += Duration::from_secs(secs - dur.as_secs() % secs)
  }
  sleep(to_wait);
}

fn tmux() {
  use std::process::Command;
  use fork::{daemon, Fork};

  let ncpu = num_cpus::get();
  if let Ok(Fork::Child) = daemon(false, false) {
    loop {
      let dt = Local::now();
      let info = render_tmux(dt, ncpu);
      let st = Command::new("tmux")
        .args(["set", "-g", "status-right"])
        .arg(&info)
        .status()
        .unwrap();
      if !st.success() {
        // tmux has exited?
        break;
      }
      wait_for_whole_seconds(1);
    }
  }
}

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  Tmux,
}

fn main() {
  let cli = Cli::parse();
  match &cli.command {
    Command::Tmux => tmux(),
  }
}
