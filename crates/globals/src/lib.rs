//! Global singletons example.
//!
//! Tests global patterns:
//! - Global singleton definitions
//! - Accessing globals from components
//! - Global properties and callbacks

slint::slint! {
    // Global for application state
    export global AppState {
        in-out property <string> current-user: "Guest";
        in-out property <bool> is-logged-in: false;
        in-out property <int> notification-count: 0;

        callback login(string);
        callback logout();
    }

    // Global for theming
    export global Theme {
        out property <color> primary-color: #3498db;
        out property <color> secondary-color: #22cc71;
        out property <color> background-color: #ecf0f1;
        out property <color> text-color: #2c3e50;
        out property <length> default-spacing: 10px;
        out property <length> default-padding: 15px;
    }

    // Global for settings
    export global Settings {
        in-out property <bool> dark-mode: false;
        in-out property <int> font-size: 14;
        in-out property <string> language: "en";

        callback settings-changed();
    }

    export component GlobalsTest inherits Window {
        width: 400px;
        height: 300px;
        background: Theme.background-color;

        VerticalLayout {
            spacing: Theme.default-spacing;
            padding: Theme.default-padding;

            // Using global state
            Text {
                text: AppState.is-logged-in ? "Welcome, " + AppState.current-user : "Please log in";
                color: Theme.text-color;
                font-size: Settings.font-size * 1px;
            }

            // Button using theme colors
            Rectangle {
                height: 40px;
                background: Theme.primary-color;

                Text {
                    text: "Notifications: " + AppState.notification-count;
                    color: white;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }

                TouchArea {
                    clicked => {
                        if !AppState.is-logged-in {
                            AppState.login("User");
                        } else {
                            AppState.logout();
                        }
                    }
                }
            }
        }
    }
}
