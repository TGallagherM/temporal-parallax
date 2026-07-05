use chrono::{DateTime, Duration, Local, Utc};
use chrono_tz::Tz;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Task, time};
use std::time::Duration as StdDuration;

fn main() -> iced::Result {
    iced::application(
        || (ClockApp::default(), Task::none()),
        update,
        view,
    )
    .subscription(|_| {
        time::every(StdDuration::from_secs(1)).map(Message::Tick)
    })
    .run()
}

#[derive(Clone, Debug)]
enum Message {
    Tick(iced::time::Instant),
    ToggleFormat,
    TimeSynced(Result<i64, String>),
}
struct TimezoneConfig {
    label: &'static str,
    tz: Tz,
}
struct ClockApp {
    is_24h: bool,
    ntp_offset_seconds: i64,
    last_sync: Option<DateTime<Local>>,
    sync_status: String,
    timezone_configs: Vec<TimezoneConfig>,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            is_24h: true,
            ntp_offset_seconds: 0,
            last_sync: None,
            sync_status: String::from("Syncing..."),
            timezone_configs: vec![
                TimezoneConfig {
                    label: "UTC",
                    tz: Tz::UTC,
                },
                TimezoneConfig {
                    label: "New York",
                    tz: "America/New_York".parse().unwrap(),
                },
                TimezoneConfig {
                    label: "Tokyo",
                    tz: "Asia/Tokyo".parse().unwrap(),
                },
                TimezoneConfig {
                    label: "London",
                    tz: "Europe/London".parse().unwrap(),
                },
            ],
        }
    }
}

fn update(clock: &mut ClockApp, message: Message) -> Task<Message> {
    match message {
        Message::Tick(_) => Task::none(),
        Message::ToggleFormat => {
            clock.is_24h = !clock.is_24h;
            Task::none()
        }
        Message::TimeSynced(result) => {
            match result {
                Ok(offset) => {
                    clock.ntp_offset_seconds = offset;
                    clock.last_sync = Some(Local::now());
                    clock.sync_status = format!("Synced: {:+} seconds", offset);
                }
                Err(error) => {
                    clock.sync_status = format!("Sync failed: {}", error);
                }
            }
            Task::none()
        }
    }
}

fn view(clock: &ClockApp) -> Element<'_, Message> {
    let title = text("Temporal Parallax Clock").size(32);

    let toggle_button = button(if clock.is_24h {
        "Switch to 12h"
    } else {
        "Switch to 24h"
    })
    .on_press(Message::ToggleFormat);

    let status = text(format!(
        "{} | Last sync: {}",
        clock.sync_status,
        clock.last_sync
            .map(|t| t.format("%H:%M:%S").to_string())
            .unwrap_or_else(|| "never".to_string())
    ));

    let timezone_rows =
        clock.timezone_configs.iter().fold(column!().spacing(10), |column, config| {
            let time = clock.format_time(clock.current_corrected_utc(), config.tz);
            column.push(
                row![
                    text(config.label).width(Length::FillPortion(1)),
                    text(time).width(Length::FillPortion(2))
                ]
                .spacing(20),
            )
        });

    let content = column![title, toggle_button, status, timezone_rows]
        .spacing(20)
        .padding(20);

    container(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

impl ClockApp {
    fn current_corrected_utc(&self) -> DateTime<Utc> {
        let local_now = Local::now();
        let corrected = local_now + Duration::seconds(self.ntp_offset_seconds);
        corrected.with_timezone(&Utc)
    }

    fn format_time(&self, utc_time: DateTime<Utc>, tz: Tz) -> String {
        let tz_time = utc_time.with_timezone(&tz);
        if self.is_24h {
            tz_time.format("%H:%M:%S").to_string()
        } else {
            tz_time.format("%I:%M:%S %p").to_string()
        }
    }
}
