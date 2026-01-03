//! Simple UI example with basic elements.
//!
//! Tests basic Slint elements: Rectangle, Text, and simple properties.

slint::slint! {
    export component SimpleUI inherits Window {
        width: 400px;
        height: 300px;
        title: "Simple UI";

        Rectangle {
            x: 10px;
            y: 10px;
            width: 100px;
            height: 50px;
            background: #3498db;

            Text {
                text: "Hello";
                color: white;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }

        Text {
            x: 10px;
            y: 80px;
            text: "Simple Slint UI";
            font-size: 16px;
            color: #2c3e50;
        }
    }
}
