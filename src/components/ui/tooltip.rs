use leptos::prelude::*;

/// A tooltip component that displays helpful information when hovering over an info icon
#[component]
pub fn Tooltip(
    /// The text to display in the tooltip
    text: &'static str,
) -> impl IntoView {
    view! {
        <span class="group relative inline-flex items-center ml-1">
            <svg
                class="h-4 w-4 text-gray-400 hover:text-gray-600 cursor-help"
                fill="currentColor"
                viewBox="0 0 20 20"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    fill-rule="evenodd"
                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                    clip-rule="evenodd"
                />
            </svg>
            <span class="invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-opacity absolute left-0 top-6 z-10 w-64 px-3 py-2 text-sm text-white bg-gray-900 rounded-lg shadow-lg">
                {text}
                <svg
                    class="absolute text-gray-900 h-2 left-3 top-[-6px]"
                    x="0px"
                    y="0px"
                    viewBox="0 0 255 255"
                    xml:space="preserve"
                >
                    <polygon class="fill-current" points="0,255 127.5,0 255,255" />
                </svg>
            </span>
        </span>
    }
}
