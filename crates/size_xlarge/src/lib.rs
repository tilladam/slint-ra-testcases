//! Extra-large size example (~200+ lines of Slint).
//!
//! Stress test for macro expansion performance.
//! Compare expansion time against small, medium, and large.

slint::slint! {
    import { Button, Slider, ComboBox, CheckBox, LineEdit, ProgressIndicator, VerticalBox, HorizontalBox, GroupBox, TabWidget } from "std-widgets.slint";

    // Multiple struct definitions
    export struct User {
        id: int,
        name: string,
        email: string,
        is-active: bool,
        role: string,
    }

    export struct Product {
        id: int,
        name: string,
        price: float,
        quantity: int,
        category: string,
    }

    export struct Order {
        id: int,
        user-id: int,
        product-id: int,
        quantity: int,
        total: float,
        status: string,
    }

    export struct AppSettings {
        theme: string,
        language: string,
        notifications-enabled: bool,
        auto-save: bool,
        font-size: int,
    }

    export struct DashboardStats {
        total-users: int,
        total-products: int,
        total-orders: int,
        revenue: float,
    }

    // Global state
    export global AppState {
        in-out property <User> current-user: {
            id: 1,
            name: "Admin",
            email: "admin@example.com",
            is-active: true,
            role: "Administrator",
        };

        in-out property <AppSettings> settings: {
            theme: "light",
            language: "en",
            notifications-enabled: true,
            auto-save: true,
            font-size: 14,
        };

        in-out property <DashboardStats> stats: {
            total-users: 0,
            total-products: 0,
            total-orders: 0,
            revenue: 0.0,
        };

        callback refresh-stats();
        callback save-settings();
        callback logout();
    }

    // Theme global
    export global Theme {
        out property <color> primary: #3498db;
        out property <color> secondary: #22cc71;
        out property <color> danger: #dd4c3c;
        out property <color> warning: #f39c12;
        out property <color> info: #17a2b8;
        out property <color> light: #ecf0f1;
        out property <color> dark: #2c3e50;
        out property <color> background: #ffffff;
        out property <color> text: #333333;
        out property <length> spacing: 10px;
        out property <length> padding: 15px;
        out property <length> border-radius: 4px;
    }

    // Reusable card component
    component Card inherits Rectangle {
        in property <string> title;
        background: Theme.background;
        border-radius: Theme.border-radius;
        drop-shadow-blur: 5px;
        drop-shadow-color: #00000020;

        VerticalLayout {
            padding: Theme.padding;
            spacing: Theme.spacing;

            if title != "": Text {
                text: title;
                font-size: 18px;
                font-weight: 600;
                color: Theme.dark;
            }

            @children
        }
    }

    // Stat card component
    component StatCard inherits Card {
        in property <string> label;
        in property <string> value;
        in property <color> accent: Theme.primary;

        HorizontalLayout {
            spacing: Theme.spacing;

            Rectangle {
                width: 4px;
                background: accent;
                border-radius: 2px;
            }

            VerticalLayout {
                Text {
                    text: label;
                    color: Theme.text;
                    font-size: 12px;
                }
                Text {
                    text: value;
                    color: accent;
                    font-size: 24px;
                    font-weight: 700;
                }
            }
        }
    }

    // Main application component
    export component XLargeComponent inherits Window {
        width: 1200px;
        height: 800px;
        title: "Dashboard Application";
        background: Theme.light;

        // Properties
        in property <[User]> users: [];
        in property <[Product]> products: [];
        in property <[Order]> orders: [];
        in-out property <int> active-tab: 0;
        in-out property <string> search-query: "";
        in-out property <bool> sidebar-collapsed: false;

        // Callbacks
        callback add-user(User);
        callback update-user(User);
        callback delete-user(int);
        callback add-product(Product);
        callback update-product(Product);
        callback delete-product(int);
        callback create-order(Order);
        callback cancel-order(int);
        callback search(string);
        callback export-data(string);

        // Helper functions
        pure function format-currency(value: float) -> string {
            return "$" + round(value * 100) / 100;
        }

        pure function get-status-color(status: string) -> color {
            if status == "completed" { return Theme.secondary; }
            if status == "pending" { return Theme.warning; }
            if status == "cancelled" { return Theme.danger; }
            return Theme.info;
        }

        HorizontalLayout {
            // Sidebar
            Rectangle {
                width: sidebar-collapsed ? 60px : 200px;
                background: Theme.dark;

                VerticalLayout {
                    padding: Theme.padding;
                    spacing: 20px;

                    // Logo area
                    Rectangle {
                        height: 50px;
                        Text {
                            text: sidebar-collapsed ? "D" : "Dashboard";
                            color: white;
                            font-size: 20px;
                            font-weight: 700;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                    }

                    // Nav items
                    for nav-item[idx] in ["Home", "Users", "Products", "Orders", "Settings"]: Rectangle {
                        height: 40px;
                        background: idx == active-tab ? Theme.primary : transparent;
                        border-radius: Theme.border-radius;

                        HorizontalLayout {
                            padding-left: 10px;
                            spacing: 10px;

                            Text {
                                text: nav-item;
                                color: white;
                                vertical-alignment: center;
                                overflow: elide;
                            }
                        }

                        TouchArea {
                            clicked => { active-tab = idx; }
                        }
                    }

                    // Spacer
                    Rectangle { vertical-stretch: 1; }

                    // Collapse button
                    Rectangle {
                        height: 40px;
                        background: #ffffff20;
                        border-radius: Theme.border-radius;

                        Text {
                            text: sidebar-collapsed ? ">" : "<";
                            color: white;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }

                        TouchArea {
                            clicked => { sidebar-collapsed = !sidebar-collapsed; }
                        }
                    }
                }
            }

            // Main content
            VerticalLayout {
                padding: Theme.padding;
                spacing: Theme.spacing;

                // Header
                HorizontalLayout {
                    height: 60px;
                    spacing: Theme.spacing;

                    LineEdit {
                        horizontal-stretch: 1;
                        placeholder-text: "Search...";
                        text <=> search-query;
                        edited => { root.search(self.text); }
                    }

                    Button {
                        text: "Export";
                        clicked => { root.export-data("csv"); }
                    }

                    Rectangle {
                        width: 40px;
                        height: 40px;
                        background: Theme.primary;
                        border-radius: 20px;

                        Text {
                            text: "U";
                            color: white;
                            font-size: 18px;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                    }
                }

                // Stats row
                HorizontalLayout {
                    spacing: Theme.spacing;

                    StatCard {
                        horizontal-stretch: 1;
                        label: "Total Users";
                        value: "" + AppState.stats.total-users;
                        accent: Theme.primary;
                    }

                    StatCard {
                        horizontal-stretch: 1;
                        label: "Products";
                        value: "" + AppState.stats.total-products;
                        accent: Theme.secondary;
                    }

                    StatCard {
                        horizontal-stretch: 1;
                        label: "Orders";
                        value: "" + AppState.stats.total-orders;
                        accent: Theme.warning;
                    }

                    StatCard {
                        horizontal-stretch: 1;
                        label: "Revenue";
                        value: format-currency(AppState.stats.revenue);
                        accent: Theme.info;
                    }
                }

                // Content area
                Card {
                    vertical-stretch: 1;
                    title: active-tab == 0 ? "Dashboard" : active-tab == 1 ? "Users" : active-tab == 2 ? "Products" : active-tab == 3 ? "Orders" : "Settings";

                    // Tab content would go here based on active-tab
                    if active-tab == 0: VerticalLayout {
                        Text { text: "Welcome to the dashboard!"; }
                        Text { text: "Select a section from the sidebar."; color: Theme.text; }
                    }

                    if active-tab == 1: VerticalLayout {
                        spacing: 5px;
                        for user in users: Rectangle {
                            height: 50px;
                            background: user.is-active ? #d5f5e3 : #fadbd8;
                            border-radius: Theme.border-radius;

                            HorizontalLayout {
                                padding: 10px;
                                spacing: 20px;
                                Text { text: user.name; horizontal-stretch: 1; }
                                Text { text: user.email; horizontal-stretch: 1; }
                                Text { text: user.role; }
                            }
                        }
                    }

                    if active-tab == 2: VerticalLayout {
                        spacing: 5px;
                        for product in products: Rectangle {
                            height: 50px;
                            background: Theme.light;
                            border-radius: Theme.border-radius;

                            HorizontalLayout {
                                padding: 10px;
                                spacing: 20px;
                                Text { text: product.name; horizontal-stretch: 1; }
                                Text { text: format-currency(product.price); }
                                Text { text: "Qty: " + product.quantity; }
                            }
                        }
                    }

                    if active-tab == 3: VerticalLayout {
                        spacing: 5px;
                        for order in orders: Rectangle {
                            height: 50px;
                            background: Theme.light;
                            border-radius: Theme.border-radius;

                            HorizontalLayout {
                                padding: 10px;
                                spacing: 20px;
                                Text { text: "#" + order.id; }
                                Text { text: format-currency(order.total); horizontal-stretch: 1; }
                                Rectangle {
                                    width: 80px;
                                    background: get-status-color(order.status);
                                    border-radius: 10px;
                                    Text {
                                        text: order.status;
                                        color: white;
                                        font-size: 12px;
                                        horizontal-alignment: center;
                                        vertical-alignment: center;
                                    }
                                }
                            }
                        }
                    }

                    if active-tab == 4: VerticalLayout {
                        spacing: Theme.spacing;

                        HorizontalLayout {
                            Text { text: "Theme:"; min-width: 100px; }
                            ComboBox {
                                model: ["Light", "Dark", "System"];
                                current-index: 0;
                            }
                        }

                        HorizontalLayout {
                            Text { text: "Language:"; min-width: 100px; }
                            ComboBox {
                                model: ["English", "Spanish", "French", "German"];
                                current-index: 0;
                            }
                        }

                        CheckBox {
                            text: "Enable notifications";
                            checked: AppState.settings.notifications-enabled;
                        }

                        CheckBox {
                            text: "Auto-save";
                            checked: AppState.settings.auto-save;
                        }

                        HorizontalLayout {
                            Text { text: "Font size:"; min-width: 100px; }
                            Slider {
                                minimum: 10;
                                maximum: 24;
                                value: AppState.settings.font-size;
                            }
                        }

                        Button {
                            text: "Save Settings";
                            clicked => { AppState.save-settings(); }
                        }
                    }
                }

                // Footer
                HorizontalLayout {
                    height: 30px;

                    Text {
                        text: "Logged in as: " + AppState.current-user.name;
                        color: Theme.text;
                        font-size: 12px;
                    }

                    Rectangle { horizontal-stretch: 1; }

                    Text {
                        text: "v1.0.0";
                        color: Theme.text;
                        font-size: 12px;
                    }
                }
            }
        }
    }
}
