use crate::data::{SessionRecord, Stats};

/// Group sessions by day and calculate daily averages
pub fn calculate_daily_progress(sessions: &[SessionRecord]) -> Vec<(String, f64, f64)> {
    if sessions.is_empty() {
        return Vec::new();
    }

    let mut daily_sessions: std::collections::HashMap<String, Vec<&SessionRecord>> =
        std::collections::HashMap::new();

    // Group sessions by date (YYYY-MM-DD format)
    for session in sessions {
        if let Some(date_part) = session.timestamp.split('T').next() {
            daily_sessions
                .entry(date_part.to_string())
                .or_default()
                .push(session);
        }
    }

    // Sort dates and calculate averages for each day
    let mut daily_progress: Vec<(String, f64, f64)> = daily_sessions
        .into_iter()
        .map(|(date, day_sessions)| {
            let avg_wpm =
                day_sessions.iter().map(|s| s.wpm).sum::<f64>() / day_sessions.len() as f64;
            let avg_accuracy =
                day_sessions.iter().map(|s| s.accuracy).sum::<f64>() / day_sessions.len() as f64;
            (date, avg_wpm, avg_accuracy)
        })
        .collect();

    // Sort by date (oldest first)
    daily_progress.sort_by(|a, b| a.0.cmp(&b.0));

    daily_progress
}

/// Group sessions by week and calculate weekly averages
pub fn calculate_weekly_progress(sessions: &[SessionRecord]) -> Vec<(String, f64, f64)> {
    if sessions.is_empty() {
        return Vec::new();
    }

    let mut weekly_sessions: std::collections::HashMap<String, Vec<&SessionRecord>> =
        std::collections::HashMap::new();

    // Group sessions by week (YYYY-WW format)
    for session in sessions {
        if let Some(date_part) = session.timestamp.split('T').next() {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                let week_key = date.format("%Y-W%U").to_string();
                weekly_sessions.entry(week_key).or_default().push(session);
            }
        }
    }

    // Sort weeks and calculate averages for each week
    let mut weekly_progress: Vec<(String, f64, f64)> = weekly_sessions
        .into_iter()
        .map(|(week, week_sessions)| {
            let avg_wpm =
                week_sessions.iter().map(|s| s.wpm).sum::<f64>() / week_sessions.len() as f64;
            let avg_accuracy =
                week_sessions.iter().map(|s| s.accuracy).sum::<f64>() / week_sessions.len() as f64;
            (week, avg_wpm, avg_accuracy)
        })
        .collect();

    // Sort by week (oldest first)
    weekly_progress.sort_by(|a, b| a.0.cmp(&b.0));

    weekly_progress
}

/// Group sessions by month and calculate monthly averages
pub fn calculate_monthly_progress(sessions: &[SessionRecord]) -> Vec<(String, f64, f64)> {
    if sessions.is_empty() {
        return Vec::new();
    }

    let mut monthly_sessions: std::collections::HashMap<String, Vec<&SessionRecord>> =
        std::collections::HashMap::new();

    // Group sessions by month (YYYY-MM format)
    for session in sessions {
        if let Some(date_part) = session.timestamp.split('T').next() {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                let month_key = date.format("%Y-%m").to_string();
                monthly_sessions.entry(month_key).or_default().push(session);
            }
        }
    }

    // Sort months and calculate averages for each month
    let mut monthly_progress: Vec<(String, f64, f64)> = monthly_sessions
        .into_iter()
        .map(|(month, month_sessions)| {
            let avg_wpm =
                month_sessions.iter().map(|s| s.wpm).sum::<f64>() / month_sessions.len() as f64;
            let avg_accuracy = month_sessions.iter().map(|s| s.accuracy).sum::<f64>()
                / month_sessions.len() as f64;
            (month, avg_wpm, avg_accuracy)
        })
        .collect();

    // Sort by month (oldest first)
    monthly_progress.sort_by(|a, b| a.0.cmp(&b.0));

    monthly_progress
}

/// Create a daily progress chart showing WPM by day
pub fn create_daily_wpm_chart(daily_progress: &[(String, f64, f64)]) -> Vec<String> {
    if daily_progress.is_empty() {
        return vec!["No daily data available".to_string()];
    }

    let mut chart = vec!["Daily WPM Progress:".to_string(), "".to_string()];

    // Show last 7 days or all if less
    let days_to_show = daily_progress.len().min(7);
    let recent_days = &daily_progress[daily_progress.len().saturating_sub(days_to_show)..];

    for (date, wpm, _accuracy) in recent_days {
        let day_num = date.split('-').next_back().unwrap_or("??");

        // Create a simple bar (up to 10 characters)
        let bar_length = ((wpm / 100.0) * 10.0).round() as usize; // Assuming 100 WPM max for scaling
        let bar = "█".repeat(bar_length.min(10));

        chart.push(format!("Day {}: {:.0} WPM {}", day_num, wpm, bar));
    }

    chart
}

