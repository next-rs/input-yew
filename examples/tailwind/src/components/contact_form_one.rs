use crate::components::common::{validate_email, validate_input, LoginUserSchema};
use input_yew::CustomInput;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[function_component(ContactFormOne)]
pub fn contact_form_one() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let name_valid_handle = use_state(|| true);
    let name_valid = (*name_valid_handle).clone();

    let subject_valid_handle = use_state(|| true);
    let subject_valid = (*subject_valid_handle).clone();

    let message_valid_handle = use_state(|| true);
    let message_valid = (*message_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_name_ref = use_node_ref();
    let input_name_handle = use_state(|| "afasfasf".to_string());
    let input_name = (*input_name_handle).clone();

    let input_subject_ref = use_node_ref();
    let input_subject_handle = use_state(String::default);
    let input_subject = (*input_subject_handle).clone();

    let input_message_ref = use_node_ref();
    let input_message_handle = use_state(String::default);
    let input_message = (*input_message_handle).clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_email.clone();
        let name_ref = input_name.clone();
        let subject_ref = input_subject.clone();
        let message_ref = input_message.clone();
        let error_handle = error_handle.clone();
        console::log_1(
            &format!(
                "Email: {}, Name: {}, Subject: {}, Message: {}",
                input_email, input_name, input_subject, input_message
            )
            .into(),
        );

        spawn_local(async move {
            let email_val = email_ref.clone();
            let _name_val = name_ref.clone();
            let subject_val = subject_ref.clone();
            let _message_val = message_ref.clone();

            let error_handle = error_handle.clone();
            if email_valid && name_valid && subject_valid && message_valid {
                // TODO: create a contact us endpoint
                let response = login_user(email_val.to_string(), subject_val.to_string()).await;
                match response {
                    Ok(_) => {
                        console::log_1(&"success".into());
                        let window: Window = web_sys::window().expect("window not available");
                        let location = window.location();
                        let _ = location.set_href("/home");
                    }
                    Err(err) => {
                        error_handle.set(err);
                    }
                }
            } else {
                error_handle.set("Please provide valid contact information!".into());
            }
        });
    });

    html! {
        <section class="bg-gradient-to-r from-purple-500 to-yellow-700 h-screen">
          <div class="flex items-center justify-center h-full">
            <div
              class="w-full max-w-5xl bg-white rounded-lg overflow-hidden shadow-lg p-8 mx-4 md:mx-8 md:grid md:grid-cols-2 md:gap-10"
            >
              <div
                class="contact1-pic hidden md:block w-full mb-6 md:mb-0 md:mr-6 flex justify-center items-center"
              >
                <img
                  src="https://cdn.pixabay.com/photo/2023/06/09/19/37/letter-8052497_960_720.png"
                  alt="Contact Image"
                  class="w-full max-w-2xl h-auto"
                  role="img"
                  aria-label="Contact Image"
                />
              </div>
              <form class="w-full flex-grow" aria-label="Contact Form" onsubmit={onsubmit}>
                if !error.is_empty() {
                  <div class="mb-3 error bg-red-600 text-white px-4 py-3 font-semibold rounded-md text-center text-base">{error}</div>
                }
                <span
                  class="text-3xl font-bold text-gray-900 block text-center mb-6"
                >
                  {"Contact US"}
                </span>
                <CustomInput
                  input_type={"text"}
                  input_handle={input_name_handle}
                  name={"name"}
                  input_ref={input_name_ref}
                  input_placeholder={"Your Name"}
                  error_message={"Name can't be blank"}
                  form_input_field_class={"wrap-input1 validate-input mb-6"}
                  form_input_input_class={"w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"}
                  form_input_error_class={"text-red-500 text-sm mb-2"}
                  required={true}
                  input_valid_handle={name_valid_handle}
                  validate_function={validate_input}
                />
                <CustomInput
                  input_type={"text"}
                  input_handle={input_email_handle}
                  name={"email"}
                  input_ref={input_email_ref}
                  input_placeholder={"Your Email"}
                  error_message={"Enter a valid email address"}
                  form_input_field_class={"wrap-input1 validate-input mb-6"}
                  form_input_input_class={"w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"}
                  form_input_error_class={"text-red-500 text-sm mb-2"}
                  required={true}
                  input_valid_handle={email_valid_handle}
                  validate_function={validate_email}
                />
                <CustomInput
                  input_type={"text"}
                  input_handle={input_subject_handle}
                  name={"subject"}
                  input_ref={input_subject_ref}
                  input_placeholder={"Your Subject"}
                  error_message={"Subject can't be blank"}
                  form_input_field_class={"wrap-input1 validate-input mb-6"}
                  form_input_input_class={"w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"}
                  form_input_error_class={"text-red-500 text-sm mb-2"}
                  required={true}
                  input_valid_handle={subject_valid_handle}
                  validate_function={validate_input}
                />
                <CustomInput
                  input_type={"textarea"}
                  input_handle={input_message_handle}
                  name={"message"}
                  input_ref={input_message_ref}
                  input_placeholder={"Your Message"}
                  error_message={"Message can't be blank"}
                  form_input_field_class={"wrap-input1 validate-input mb-6"}
                  form_input_input_class={"w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"}
                  form_input_error_class={"text-red-500 text-sm mb-2"}
                  required={true}
                  input_valid_handle={message_valid_handle}
                  validate_function={validate_input}
                />
                <div class="container-contact1-form-btn">
                  <button
                    class="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-3 rounded-lg transition duration-300"
                    type="submit"
                    aria-label="Submit"
                  >
                    <span>
                      {"Send Email"}
                      <i class="fa fa-long-arrow-right ml-2" aria-hidden="true"></i>
                    </span>
                  </button>
                </div>
              </form>
            </div>
          </div>
        </section>
    }
}
