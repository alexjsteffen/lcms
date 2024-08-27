use yew::prelude::*;

pub struct PageNotFound;

/// Implements the `Component` trait for the `PageNotFound` struct.
///
/// The `PageNotFound` struct represents a component for rendering a "Page not found" message.
/// It displays a hero section with a title and subtitle indicating that the page does not exist.
///
/// # Example
///
/// ```rust
/// // Create a new instance of `PageNotFound`
/// let page_not_found = PageNotFound::create(&ctx);
///
/// // Render the component
/// let html = page_not_found.view(&ctx);
/// ```
///
/// # Filepath
///
/// The code for this component is located at `/Users/ajhs/Repositories/tp.ajhs.li/src/pages/page_not_found.rs`.
///
/// # Generic Parameters
///
/// - `Message`: This component does not emit any messages.
/// - `Properties`: This component does not have any properties.
///
/// # Methods
///
/// - `create`: Creates a new instance of `PageNotFound`.
/// - `view`: Renders the component and returns the corresponding HTML.
///
/// # Example Usage
///
/// ```rust
/// // Create a new instance of `PageNotFound`
/// let page_not_found = PageNotFound::create(&ctx);
///
/// // Render the component
/// let html = page_not_found.view(&ctx);
/// ```
impl Component for PageNotFound {
    type Message = (); // Add the missing `type Message` item
    type Properties = (); // Add the missing `type Properties` item

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Page not found" }
                        </h1>
                        <h2 class="subtitle">
                            { "This page does not exist. Sorry." }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}
