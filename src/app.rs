#![allow(dead_code, unused_imports)]
use chrono::prelude::*;
use std::convert::TryInto;
use std::ops::{Add, AddAssign, Sub, SubAssign};
pub use std::time::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(std::time::Instant);
#[cfg(not(target_arch = "wasm32"))]
impl Instant {
    pub fn now() -> Self {
        Self(std::time::Instant::now())
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.duration_since(earlier.0)
    }
    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(|i| Self(i))
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration).map(|i| Self(i))
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    fn date_now() -> f64;
}
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);
#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self {
        Self(date_now() as u64)
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_millis(self.0 - earlier.0)
    }
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        match duration.as_millis().try_into() {
            Ok(duration) => self.0.checked_add(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        match duration.as_millis().try_into() {
            Ok(duration) => self.0.checked_sub(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, other: Duration) -> Instant {
        self.checked_add(other).unwrap()
    }
}
impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, other: Duration) -> Instant {
        self.checked_sub(other).unwrap()
    }
}
impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}
impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}
impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let drink_at = use_state(|| Local::now().format("%H:%M:%S").to_string());
    let drink_counter = use_state(|| 0);

    let drink = {
        let drink_counter = drink_counter.clone();
        let drink_at = drink_at.clone();
        move |_| {
            let v = *drink_counter + 1;
            drink_counter.set(v);
            drink_at.set(Local::now().format("%H:%M:%S").to_string());
        }
    };

    let medicine_at = use_state(|| Local::now().format("%H:%M:%S").to_string());
    let medicine_counter = use_state(|| 0);

    let medicine = {
        let medicine_counter = medicine_counter.clone();
        let medicine_at = medicine_at.clone();
        move |_| {
            let v = *medicine_counter + 1;
            medicine_counter.set(v);
            medicine_at.set(Local::now().format("%H:%M:%S").to_string());
        }
    };

    let reset = {
        let drink_counter = drink_counter.clone();
        let drink_at = drink_at.clone();
        let medicine_counter = medicine_counter.clone();
        let medicine_at = medicine_at.clone();
        move |_| {
            drink_counter.set(0);
            drink_at.set(Local::now().format("%H:%M:%S").to_string());
            medicine_counter.set(0);
            medicine_at.set(Local::now().format("%H:%M:%S").to_string());
        }
    };

    html! {
        <main class="container">
            <h1>{"Keep health, Drink water!"}</h1>
            <div>
                <button type="button" onclick={reset}>{"Reset Today"}</button>
            </div>
            <div class="row">
                <span class="logo"><b>{ *drink_counter }</b></span>
                <span class="logo"><b>{ *medicine_counter }</b></span>
            </div>
            <div class="row">
                <button type="button" onclick={drink}>{"Drink @ "}{ (*drink_at).clone() }</button>
                <button type="button" onclick={medicine}>{"Medicine @ "}{ (*medicine_at).clone() }</button>
            </div>
        </main>
    }
}
