pub mod countries;

use crate::countries::COUNTRY_CODES;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Props for a custom input component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The type of the input, e.g., "text", "password", etc.
    #[prop_or("text")]
    pub input_type: &'static str,

    /// The label to be displayed for the input field.
    #[prop_or_default]
    pub label: &'static str,

    /// The name of the input field, used for form submission and accessibility.
    #[prop_or_default]
    pub name: &'static str,

    /// Indicates whether the input is required or not.
    #[prop_or_default]
    pub required: bool,

    /// A reference to the DOM node of the input element.
    pub input_ref: NodeRef,

    /// The error message to display when there is a validation error.
    #[prop_or_default]
    pub error_message: &'static str,

    /// The CSS class to be applied to all inner elements.
    #[prop_or_default]
    pub form_input_class: &'static str,

    /// The CSS class to be applied to the inner input element and icon.
    #[prop_or_default]
    pub form_input_field_class: &'static str,

    /// The CSS class to be applied to the label for the input element.
    #[prop_or_default]
    pub form_input_label_class: &'static str,

    /// The CSS class to be applied to the input element.
    #[prop_or_default]
    pub form_input_input_class: &'static str,

    /// The CSS class to be applied to the error div element.
    #[prop_or_default]
    pub form_input_error_class: &'static str,

    /// The CSS class to be applied to the icon element.
    #[prop_or_default]
    pub icon_class: &'static str,

    /// The state handle for managing the value of the input.
    pub input_handle: UseStateHandle<String>,

    /// The state handle for managing the validity state of the input.
    pub input_valid_handle: UseStateHandle<bool>,

    /// A callback function to validate the input value. It takes a `String` as input and returns a `bool`.
    pub validate_function: Callback<String, bool>,

    /// The icon when the password is visible. Assuming fontawesome icons is used by default.
    #[prop_or("fa fa-eye")]
    pub eye_active: &'static str,

    /// The icon when the password is not visible. Assuming fontawesome icons is used by default.
    #[prop_or("fa fa-eye-slash")]
    pub eye_disabled: &'static str,

    // Additional props for accessibility and SEO:
    /// The ID attribute of the input element.
    #[prop_or_default]
    pub input_id: &'static str,

    /// The placeholder text to be displayed in the input element.
    #[prop_or_default]
    pub input_placeholder: &'static str,

    /// The aria-label attribute for screen readers, providing a label for accessibility.
    #[prop_or_default]
    pub aria_label: &'static str,

    /// The aria-required attribute for screen readers, indicating whether the input is required.
    #[prop_or("true")]
    pub aria_required: &'static str,

    /// The aria-invalid attribute for screen readers, indicating whether the input value is invalid.
    #[prop_or("true")]
    pub aria_invalid: &'static str,

    /// The aria-describedby attribute for screen readers, describing the input element's error message.
    #[prop_or_default]
    pub aria_describedby: &'static str,
}

