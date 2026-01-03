//! Many components stress test.
//!
//! Tests macro expansion with many component definitions in a single macro.
//! This stresses the component registry and code generation.

slint::slint! {
    // Component 1
    export component Component1 {
        in property <int> value: 1;
        Rectangle { background: #dd4c3c; Text { text: "C1"; color: white; } }
    }

    // Component 2
    export component Component2 {
        in property <int> value: 2;
        Rectangle { background: #e67e22; Text { text: "C2"; color: white; } }
    }

    // Component 3
    export component Component3 {
        in property <int> value: 3;
        Rectangle { background: #f1c40f; Text { text: "C3"; color: black; } }
    }

    // Component 4
    export component Component4 {
        in property <int> value: 4;
        Rectangle { background: #22cc71; Text { text: "C4"; color: white; } }
    }

    // Component 5
    export component Component5 {
        in property <int> value: 5;
        Rectangle { background: #1abc9c; Text { text: "C5"; color: white; } }
    }

    // Component 6
    export component Component6 {
        in property <int> value: 6;
        Rectangle { background: #3498db; Text { text: "C6"; color: white; } }
    }

    // Component 7
    export component Component7 {
        in property <int> value: 7;
        Rectangle { background: #9b59b6; Text { text: "C7"; color: white; } }
    }

    // Component 8
    export component Component8 {
        in property <int> value: 8;
        Rectangle { background: #34495a; Text { text: "C8"; color: white; } }
    }

    // Component 9
    export component Component9 {
        in property <int> value: 9;
        Rectangle { background: #16a085; Text { text: "C9"; color: white; } }
    }

    // Component 10
    export component Component10 {
        in property <int> value: 10;
        Rectangle { background: #27ae60; Text { text: "C10"; color: white; } }
    }

    // Component 11-20 with more complexity
    export component Component11 {
        in property <string> label: "11";
        callback clicked();
        Rectangle {
            background: #2980b9;
            TouchArea { clicked => { root.clicked(); } }
            Text { text: label; color: white; }
        }
    }

    export component Component12 {
        in property <string> label: "12";
        in-out property <bool> active: false;
        Rectangle {
            background: active ? #27ae60 : #c0392b;
            Text { text: label; color: white; }
        }
    }

    export component Component13 {
        in property <[string]> items: ["A", "B", "C"];
        VerticalLayout {
            for item in items: Text { text: item; }
        }
    }

    export component Component14 {
        in property <int> count: 0;
        out property <int> doubled: count * 2;
        Text { text: "Count: " + count + " (x2 = " + doubled + ")"; }
    }

    export component Component15 {
        public function reset() { value = 0; }
        in-out property <int> value: 100;
        Text { text: "Value: " + value; }
    }

    export component Component16 {
        in property <color> bg: #3498db;
        in property <color> fg: white;
        Rectangle {
            background: bg;
            Text { text: "Themed"; color: fg; }
        }
    }

    export component Component17 {
        in property <length> size: 50px;
        Rectangle {
            width: size;
            height: size;
            background: #9b59b6;
        }
    }

    export component Component18 {
        in property <float> progress: 0.5;
        Rectangle {
            background: #ecf0f1;
            Rectangle {
                width: parent.width * progress;
                background: #3498db;
            }
        }
    }

    export component Component19 {
        in property <image> icon;
        Rectangle {
            Image { source: icon; }
        }
    }

    export component Component20 {
        callback value-changed(int);
        in-out property <int> value: 0;
        HorizontalLayout {
            Rectangle {
                background: #dd4c3c;
                TouchArea { clicked => { value -= 1; value-changed(value); } }
                Text { text: "-"; color: white; }
            }
            Text { text: value; }
            Rectangle {
                background: #22cc71;
                TouchArea { clicked => { value += 1; value-changed(value); } }
                Text { text: "+"; color: white; }
            }
        }
    }

    // Main component using all others
    export component ManyComponents inherits Window {
        width: 800px;
        height: 600px;

        GridLayout {
            spacing: 5px;
            padding: 10px;

            Row {
                Component1 {}
                Component2 {}
                Component3 {}
                Component4 {}
                Component5 {}
            }
            Row {
                Component6 {}
                Component7 {}
                Component8 {}
                Component9 {}
                Component10 {}
            }
            Row {
                Component11 {}
                Component12 {}
                Component13 {}
                Component14 { count: 42; }
                Component15 {}
            }
            Row {
                Component16 { bg: #8e44ad; }
                Component17 { size: 80px; }
                Component18 { progress: 0.75; }
                Component19 {}
                Component20 {}
            }
        }
    }
}
