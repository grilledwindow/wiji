use std::fs;
use time::*;
use time::macros::*;

mod nofap;
use nofap::NoFap;

fn main() {
    let json_str = fs::read_to_string("nofap.json").unwrap(); 
    let NoFap { goal, relapsed } = serde_json::from_str(&json_str).unwrap();

    let now = OffsetDateTime::now_local().unwrap();
    let now_date = now.date();

    let goal_days = Duration::days(goal.into());
    let day_zero = Date::parse(relapsed.last().unwrap(), format_description!("[year]-[month]-[day]")).unwrap();

    let duration_clean = now_date - day_zero;
    let goal_days_left = goal_days.checked_sub(duration_clean).unwrap();
    let goal_date = now_date.saturating_add(goal_days_left).format(format_description!("[day] [month repr:short] [year]")).unwrap();

    let days_clean = duration_clean.whole_days();
    let num_goal_days = goal_days.whole_days();
    let num_goal_days_left = goal_days_left.whole_days();
    let progress = days_clean as f64 / num_goal_days as f64 * 100.0;
    println!("goal date: {goal_date}\ngoal: {days_clean}/{num_goal_days} days ({progress:.2}%)\n{num_goal_days_left} days left!");
}

