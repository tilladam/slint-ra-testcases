//! Deep nesting stress test.
//!
//! Tests macro expansion with deeply nested element hierarchies.
//! This stresses the parser and code generator with recursive structures.

slint::slint! {
    export component DeepNesting inherits Window {
        width: 800px;
        height: 600px;

        // Level 1
        Rectangle {
            background: #f8f9fa;

            // Level 2
            Rectangle {
                x: 10px; y: 10px;
                width: parent.width - 20px;
                height: parent.height - 20px;
                background: #e9ecef;

                // Level 3
                Rectangle {
                    x: 10px; y: 10px;
                    width: parent.width - 20px;
                    height: parent.height - 20px;
                    background: #dee2e6;

                    // Level 4
                    Rectangle {
                        x: 10px; y: 10px;
                        width: parent.width - 20px;
                        height: parent.height - 20px;
                        background: #ced4da;

                        // Level 5
                        Rectangle {
                            x: 10px; y: 10px;
                            width: parent.width - 20px;
                            height: parent.height - 20px;
                            background: #adb5bd;

                            // Level 6
                            Rectangle {
                                x: 10px; y: 10px;
                                width: parent.width - 20px;
                                height: parent.height - 20px;
                                background: #6c757d;

                                // Level 7
                                Rectangle {
                                    x: 10px; y: 10px;
                                    width: parent.width - 20px;
                                    height: parent.height - 20px;
                                    background: #495057;

                                    // Level 8
                                    Rectangle {
                                        x: 10px; y: 10px;
                                        width: parent.width - 20px;
                                        height: parent.height - 20px;
                                        background: #343a40;

                                        // Level 9
                                        Rectangle {
                                            x: 10px; y: 10px;
                                            width: parent.width - 20px;
                                            height: parent.height - 20px;
                                            background: #212529;

                                            // Level 10
                                            Rectangle {
                                                x: 10px; y: 10px;
                                                width: parent.width - 20px;
                                                height: parent.height - 20px;
                                                background: #3498db;

                                                // Level 11
                                                VerticalLayout {
                                                    padding: 10px;

                                                    // Level 12
                                                    HorizontalLayout {
                                                        spacing: 5px;

                                                        // Level 13
                                                        Rectangle {
                                                            background: #dd4c3c;

                                                            // Level 14
                                                            Text {
                                                                text: "Deep!";
                                                                color: white;
                                                                horizontal-alignment: center;
                                                                vertical-alignment: center;
                                                            }
                                                        }

                                                        // Level 13
                                                        Rectangle {
                                                            background: #22cc71;

                                                            // Level 14
                                                            VerticalLayout {
                                                                // Level 15
                                                                Text {
                                                                    text: "Very";
                                                                    color: white;
                                                                }
                                                                // Level 15
                                                                Text {
                                                                    text: "Deep!";
                                                                    color: white;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Another component with nested layouts
    export component NestedLayouts {
        VerticalLayout {
            HorizontalLayout {
                VerticalLayout {
                    HorizontalLayout {
                        VerticalLayout {
                            HorizontalLayout {
                                VerticalLayout {
                                    HorizontalLayout {
                                        Rectangle { background: red; }
                                        Rectangle { background: green; }
                                    }
                                    HorizontalLayout {
                                        Rectangle { background: blue; }
                                        Rectangle { background: yellow; }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Component with nested conditionals
    export component NestedConditionals {
        in property <int> level: 5;

        if level > 0: Rectangle {
            background: #dd4c3c;
            if level > 1: Rectangle {
                x: 10px; y: 10px;
                background: #e67e22;
                if level > 2: Rectangle {
                    x: 10px; y: 10px;
                    background: #f1c40f;
                    if level > 3: Rectangle {
                        x: 10px; y: 10px;
                        background: #22cc71;
                        if level > 4: Rectangle {
                            x: 10px; y: 10px;
                            background: #3498db;
                            Text {
                                text: "Level " + level;
                                color: white;
                            }
                        }
                    }
                }
            }
        }
    }
}
