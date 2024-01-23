use std::time::{Duration, Instant};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct CountUpOptions {
    pub start_val: f64,
    pub decimal_places: usize,
    pub duration: f64,
    pub use_grouping: bool,
    pub use_indian_separators: bool,
    pub use_easing: bool,
    pub smart_easing_threshold: f64,
    pub smart_easing_amount: f64,
    pub separator: String,
    pub decimal: String,
    pub prefix: String,
    pub suffix: String,
    pub enable_scroll_spy: bool,
    pub scroll_spy_delay: u64,
    pub scroll_spy_once: bool,
    pub on_complete_callback: Callback<()>,
    pub on_start_callback: Callback<()>,
}

pub fn count_up(options: CountUpOptions) -> Html {
    let start_val = options.start_val;
    let duration = options.duration * 1000.0; // Convert seconds to milliseconds

    let (link, state) = use_state(|| CountUpState {
        options,
        start_val,
        end_val: 0.0,
        duration,
        frame_val: start_val,
        paused: true,
        final_end_val: None,
        use_easing: true,
        count_down: false,
        error: None,
        start_time: None,
        remaining: duration,
    });

    let on_start = link.callback(|_| Msg::Start);
    let on_pause_resume = link.callback(|_| Msg::PauseResume);
    let on_reset = link.callback(|_| Msg::Reset);
    let on_update = link.callback(|new_end_val| Msg::Update(new_end_val));

    html! {
        <div>
            <span>{ state.frame_val }</span>
            <button onclick=on_start>{"Start"}</button>
            <button onclick=on_pause_resume>{"Pause/Resume"}</button>
            <button onclick=on_reset>{"Reset"}</button>
            <button onclick=|_| on_update.emit(1000.0)>{"Update to 1000"}</button>
        </div>
    }
}
#[derive(Clone, Properties)]
pub struct CountUpOptions {
    pub start_val: f64,
    pub decimal_places: usize,
    pub duration: f64,
    pub use_grouping: bool,
    pub use_indian_separators: bool,
    pub use_easing: bool,
    pub smart_easing_threshold: f64,
    pub smart_easing_amount: f64,
    pub separator: String,
    pub decimal: String,
    pub prefix: String,
    pub suffix: String,
    pub enable_scroll_spy: bool,
    pub scroll_spy_delay: u64,
    pub scroll_spy_once: bool,
    pub on_complete_callback: Callback<()>,
    pub on_start_callback: Callback<()>,
}
struct CountUpState {
    options: CountUpOptions,
    start_val: f64,
    end_val: f64,
    duration: f64,
    frame_val: f64,
    paused: bool,
    final_end_val: Option<f64>,
    use_easing: bool,
    count_down: bool,
    error: Option<String>,
    start_time: Option<Instant>,
    remaining: f64,
}

enum Msg {
    Start,
    PauseResume,
    Reset,
    Update(f64),
    Tick(Instant),
}

