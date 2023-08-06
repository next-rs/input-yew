use yew::web_sys::HtmlInputElement;
use yew::prelude::*;

/// Props for a custom input component.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The type of the input, e.g., "text", "email", "password", etc.
    pub input_type: Option<String>,

    /// The label to be displayed for the input field.
    pub label: String,

    /// The name of the input field, used for form submission and accessibility.
    pub name: String,

    /// The icon class to be used for displaying an icon alongside the input.
    pub icon: String,

    /// Indicates whether the input is required or not.
    pub required: bool,

    /// A reference to the DOM node of the input element.
    pub input_ref: NodeRef,

    /// The error message to display when there is a validation error.
    pub error_message: String,

    /// The CSS class to be applied to all inner elements.
    pub form_input_class: String,

    /// The CSS class to be applied to the inner input element and icon.
    pub form_input_field_class: String,

    /// The CSS class to be applied to the label for the input element.
    pub form_input_label_class: String,

    /// The CSS class to be applied to the input element.
    pub form_input_input_class: String,

    /// The CSS class to be applied to the error div element.
    pub form_input_error_class: String,

    /// The CSS class to be applied to the icon element.
    pub icon_class: String,

    /// The state handle for managing the value of the input.
    pub input_handle: UseStateHandle<String>,

    /// The state handle for managing the validity state of the input.
    pub input_valid_handle: UseStateHandle<bool>,

    /// A callback function to validate the input value. It takes a `String` as input and returns a `bool`.
    pub validate_function: Callback<String, bool>,

    // Additional props for accessibility and SEO:
    /// The ID attribute of the input element.
    pub input_id: Option<String>,

    /// The placeholder text to be displayed in the input element.
    pub input_placeholder: Option<String>,

    /// The aria-label attribute for screen readers, providing a label for accessibility.
    pub aria_label: Option<String>,

    /// The aria-required attribute for screen readers, indicating whether the input is required.
    pub aria_required: Option<String>,

    /// The aria-invalid attribute for screen readers, indicating whether the input value is invalid.
    pub aria_invalid: Option<String>,

    /// The aria-describedby attribute for screen readers, describing the input element's error message.
    pub aria_describedby: Option<String>,
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
/// use yew::prelude::*;
/// use input_yew::CustomInput;
///
/// fn validate_email(email: String) -> bool {
///     let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
///     pattern.is_match(&email)
/// }
///
/// #[function_component(LoginForm)]
/// pub fn login_form() -> Html {
///
///     let input_email_ref = use_node_ref();
///     let input_email_handle = use_state(String::default);
///     let email_valid_handle = use_state(|| true);
///     let onsubmit = Callback::from(move |event: SubmitEvent| {};
///
///     html! {
///           <form action="#" aria-label="Sign In Form" onsubmit={onsubmit}>
///               <CustomInput
///                 input_type={Some("text".to_string())}
///                 label={"".to_string()}
///                 input_handle={input_email_handle}
///                 name={"email".to_string()}
///                 input_ref={input_email_ref}
///                 input_placeholder={"Email".to_string()}
///                 icon_class={"fas fa-user".to_string()}
///                 icon={"fas fa-user".to_string()}
///                 error_message={"Enter a valid email address".to_string()}
///                 form_input_class={"".to_string()}
///                 form_input_field_class={"form-one-field".to_string()}
///                 form_input_label_class={"".to_string()}
///                 form_input_input_class={"".to_string()}
///                 form_input_error_class={"error-txt".to_string()}
///                 required={true}
///                 input_valid_handle={email_valid_handle}
///                 validate_function={validate_email}
///               />
///           </form>
///     }
/// }
/// ```
#[function_component(CustomInput)]
pub fn custom_input(props: &Props) -> Html {
    let input_valid = *props.input_valid_handle;

    let aria_invalid = props
        .aria_invalid
        .clone()
        .unwrap_or_else(|| "true".to_string());

    let aria_required = props
        .aria_required
        .clone()
        .unwrap_or_else(|| "true".to_string());

    let input_type = props
        .input_type
        .clone()
        .unwrap_or_else(|| "text".to_string());

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

    html! {
        <div class={props.form_input_class.clone()}>
            <label class={props.form_input_label_class.clone()} for={props.input_id.clone()}>
                {&props.label}
            </label>
            <div class={props.form_input_field_class.clone()}>
                <input
                    type={input_type}
                    class={props.form_input_input_class.clone()}
                    id={props.input_id.clone()}
                    name={props.name.clone()}
                    ref={props.input_ref.clone()}
                    placeholder={props.input_placeholder.clone()}
                    aria-label={props.aria_label.clone()}
                    aria-required={aria_required}
                    aria-invalid={aria_invalid}
                    aria-describedby={props.aria_describedby.clone()}
                    oninput={onchange}
                    required={props.required}
                />
                <span class={props.icon_class.clone()}></span>
            </div>
            if !input_valid {
                <div class={props.form_input_error_class.clone()} id={props.aria_describedby.clone()}>{&props.error_message}</div>
            }
        </div>
    }
}
