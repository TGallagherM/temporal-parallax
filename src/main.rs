use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use chrono_tz::Tz;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{executor, time, Application, Command, Element, Length, Settings, Subscription};
use std::time::Duration as StdDuration;
fn main() {
    
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
impl Application for ClockApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let app = ClockApp {
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
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Temporal Parallax Clock")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(StdDuration::from_secs(1)).map(Message::Tick)
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Temporal Parallax Clock")
            .size(32);

        container(title)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
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