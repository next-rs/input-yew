# 💡 Input Yew

![demo](./assets/demo.gif)

## 📜 Prologue

This custom reusable input functional component is a solution built exclusively for the Rust Yew framework. Seamlessly integrating into your Yew applications, it combines powerful functionality with SEO optimization and comprehensive accessibility features. With a focus on reusability and customizability, this component empowers you to effortlessly create dynamic input fields that adapt to various forms of user input.

## 🤔 Why is this Component Useful?

The Yew Custom Reusable Input Component is a powerful tool designed to make your life as a Yew developer easier and more enjoyable. Let's explore some of the reasons why this component is an essential addition to your web development arsenal:

1. 🔄 Reusability: No repetitive code! This component is built to be highly reusable, allowing you to sprinkle it across your application without any fuss.

1. 🎨 Customizability: You can now fine-tune the appearance and behavior of the input component to fit your specific needs and aesthetics.

1. ✔️ Validations: You can easily add your custom validation functions.

1. 🎫 Event Handling: The component exposes the `oninput` event handler, making it super easy to implement dynamic behavior based on your interactions.

1. ♿ Accessibility: This compoenent was designed with accessibility in mind, ensuring that it's user-friendly and perceivable by all, regardless of ability.

1. ❌ Error Handling: When users provide invalid input, the component gracefully displays clear error messages, guiding them towards valid data entry and enhancing the overall user experience.

## Installation ⚙️

You can quickly integrate the Yew Custom Reusable Input Component into your Yew project by following these simple steps:

1. First, make sure you have Yew set up in your project. If not, check out the [Yew documentation](https://yew.rs/docs/getting-started/introduction) for installation instructions.

2. Then, install the input component package using your preferred package manager:

   ```bash
   $ cargo add input-yew
   ```

3. Finally, import the component into your Yew application and start using it to power up your forms and user interactions.

## 🛠️ Usage

Using this custom reusable input component is a breeze! Simply follow these steps:

1. Import the component into your Yew application:

   ```rust
   // Add these lines at the beginning of your file
   use yew::prelude::*;
   use input_yew::CustomInput;
   ```

1. Use the `CustomInput` component wherever you need an input field:

   ```rust
   fn validate_email(email: String) -> bool {
       let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
       pattern.is_match(&email)
   }


   #[function_component(LoginForm)]
   pub fn login_form() -> Html {
       let input_email_ref = use_node_ref();
       let input_email_handle = use_state(String::default);
       let email_valid_handle = use_state(|| true);
       let onsubmit = Callback::from(move |event: SubmitEvent| {};
       html! {
             <form action="#" aria-label="Sign In Form" onsubmit={onsubmit}>
                 <CustomInput
                   input_type={Some("text".to_string())}
                   label={"".to_string()}
                   input_handle={input_email_handle}
                   name={"email".to_string()}
                   input_ref={input_email_ref}
                   input_placeholder={"Email".to_string()}
                   icon_class={"fas fa-user".to_string()}
                   icon={"fas fa-user".to_string()}
                   error_message={"Enter a valid email address".to_string()}
                   form_input_class={"".to_string()}
                   form_input_field_class={"form-one-field".to_string()}
                   form_input_label_class={"".to_string()}
                   form_input_input_class={"".to_string()}
                   form_input_error_class={"error-txt".to_string()}
                   required={true}
                   input_valid_handle={email_valid_handle}
                   validate_function={validate_email}
                 />
             </form>
       }
   }
   ```

1. Customize the input component's appearance and behavior according to your project requirements.

## Contribution 🤝

We welcome contributions from the community to make the Input Yew even better! Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate and create something amazing together!

## License 📜

The Yew Custom Reusable Input Component is licensed under the `Apache-2.0` License, giving you the freedom to use, modify, and distribute it as you see fit. Please check the `LICENSE` file for more details.

## Epilogue 📝

Congratulations! You're now equipped with a fantastic Yew Custom Reusable Input Component that will supercharge your web applications with its flexibility, user-friendliness, and robustness. Happy coding, and may your projects thrive with this powerful tool! 🎉