impl Component for CountUpState {
    type Message = Msg;
    type Properties = CountUpOptions;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let start_val = props.start_val;
        let duration = props.duration * 1000.0; // Convert seconds to milliseconds
        let remaining = duration;
        CountUpState {
            options: props,
            start_val,
            end_val: 0.0,
            duration,
            frame_val: start_val,
            paused: true,
            final_end_val: None,
            use_easing: true,
            count_down: false,
            error: None,
            start_time: None,
            remaining,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Start => {
                if let Some(on_start_callback) = &self.options.on_start_callback {
                    on_start_callback.emit(());
                }
                if self.duration > 0.0 {
                    self.determine_direction_and_smart_easing();
                    self.paused = false;
                    self.start_time = Some(Instant::now());
                    self.link.send_message(Msg::Tick(self.start_time.unwrap()));
                } else {
                    self.print_value(self.end_val);
                }
            }
            Msg::PauseResume => {
                if !self.paused {
                    self.paused = true;
                } else {
                    self.start_time = None;
                    self.duration = self.remaining;
                    self.start_val = self.frame_val;
                    self.determine_direction_and_smart_easing();
                    self.link.send_message(Msg::Tick(Instant::now()));
                }
            }
            Msg::Reset => {
                self.paused = true;
                self.reset_duration();
                self.start_val = self.options.start_val;
                self.frame_val = self.start_val;
                self.print_value(self.start_val);
            }
            Msg::Update(new_end_val) => {
                self.end_val = new_end_val;
                if self.end_val == self.frame_val {
                    return false;
                }
                self.start_val = self.frame_val;
                if self.final_end_val.is_none() {
                    self.reset_duration();
                }
                self.final_end_val = None;
                self.determine_direction_and_smart_easing();
                self.link.send_message(Msg::Tick(Instant::now()));
            }
            Msg::Tick(timestamp) => {
                if self.start_time.is_none() {
                    self.start_time = Some(timestamp);
                }

                let progress = timestamp.duration_since(self.start_time.unwrap()).as_millis() as f64;
                self.remaining = self.duration - progress;

                // To ease or not to ease
                if self.use_easing {
                    if self.count_down {
                        self.frame_val = self.start_val
                            - self.easing_fn(progress, 0.0, self.start_val - self.end_val, self.duration);
                    } else {
                        self.frame_val = self.easing_fn(progress, self.start_val, self.end_val - self.start_val, self.duration);
                    }
                } else {
                    self.frame_val = self.start_val + (self.end_val - self.start_val) * (progress / self.duration);
                }

                // Don't go past end_val since progress can exceed duration in the last frame
                let went_past = if self.count_down { self.frame_val < self.end_val } else { self.frame_val > self.end_val };
                if went_past {
                    self.frame_val = self.end_val;
                }

                // Decimal
                self.frame_val = (self.frame_val * 10_f64.powi(self.options.decimal_places as i32)).round()
                    / 10_f64.powi(self.options.decimal_places as i32);

                // Format and print value
                self.print_value(self.frame_val);

                // Whether to continue
                if progress < self.duration {
                    self.link.send_message(Msg::Tick(timestamp));
                } else if self.final_end_val.is_some() {
                    // Smart easing
                    self.link.send_message(Msg::Update(self.final_end_val.unwrap()));
                } else {
                    if let Some(on_complete_callback) = &self.options.on_complete_callback {
                        on_complete_callback.emit(());
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.options = props;
        self.start_val = self.options.start_val;
        self.end_val = self.options.end_val;
        self.reset_duration();
        self.final_end_val = None;
        self.paused = true;
        self.frame_val = self.start_val;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <span>{ self.frame_val }</span>
                <button onclick=self.link.callback(|_| Msg::Start)>{"Start"}</button>
                <button onclick=self.link.callback(|_| Msg::PauseResume)>{"Pause/Resume"}</button>
                <button onclick=self.link.callback(|_| Msg::Reset)>{"Reset"}</button>
                <button onclick=self.link.callback(|_| Msg::Update(1000.0))>{"Update to 1000"}</button>
            </div>
        }
    }
}

impl CountUpState {
    fn determine_direction_and_smart_easing(&mut self) {
        let end = self.final_end_val.unwrap_or(self.end_val);
        self.count_down = self.start_val > end;
        let animate_amount = end - self.start_val;
        if animate_amount.abs() > self.options.smart_easing_threshold && self.options.use_easing {
            self.final_end_val = Some(end);
            let up = if self.count_down { 1.0 } else { -1.0 };
            self.end_val = end + (up * self.options.smart_easing_amount);
            self.duration /= 2.0;
        } else {
            self.end_val = end;
            self.final_end_val = None;
        }
        if self.final_end_val.is_none() {
            self.use_easing = self.options.use_easing;
        } else {
            // Setting final_end_val indicates smart easing
            self.use_easing = false;
        }
    }

    fn reset_duration(&mut self) {
        self.start_time = None;
        self.duration = self.options.duration * 1000.0; // Convert seconds to milliseconds
        self.remaining = self.duration;
    }

    fn print_value(&self, val: f64) {
        // Implement rendering logic here
        // You can use self.options to access formatting options
        // and update the rendered value in your HTML elements.
    }

    // Add your easing function here
    fn easing_fn(&self, t: f64, b: f64, c: f64, d: f64) -> f64 {
        // Implement your easing function here
        // Example: easeOutExpo
        c * (-2.0_f64.powf(-10.0 * t / d) + 1.0) + b
    }
}

fn main() {
    yew::start_app::<CountUpState>();
}