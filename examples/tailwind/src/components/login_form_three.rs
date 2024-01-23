use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

#[function_component(LoginFormThree)]
pub fn login_form_three() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let eye_active_handle = use_state(|| false);
    let eye_active = (*eye_active_handle).clone();

    let password_valid_handle = use_state(|| true);
    let password_valid = (*password_valid_handle).clone();

    let password_type_handle = use_state(|| "text");
    let password_type = (*password_type_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let validate_email = |email: &str| {
        let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
        pattern.is_match(email)
    };

    let validate_password = |password: &str| !password.is_empty();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
                email_valid_handle.set(validate_email(&input.value()));
            }
        })
    };

    let on_password_change = {
        let input_password_ref = input_password_ref.clone();

        Callback::from(move |_| {
            let input = input_password_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_password_handle.set(value);
                password_valid_handle.set(validate_password(&input.value()));
            }
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_password.clone();
        let password_ref = input_password.clone();
        let error_handle = error_handle.clone();
        console::log_1(&format!("Email: {}, Password: {}", input_email, input_password).into());

        spawn_local(async move {
            let email_val = email_ref.clone();
            let password_val = password_ref.clone();
            let error_handle = error_handle.clone();
            if email_valid && password_valid {
                let response = login_user(email_val.to_string(), password_val.to_string()).await;
                match response {
                    Ok(_) => {
                        console::log_1(&"success".into());
                        let window: Window = web_sys::window().expect("window not available");
                        let location = window.location();
                        let _ = location.set_href("/error");
                    }
                    Err(err) => {
                        error_handle.set(err);
                    }
                }
            } else {
                error_handle.set("Please provide a valid email and password!".into());
            }
        });
    });

    let on_toggle_password = {
        Callback::from(move |_| {
            if eye_active {
                password_type_handle.set("password".into())
            } else {
                password_type_handle.set("text".into())
            }
            eye_active_handle.set(!eye_active);
        })
    };

    html! {
        <section
          style="background: linear-gradient(135deg, #008080, #800080)"
          class="min-h-screen flex justify-center items-center"
        >
          <button
            class="dark-mode-toggle text-white text-2xl absolute top-8 right-8 cursor-pointer transition-colors hover:text-gray-50"
            aria-label="Toggle Dark Mode"
          >
            <i class="fas fa-sun"></i>
          </button>
          <div
            class="login-container w-11/12 md:w-2/3 lg:w-1/2 max-w-screen-md p-10 bg-white bg-opacity-90 border border-blue-500 rounded-lg relative"
          >
            <div class="flex flex-wrap-reverse justify-end items-center">
              <div
                class="image-placeholder flex-1 h-96 md:block hidden bg-cover bg-no-repeat bg-center mr-10"
                style="
                  background-image: url('https://images.unsplash.com/photo-1584713503693-bb386ec95cf2?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1074&q=80');
                "
                aria-hidden="true"
              ></div>
              <div class="form-container flex-1 max-w-xs">
                <h2 class="text-blue-500 text-3xl font-semibold mb-6 text-center">
                  {"Welcome Back!"}
                </h2>
                <form onsubmit={onsubmit}>
                  if !error.is_empty() {
                    <div class="bg-red-600 text-white py-3 font-bold rounded-md text-center mb-4 text-lg">{error}</div>
                  }
                  <div class="mb-4">
                    <label for="username" class="text-gray-600 font-semibold"
                      >{"Email"}</label
                    >
                    <div class="relative">
                      <span
                        class="input-group-text bg-blue-500 text-white px-3 py-2 rounded-l-lg absolute left-0 top-0 bottom-0 flex items-center"
                      >
                        <i class="fas fa-user"></i>
                      </span>
                      <input
                        type="text"
                        class="form-control rounded-r-lg pl-10 border border-blue-500 focus:ring focus:ring-blue-300 focus:outline-none w-full"
                        id="username"
                        name="username"
                        aria-required="true"
                        placeholder="Email"
                        required={true}
                        ref={input_email_ref}
                        oninput={on_email_change}
                        aria-label="Email"
                      />
                    </div>
                  </div>
                  if !email_valid {
                      <div class="error-txt text-red-500 text-sm mb-2">{"Enter a valid email address"}</div>
                  }
                  <div class="mb-4">
                    <label for="password" class="text-gray-600 font-semibold"
                      >{"Password"}</label
                    >
                    <div class="relative">
                      <span
                        class="input-group-text bg-blue-500 text-white px-3 py-2 rounded-l-lg absolute left-0 top-0 bottom-0 flex items-center"
                      >
                        <i class="fas fa-lock"></i>
                      </span>
                      <input
                        type={&*password_type}
                        class="form-control rounded-r-lg pl-10 border border-blue-500 focus:ring focus:ring-blue-300 focus:outline-none w-full"
                        id="password"
                        name="password"
                        placeholder="Password"
                        required={true}
                        aria-required="true"
                        aria-label="Password Input"
                        ref={input_password_ref}
                        oninput={on_password_change}
                      />
                      <i
                        class={format!("cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-600 toggle-button fa {}", if eye_active { "fa-eye" } else { "fa-eye-slash " })}
                        aria-hidden="true"
                        role="presentation"
                        onclick={on_toggle_password}
                      ></i>
                    </div>
                  </div>
                  if !password_valid {
                     <div class="error-txt text-red-500 text-sm my-2">{"Password can't be blank"}</div>
                  }
                  <div class="mb-4 flex items-center">
                    <input
                      type="checkbox"
                      id="rememberMe"
                      name="rememberMe"
                      class="mr-2"
                      aria-label="Remember Me Checkbox"
                    />
                    <label for="rememberMe" class="text-gray-600">{"Remember Me"}</label>
                  </div>
                  <button
                    type="submit"
                    class="btn btn-primary w-full flex items-center justify-center rounded-lg px-4 py-2 text-white text-lg font-semibold bg-blue-500 hover:bg-blue-600 active:bg-blue-700"
                    aria-label="Login Button"
                  >
                    <i class="fas fa-box-arrow mr-2"></i>{"Login"}
                  </button>
                  <div class="forgot-password text-center mt-6 mb-4">
                    <a
                      href="#"
                      class="text-gray-600 font-semibold hover:text-blue-500"
                      aria-label="Forgot Password Link"
                    >
                      {"Forgot Password?"}
                    </a>
                  </div>
                  <div
                    class="form-divider flex items-center text-gray-600 mb-6"
                    aria-hidden="true"
                  >
                    <div class="flex-1 h-px bg-gray-600"></div>
                    <div class="mx-2">{"Or login with"}</div>
                    <div class="flex-1 h-px bg-gray-600"></div>
                  </div>
                  <div class="social-buttons flex space-x-2">
                    <button
                      class="btn facebook-btn flex items-center justify-center bg-blue-800 text-white text-3xl rounded-lg w-full h-12 hover:bg-blue-700"
                      aria-label="Login with Facebook"
                    >
                      <i class="fab fa-facebook"></i>
                    </button>
                    <button
                      class="btn google-btn flex items-center justify-center bg-red-600 text-white text-3xl rounded-lg w-full h-12 hover:bg-red-500"
                      aria-label="Login with Google"
                    >
                      <i class="fab fa-google"></i>
                    </button>
                    <button
                      class="btn linkedin-btn flex items-center justify-center bg-blue-600 text-white text-3xl rounded-lg w-full h-12 hover:bg-blue-500"
                      aria-label="Login with LinkedIn"
                    >
                      <i class="fab fa-linkedin"></i>
                    </button>
                    <button
                      class="btn twitter-btn flex items-center justify-center bg-blue-400 text-white text-3xl rounded-lg w-full h-12 hover:bg-blue-300"
                      aria-label="Login with Twitter"
                    >
                      <i class="fab fa-twitter"></i>
                    </button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        </section>
    }
}
