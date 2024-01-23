use crate::components::common::{validate_email, validate_input, LoginUserSchema};
use input_yew::CustomInput;
use regex::Regex;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[function_component(LoginFormOne)]
pub fn login_form_one() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let password_valid_handle = use_state(|| true);
    let password_valid = (*password_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(|| "sad".to_string());
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(|| "sad".to_string());
    let input_password = (*input_password_handle).clone();

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

    html! {
        <div
          class="min-h-screen bg-gradient-to-tr from-blue-500 to-purple-600 flex items-center justify-center"
        >
          <div class="w-96 bg-white rounded-lg shadow-lg p-8">
            <form class="flex flex-col space-y-6" onsubmit={onsubmit}>
              if !error.is_empty() {
                <div class="mb-3 error bg-red-600 text-white px-4 py-3 font-semibold rounded-md text-center text-base">{error}</div>
              }
              <span class="text-4xl text-gray-800 pb-6">{"Login"}</span>
              <CustomInput
                input_type={"tek"}
                label={"Email"}
                input_handle={input_email_handle}
                name={"username"}
                input_ref={input_email_ref}
                input_placeholder={"Your Email"}
                icon_class={"text-2xl fa fa-person text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"}
                error_message={"Enter a valid email address!"}
                form_input_field_class={"relative"}
                form_input_input_class={"input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"}
                form_input_error_class={"text-red-500 absolute text-sm mb-2"}
                required={true}
                input_valid_handle={email_valid_handle}
                validate_function={validate_email}
              />
              <CustomInput
                input_type={"password"}
                label={"Password"}
                input_handle={input_password_handle}
                name={"password"}
                input_ref={input_password_ref}
                input_placeholder={"Password"}
                icon_class={"text-2xl fa fa-lock text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"}
                error_message={"Password can't be blank!"}
                form_input_field_class={"relative mt-2 mb-2"}
                form_input_input_class={"input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"}
                form_input_error_class={"text-red-500 absolute text-sm"}
                required={true}
                input_valid_handle={password_valid_handle}
                validate_function={validate_input}
                eye_active={"cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye"}
                eye_disabled={"cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye-slash"}
              />
              <div class="w-full">
                <button
                  class="btn-social bg-indigo-600 hover:bg-indigo-700 text-white w-full py-3 rounded-lg text-base font-medium tracking-wide"
                  type="submit"
                >
                  <i class="fa fa-box-arrow-in-right me-2"></i>
                  {"Sign In"}
                </button>
              </div>
              <div class="w-full text-center">
                <span class="text-base text-gray-800">{"Not a member?"}</span>
                <a href="#" class="text-base text-gray-800">{"Sign Up Now"}</a>
              </div>
              <div class="mt-8">
                <div class="relative text-center text-gray-800 mt-4">
                  <span class="px-4 bg-white relative z-10">{"Or Sign In With"}</span>
                  <div class="absolute inset-0 flex items-center">
                    <div class="w-full h-px bg-gray-400"></div>
                  </div>
                </div>
                <div class="flex justify-center space-x-4 mt-12">
                  <button
                    class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Facebook"
                  >
                    <i class="fa fa-facebook"></i>
                  </button>
                  <button
                    class="btn-social bg-red-600 hover:bg-red-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Google"
                  >
                    <i class="fa fa-google"></i>
                  </button>
                  <button
                    class="btn-social bg-blue-400 hover:bg-blue-500 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Twitter"
                  >
                    <i class="fa fa-twitter"></i>
                  </button>
                  <button
                    class="btn-social bg-blue-900 hover:bg-blue-800 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with LinkedIn"
                  >
                    <i class="fa fa-linkedin"></i>
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
    }
}