/// custom_input_component
/// A custom input component that handles user input and validation.
///
/// # Arguments
/// * `props` - The properties of the component.
///   - `input_valid_handle` - A handle to track the validity of the input.
///   - `aria_invalid` - A string representing the 'aria-invalid' attribute value for accessibility. Defaults to "true".
///   - `aria_required` - A string representing the 'aria-required' attribute value for accessibility. Defaults to "true".
///   - `input_type` - The type of the input element. Defaults to "text".
///   - `input_ref` - A reference to the input element.
///   - `input_handle` - A handle to set the value of the input.
///   - `validate_function` - A callback function to validate the input value.
///
/// # Returns
/// (Html): An HTML representation of the input component.
///
/// # Examples
/// ```
/// // Example of using the custom_input_component
/// use regex::Regex;
/// use serde::{Deserialize, Serialize};
/// use input_yew::CustomInput;
/// use yew::prelude::*;
///
/// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// struct LoginUserSchema {
///     email: String,
///     password: String,
/// }
///
/// fn validate_email(email: String) -> bool {
///     let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
///     pattern.is_match(&email)
/// }
///
/// fn validate_password(password: String) -> bool {
///     !&password.is_empty()
/// }
///
/// #[function_component(LoginFormOne)]
/// pub fn login_form_one() -> Html {
///     let error_handle = use_state(String::default);
///     let error = (*error_handle).clone();;
///
///     let email_valid_handle = use_state(|| true);
///     let email_valid = (*email_valid_handle).clone();;
///
///     let password_valid_handle = use_state(|| true);
///     let password_valid = (*password_valid_handle).clone();;
///
///     let input_email_ref = use_node_ref();
///     let input_email_handle = use_state(String::default);
///     let input_email = (*input_email_handle).clone();;
///
///     let input_password_ref = use_node_ref();
///     let input_password_handle = use_state(String::default);
///     let input_password = (*input_password_handle).clone();;
///
///     let onsubmit = Callback::from(move |event: SubmitEvent| {
///         event.prevent_default();
///
///         let email_ref = input_password.clone();
///         let password_ref = input_password.clone();
///         let error_handle = error_handle.clone();
///
///         // Custom logic for your endpoint goes here: `spawn_local`
///     });
///
///     html! {
///         <div class="form-one-content" role="main" aria-label="Sign In Form">
///           <div class="text">
///             <h2>{"Sign In"}</h2>
///             if !error.is_empty() {
///               <div class="error">{error}</div>
///             }
///           </div>
///           <form action="#" aria-label="Sign In Form" onsubmit={onsubmit}>
///               <CustomInput
///                 input_type={"text"}
///                 input_handle={input_email_handle}
///                 name={"email"}
///                 input_ref={input_email_ref}
///                 input_placeholder={"Email"}
///                 icon_class={"fas fa-user"}
///                 error_message={"Enter a valid email address"}
///                 form_input_field_class={"form-one-field"}
///                 form_input_error_class={"error-txt"}
///                 required={true}
///                 input_valid_handle={email_valid_handle}
///                 validate_function={validate_email}
///               />
///               <CustomInput
///                 input_type={"password"}
///                 input_handle={input_password_handle}
///                 name={"password"}
///                 input_ref={input_password_ref}
///                 input_placeholder={"Password"}
///                 icon_class={"fas fa-lock"}
///                 error_message={"Password can't be blank!"}
///                 form_input_field_class={"form-one-field"}
///                 form_input_error_class={"error-txt"}
///                 required={true}
///                 input_valid_handle={password_valid_handle}
///                 validate_function={validate_password}
///                 eye_active={"fa fa-eye"}
///                 eye_disabled={"fa fa-eye-slash"}
///               />
///             <div class="form-one-forgot-pass">
///               <a href="#" aria-label="Forgot Password?">{"Forgot Password?"}</a>
///             </div>
///             <button type="submit">{"Sign in"}</button>
///             <div class="sign-up">
///               {"Not a member?"}
///               <a href="#" aria-label="Sign up now">{"Sign up now"}</a>
///             </div>
///           </form>
///         </div>
///     }
/// }
/// ```
#[function_component(CustomInput)]
pub fn custom_input(props: &Props) -> Html {
    let eye_active_handle = use_state(|| false);
    let eye_active = *eye_active_handle;

    let input_country_ref = use_node_ref();
    let country_handle = use_state(String::default);
    let country = (*country_handle).clone();

    let password_type_handle = use_state(|| "password");
    let password_type = *password_type_handle;

    let input_valid = *props.input_valid_handle;

    let aria_invalid = props.aria_invalid;

    let eye_icon_active = props.eye_active;

    let eye_icon_disabled = props.eye_disabled;

    let aria_required = props.aria_required;

    let input_type = props.input_type;

    let onchange = {
        let input_ref = props.input_ref.clone();
        let input_handle = props.input_handle.clone();
        let input_valid_handle = props.input_valid_handle.clone();
        let validate_function = props.validate_function.clone();

        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                input_handle.set(value);
                input_valid_handle.set(validate_function.emit(input.value()));
            }
        })
    };

    let on_select_change = {
        let input_country_ref = input_country_ref.clone();
        let input_handle = props.input_handle.clone();
        let country_handle = country_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = input_country_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                country_handle.set(value);
                input_handle.set(input.value());
            }
        })
    };

    let on_phone_number_input = {
        let input_ref = props.input_ref.clone();
        let input_handle = props.input_handle.clone();
        let country_handle = country_handle;
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                for (code, _, _, _, _, _) in &COUNTRY_CODES {
                    if code.starts_with(&input.value()) {
                        country_handle.set(input.value());
                        break;
                    }
                }
                // Filter out non-numeric characters
                let numeric_value: String =
                    input.value().chars().filter(|c| c.is_numeric()).collect();
                input_handle.set('+'.to_string() + &numeric_value);
            }
        })
    };

    let on_toggle_password = {
        Callback::from(move |_| {
            if eye_active {
                password_type_handle.set("password")
            } else {
                password_type_handle.set("text")
            }
            eye_active_handle.set(!eye_active);
        })
    };

    let input_tag = match (*input_type).into() {
        "password" => html! {
            <>
                <input
                    type={password_type}
                    class={props.form_input_input_class}
                    id={props.input_id}
                    name={props.name}
                    value={(*props.input_handle).clone()}
                    ref={props.input_ref.clone()}
                    placeholder={props.input_placeholder}
                    aria-label={props.aria_label}
                    aria-required={aria_required}
                    aria-invalid={aria_invalid}
                    aria-describedby={props.aria_describedby}
                    oninput={onchange}
                    required={props.required}
                />
                <span
                    class={format!("toggle-button {}", if eye_active { eye_icon_active } else { eye_icon_disabled })}
                    onclick={on_toggle_password}
                />
            </>
        },
        "textarea" => html! {
            <textarea
                class={props.form_input_input_class}
                id={props.input_id}
                name={props.name}
                value={(*props.input_handle).clone()}
                ref={props.input_ref.clone()}
                placeholder={props.input_placeholder}
                aria-label={props.aria_label}
                aria-required={aria_required}
                aria-invalid={aria_invalid}
                aria-describedby={props.aria_describedby}
                oninput={onchange}
                required={props.required}
            />
        },
        "tel" => html! {
            <>
                <select ref={input_country_ref} onchange={on_select_change}>
                    { for COUNTRY_CODES.iter().map(|(code, emoji, _, name, _, _)| {
                            let selected = *code == country;
                            html! {
                                <option value={*code} selected={selected}>{ format!("{} {} {}", emoji, name, code) }</option>
                            }
                        }) }
                </select>
                <input
                    type="tel"
                    id="telNo"
                    name="telNo"
                    size="20"
                    minlength="9"
                    value={(*props.input_handle).clone()}
                    maxlength="14"
                    class={props.form_input_input_class}
                    placeholder={props.input_placeholder}
                    aria-label={props.aria_label}
                    aria-required={aria_required}
                    aria-invalid={aria_invalid}
                    oninput={on_phone_number_input}
                    ref={props.input_ref.clone()}
                />
            </>
        },
        _ => html! {
            <input
                type={input_type}
                class={props.form_input_input_class}
                id={props.input_id}
                value={(*props.input_handle).clone()}
                name={props.name}
                ref={props.input_ref.clone()}
                placeholder={props.input_placeholder}
                aria-label={props.aria_label}
                aria-required={aria_required}
                aria-invalid={aria_invalid}
                aria-describedby={props.aria_describedby}
                oninput={onchange}
                required={props.required}
            />
        },
    };

    html! {
        <div class={props.form_input_class}>
            <label class={props.form_input_label_class} for={props.input_id}>{ props.label }</label>
            <div class={props.form_input_field_class}>
                { input_tag }
                <span class={props.icon_class} />
            </div>
            if !input_valid {
                <div class={props.form_input_error_class} id={props.aria_describedby}>
                    { &props.error_message }
                </div>
            }
        </div>
    }
}