/// Create a weekly progress chart showing WPM by week
pub fn create_weekly_wpm_chart(weekly_progress: &[(String, f64, f64)]) -> Vec<String> {
    if weekly_progress.is_empty() {
        return vec!["No weekly data available".to_string()];
    }

    let mut chart = vec!["Weekly WPM Progress:".to_string(), "".to_string()];

    // Show last 4 weeks or all if less
    let weeks_to_show = weekly_progress.len().min(4);
    let recent_weeks = &weekly_progress[weekly_progress.len().saturating_sub(weeks_to_show)..];

    for (week, wpm, _accuracy) in recent_weeks {
        // Extract week number
        let week_num = week.split('-').next_back().unwrap_or("??").replace('W', "");

        // Create a simple bar (up to 15 characters)
        let bar_length = ((wpm / 100.0) * 15.0).round() as usize; // Assuming 100 WPM max for scaling
        let bar = "█".repeat(bar_length.min(15));

        chart.push(format!("Week {}: {:.0} WPM {}", week_num, wpm, bar));
    }

    chart
}

/// Create a monthly progress chart showing WPM by month
pub fn create_monthly_wpm_chart(monthly_progress: &[(String, f64, f64)]) -> Vec<String> {
    if monthly_progress.is_empty() {
        return vec!["No monthly data available".to_string()];
    }

    let mut chart = vec!["Monthly WPM Progress:".to_string(), "".to_string()];

    // Show last 6 months or all if less
    let months_to_show = monthly_progress.len().min(6);
    let recent_months = &monthly_progress[monthly_progress.len().saturating_sub(months_to_show)..];

    for (month, wpm, _accuracy) in recent_months {
        // Extract month name
        let month_parts: Vec<&str> = month.split('-').collect();
        let month_name = if month_parts.len() == 2 {
            match month_parts[1] {
                "01" => "Jan",
                "02" => "Feb",
                "03" => "Mar",
                "04" => "Apr",
                "05" => "May",
                "06" => "Jun",
                "07" => "Jul",
                "08" => "Aug",
                "09" => "Sep",
                "10" => "Oct",
                "11" => "Nov",
                "12" => "Dec",
                _ => "???",
            }
        } else {
            "???"
        };

        // Create a simple bar (up to 20 characters)
        let bar_length = ((wpm / 100.0) * 20.0).round() as usize; // Assuming 100 WPM max for scaling
        let bar = "█".repeat(bar_length.min(20));

        chart.push(format!(
            "{} {}: {:.0} WPM {}",
            month_parts[0], month_name, wpm, bar
        ));
    }

    chart
}

/// Calculate daily goal progress
pub fn calculate_daily_goal_progress(
    sessions: &[SessionRecord],
    daily_wpm_goal: f64,
) -> (f64, f64, String, Vec<String>) {
    if sessions.is_empty() {
        return (0.0, daily_wpm_goal, "No sessions today".to_string(), vec![]);
    }

    // Get today's date
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    // Filter sessions from today
    let today_sessions: Vec<_> = sessions
        .iter()
        .filter(|s| s.timestamp.starts_with(&today))
        .collect();

    if today_sessions.is_empty() {
        return (0.0, daily_wpm_goal, "No sessions today".to_string(), vec![]);
    }

    // Calculate today's average WPM
    let total_wpm: f64 = today_sessions.iter().map(|s| s.wpm).sum();
    let avg_wpm = total_wpm / today_sessions.len() as f64;

    // Calculate progress percentage
    let progress_percent = (avg_wpm / daily_wpm_goal * 100.0).min(100.0);

    // Status message
    let status = if avg_wpm >= daily_wpm_goal {
        "🎯 Goal achieved! Great job!".to_string()
    } else {
        format!("Keep going! {:.0} WPM to go", daily_wpm_goal - avg_wpm)
    };

    // Session details
    let details = vec![
        format!("Today's Sessions: {}", today_sessions.len()),
        format!(
            "Best WPM: {:.0}",
            today_sessions.iter().map(|s| s.wpm).fold(0.0_f64, f64::max)
        ),
        format!(
            "Avg Accuracy: {:.1}%",
            today_sessions.iter().map(|s| s.accuracy).sum::<f64>() / today_sessions.len() as f64
        ),
    ];

    (avg_wpm, progress_percent, status, details)
}

