use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
  // TODO: check all dates within a week
  let mut date = None;

  let input = fs::read_to_string("input.csv")?;
  let schedules = input
    .lines()
    .map(|line| {
      let mut parts = line.split(" ");
      let day = parts.next().unwrap();
      if date == None {
        date = Some(day.to_string());
      }

      // ref https://github.com/nhn/tui.calendar/issues/446#issuecomment-559125865
      let start = format!("{}T{}:00+00:00", day, parts.next().unwrap());
      let end = format!("{}T{}:00+00:00", day, parts.next().unwrap());

      let sched = format!(include_str!("../templates/schedule.tmpl"), start = start, end = end);

      format!("{{{}}}", sched)
    })
    .collect::<Vec<String>>();

  let index = format!(
    include_str!(concat!(env!("OUT_DIR"), "/index.html")),
    date = date.unwrap(),
    schedules = schedules.join(",")
  );

  fs::create_dir_all("publish")?;
  fs::write("publish/index.html", index)?;

  Ok(())
}
