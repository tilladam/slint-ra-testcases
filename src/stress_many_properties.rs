//! Many properties stress test.
//!
//! Tests macro expansion with many properties on a single component.
//! This stresses property handling and binding resolution.

slint::slint! {
    export component ManyProperties inherits Window {
        width: 800px;
        height: 600px;

        // 50+ input properties
        in property <int> prop-int-1: 1;
        in property <int> prop-int-2: 2;
        in property <int> prop-int-3: 3;
        in property <int> prop-int-4: 4;
        in property <int> prop-int-5: 5;
        in property <int> prop-int-6: 6;
        in property <int> prop-int-7: 7;
        in property <int> prop-int-8: 8;
        in property <int> prop-int-9: 9;
        in property <int> prop-int-10: 10;

        in property <float> prop-float-1: 1.0;
        in property <float> prop-float-2: 2.0;
        in property <float> prop-float-3: 3.0;
        in property <float> prop-float-4: 4.0;
        in property <float> prop-float-5: 5.0;
        in property <float> prop-float-6: 6.0;
        in property <float> prop-float-7: 7.0;
        in property <float> prop-float-8: 8.0;
        in property <float> prop-float-9: 9.0;
        in property <float> prop-float-10: 10.0;

        in property <string> prop-str-1: "one";
        in property <string> prop-str-2: "two";
        in property <string> prop-str-3: "three";
        in property <string> prop-str-4: "four";
        in property <string> prop-str-5: "five";
        in property <string> prop-str-6: "six";
        in property <string> prop-str-7: "seven";
        in property <string> prop-str-8: "eight";
        in property <string> prop-str-9: "nine";
        in property <string> prop-str-10: "ten";

        in property <bool> prop-bool-1: true;
        in property <bool> prop-bool-2: false;
        in property <bool> prop-bool-3: true;
        in property <bool> prop-bool-4: false;
        in property <bool> prop-bool-5: true;
        in property <bool> prop-bool-6: false;
        in property <bool> prop-bool-7: true;
        in property <bool> prop-bool-8: false;
        in property <bool> prop-bool-9: true;
        in property <bool> prop-bool-10: false;

        in property <color> prop-color-1: #dd4c3c;
        in property <color> prop-color-2: #e67e22;
        in property <color> prop-color-3: #f1c40f;
        in property <color> prop-color-4: #22cc71;
        in property <color> prop-color-5: #1abc9c;
        in property <color> prop-color-6: #3498db;
        in property <color> prop-color-7: #9b59b6;
        in property <color> prop-color-8: #34495a;
        in property <color> prop-color-9: #95a5a6;
        in property <color> prop-color-10: #7f8c8d;

        in property <length> prop-len-1: 10px;
        in property <length> prop-len-2: 20px;
        in property <length> prop-len-3: 30px;
        in property <length> prop-len-4: 40px;
        in property <length> prop-len-5: 50px;
        in property <length> prop-len-6: 60px;
        in property <length> prop-len-7: 70px;
        in property <length> prop-len-8: 80px;
        in property <length> prop-len-9: 90px;
        in property <length> prop-len-10: 100px;

        // Computed output properties
        out property <int> sum-int: prop-int-1 + prop-int-2 + prop-int-3 + prop-int-4 + prop-int-5;
        out property <float> sum-float: prop-float-1 + prop-float-2 + prop-float-3 + prop-float-4 + prop-float-5;
        out property <string> concat-str: prop-str-1 + " " + prop-str-2 + " " + prop-str-3;
        out property <bool> all-true: prop-bool-1 && prop-bool-3 && prop-bool-5 && prop-bool-7 && prop-bool-9;
        out property <length> sum-len: prop-len-1 + prop-len-2 + prop-len-3 + prop-len-4 + prop-len-5;

        // In-out properties
        in-out property <int> editable-int: 0;
        in-out property <string> editable-str: "";
        in-out property <bool> editable-bool: false;
        in-out property <color> editable-color: white;
        in-out property <length> editable-len: 0px;

        // Private properties with complex expressions
        private property <int> computed-1: (prop-int-1 * prop-int-2) + prop-int-3;
        private property <int> computed-2: (prop-int-4 * prop-int-5) - prop-int-6;
        private property <int> computed-3: prop-int-7 + prop-int-8 + prop-int-9 + prop-int-10;
        private property <float> computed-4: (prop-float-1 + prop-float-2) / 2.0;
        private property <float> computed-5: prop-float-3 * prop-float-4 * prop-float-5;

        private property <color> active-color: prop-bool-1 ? prop-color-1 : prop-color-2;
        private property <length> dynamic-size: prop-bool-3 ? prop-len-1 : prop-len-2;

        // Many callbacks
        callback on-prop-1-changed(int);
        callback on-prop-2-changed(int);
        callback on-prop-3-changed(int);
        callback on-prop-4-changed(int);
        callback on-prop-5-changed(int);
        callback on-str-changed(string);
        callback on-bool-changed(bool);
        callback on-color-changed(color);
        callback on-submit();
        callback on-cancel();

        Rectangle {
            background: active-color;

            Text {
                text: "Properties: " + sum-int + " | " + concat-str;
                color: white;
            }
        }
    }
}