/// Create goal progress display
pub fn create_goal_progress_display(
    current_wpm: f64,
    target_wpm: f64,
    progress_percent: f64,
    status: &str,
    details: &[String],
) -> Vec<String> {
    // Create progress bar
    let bar_length = 20;
    let filled = (progress_percent / 100.0 * bar_length as f64).round() as usize;
    let progress_bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_length - filled));

    let mut display = vec![
        "🎯 Daily Goal Progress".to_string(),
        "".to_string(),
        format!(
            "Current: {:.0} WPM | Target: {:.0} WPM",
            current_wpm, target_wpm
        ),
        format!("Progress: {} {:.0}%", progress_bar, progress_percent),
        "".to_string(),
        status.to_string(),
    ];

    if !details.is_empty() {
        display.push("".to_string());
        display.extend(details.iter().cloned());
    }

    display
}

/// Get session statistics for charting
pub fn get_session_chart_stats(stats: &Stats) -> Vec<SessionRecord> {
    let max_sessions = 30; // Show last 30 sessions
    let session_count = stats.sessions.len().min(max_sessions);

    if stats.sessions.is_empty() {
        Vec::new()
    } else {
        stats.sessions[stats.sessions.len() - session_count..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_session_chart_stats_empty() {
        let stats = Stats::new();
        let chart_stats = get_session_chart_stats(&stats);
        assert!(chart_stats.is_empty());
    }

    #[test]
    fn test_get_session_chart_stats_limited() {
        let mut stats = Stats::new();

        // Add more than 30 sessions
        for i in 0..35 {
            stats.add_session(SessionRecord::new(
                format!("Test {}", i),
                40.0 + i as f64,
                90.0,
                std::time::Duration::from_secs(60),
                std::time::Duration::from_secs(300),
            ));
        }

        let chart_stats = get_session_chart_stats(&stats);
        assert_eq!(chart_stats.len(), 30); // Should be limited to 30

        // Should be last 30 sessions
        assert_eq!(chart_stats[0].lesson_type, "Test 5");
        assert_eq!(chart_stats[29].lesson_type, "Test 34");
    }

    #[test]
    fn test_calculate_daily_progress() {
        use std::time::Duration;

        let sessions = vec![
            SessionRecord::new(
                "test".to_string(),
                40.0,
                90.0,
                Duration::from_secs(60),
                Duration::from_secs(60),
            ),
            SessionRecord::new(
                "test".to_string(),
                50.0,
                95.0,
                Duration::from_secs(60),
                Duration::from_secs(60),
            ),
        ];

        // Mock timestamps for same day
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let mut sessions_with_dates = sessions;
        for session in &mut sessions_with_dates {
            session.timestamp = format!("{}T12:00:00Z", today);
        }

        let daily_progress = calculate_daily_progress(&sessions_with_dates);
        assert_eq!(daily_progress.len(), 1);
        assert_eq!(daily_progress[0].0, today);
        assert_eq!(daily_progress[0].1, 45.0); // Average WPM: (40 + 50) / 2
        assert_eq!(daily_progress[0].2, 92.5); // Average accuracy: (90 + 95) / 2
    }

    #[test]
    fn test_create_daily_wpm_chart() {
        let daily_progress = vec![
            ("2024-01-01".to_string(), 40.0, 90.0),
            ("2024-01-02".to_string(), 45.0, 92.0),
            ("2024-01-03".to_string(), 50.0, 95.0),
        ];

        let chart = create_daily_wpm_chart(&daily_progress);
        assert!(chart.len() > 2); // Title + empty + data
        assert!(chart[0].contains("Daily WPM Progress"));
        assert!(chart.iter().any(|line| line.contains("45 WPM")));
    }

    #[test]
    fn test_calculate_daily_goal_progress() {
        use std::time::Duration;

        let sessions = vec![SessionRecord::new(
            "test".to_string(),
            45.0,
            90.0,
            Duration::from_secs(60),
            Duration::from_secs(60),
        )];

        // Mock today's timestamp
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let mut sessions_with_dates = sessions;
        for session in &mut sessions_with_dates {
            session.timestamp = format!("{}T12:00:00Z", today);
        }

        let (current_wpm, progress_percent, status, details) =
            calculate_daily_goal_progress(&sessions_with_dates, 50.0);

        assert_eq!(current_wpm, 45.0);
        assert_eq!(progress_percent, 90.0); // 45/50 * 100
        assert!(status.contains("Keep going"));
        assert!(details.len() > 0);
        assert!(details[0].contains("Today's Sessions: 1"));
    }

    #[test]
    fn test_create_goal_progress_display() {
        let details = vec!["Session 1".to_string(), "Session 2".to_string()];
        let display = create_goal_progress_display(45.0, 50.0, 90.0, "Keep going!", &details);

        assert!(display.len() > 5);
        assert!(display[0].contains("🎯 Daily Goal Progress"));
        assert!(display[2].contains("45 WPM"));
        assert!(display[2].contains("50 WPM"));
        assert!(display[3].contains("██████████████░░")); // 90% progress = 18 filled + 2 empty
        assert!(display[5].contains("Keep going!"));
    }
}
