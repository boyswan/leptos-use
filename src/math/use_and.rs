use crate::math::shared::use_binary_logic;
use leptos::prelude::*;
use leptos::reactive_graph::wrappers::read::Signal;
use paste::paste;

use_binary_logic!(
    /// Reactive `AND` condition.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_and)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::math::use_and;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (a, set_a) = signal(true);
    /// let (b, set_b) = signal(false);
    ///
    /// let a_and_b = use_and(a, b);
    /// #
    /// # view! { }
    /// # }
    /// ```
    // #[doc(cfg(feature = "math"))]
    and
    &&
);
