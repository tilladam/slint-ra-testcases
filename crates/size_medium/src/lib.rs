//! Medium size example (~50 lines of Slint).
//!
//! Represents a typical component complexity.
//! Compare expansion time against small, large, and xlarge.

slint::slint! {
    export struct TodoItem {
        text: string,
        completed: bool,
    }

    export component MediumComponent inherits Window {
        width: 400px;
        height: 300px;
        title: "Todo List";

        in property <[TodoItem]> items: [
            { text: "First task", completed: false },
            { text: "Second task", completed: true },
            { text: "Third task", completed: false },
        ];

        in-out property <string> new-item-text: "";
        in-out property <int> selected-index: -1;

        callback add-item(string);
        callback toggle-item(int);
        callback delete-item(int);

        VerticalLayout {
            padding: 10px;
            spacing: 5px;

            HorizontalLayout {
                spacing: 5px;

                Rectangle {
                    horizontal-stretch: 1;
                    background: #ecf0f1;
                    border-radius: 4px;

                    Text {
                        text: new-item-text == "" ? "Add new item..." : new-item-text;
                        color: new-item-text == "" ? #95a5a6 : #2c3e50;
                        vertical-alignment: center;
                    }
                }

                Rectangle {
                    width: 60px;
                    background: #3498db;
                    border-radius: 4px;

                    Text {
                        text: "Add";
                        color: white;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }

                    TouchArea {
                        clicked => { root.add-item(new-item-text); }
                    }
                }
            }

            for item[index] in items: Rectangle {
                height: 40px;
                background: index == selected-index ? #d5dbdb : (item.completed ? #d5f5e3 : #fdebd0);
                border-radius: 4px;

                HorizontalLayout {
                    padding: 8px;
                    spacing: 10px;

                    Text {
                        text: item.completed ? "[x]" : "[ ]";
                        color: #2c3e50;

                        TouchArea {
                            clicked => { root.toggle-item(index); }
                        }
                    }

                    Text {
                        text: item.text;
                        color: item.completed ? #7f8c8d : #2c3e50;
                        horizontal-stretch: 1;
                    }

                    Text {
                        text: "X";
                        color: #dd4c3c;

                        TouchArea {
                            clicked => { root.delete-item(index); }
                        }
                    }
                }
            }
        }
    }
}
