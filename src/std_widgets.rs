//! Standard widgets example.
//!
//! Tests importing and using widgets from std-widgets.slint:
//! - Button, Slider, ComboBox
//! - CheckBox, SpinBox
//! - LineEdit, TextEdit
//! - ProgressIndicator

slint::slint! {
    import { Button, Slider, ComboBox, CheckBox, SpinBox, LineEdit, ProgressIndicator, VerticalBox, HorizontalBox, GroupBox } from "std-widgets.slint";

    export component StdWidgetsTest inherits Window {
        width: 500px;
        height: 400px;
        title: "Standard Widgets Test";

        VerticalBox {
            GroupBox {
                title: "Buttons and Inputs";

                HorizontalBox {
                    Button {
                        text: "Click Me";
                        clicked => { debug("Button clicked"); }
                    }

                    Button {
                        text: "Disabled";
                        enabled: false;
                    }
                }

                HorizontalBox {
                    LineEdit {
                        placeholder-text: "Enter text...";
                    }

                    SpinBox {
                        minimum: 0;
                        maximum: 100;
                        value: 50;
                    }
                }
            }

            GroupBox {
                title: "Selection Controls";

                HorizontalBox {
                    CheckBox {
                        text: "Option 1";
                        checked: true;
                    }

                    CheckBox {
                        text: "Option 2";
                    }
                }

                ComboBox {
                    model: ["Choice A", "Choice B", "Choice C"];
                    current-index: 0;
                }
            }

            GroupBox {
                title: "Sliders and Progress";

                Slider {
                    minimum: 0;
                    maximum: 100;
                    value: 30;
                }

                ProgressIndicator {
                    progress: 0.6;
                }
            }
        }
    }
}
