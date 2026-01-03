//! Large size example (~100 lines of Slint).
//!
//! Based on the maps example - complex component with multiple features.
//! Compare expansion time against small, medium, and xlarge.
//!
//! See: slint/examples/maps/main.rs

slint::slint! {
    import { Slider } from "std-widgets.slint";

    export struct Tile {
        x: length,
        y: length,
        tile: image,
    }

    export struct MapState {
        zoom-level: int,
        center-x: float,
        center-y: float,
        is-loading: bool,
    }

    export global MapSettings {
        in-out property <int> min-zoom: 1;
        in-out property <int> max-zoom: 19;
        in-out property <length> tile-size: 256px;
    }

    export component LargeComponent inherits Window {
        min-height: 500px;
        min-width: 500px;
        title: "Map Viewer";

        callback flicked(length, length);
        callback zoom-changed(float);
        callback zoom-in(length, length);
        callback zoom-out(length, length);
        callback tile-requested(int, int, int);
        callback link-clicked();

        out property <length> visible-width: flickable.width;
        out property <length> visible-height: flickable.height;

        in-out property <float> zoom <=> zoom-slider.value;
        in property <[Tile]> tiles;
        in property <MapState> state: {
            zoom-level: 1,
            center-x: 0.0,
            center-y: 0.0,
            is-loading: false,
        };

        public function set-viewport(ox: length, oy: length, width: length, height: length) {
            flickable.viewport-x = ox;
            flickable.viewport-y = oy;
            flickable.viewport-width = width;
            flickable.viewport-height = height;
        }

        pure function clamp-zoom(z: float) -> float {
            if z < MapSettings.min-zoom { return MapSettings.min-zoom; }
            if z > MapSettings.max-zoom { return MapSettings.max-zoom; }
            return z;
        }

        VerticalLayout {
            flickable := Flickable {
                for t in tiles: Image {
                    x: t.x;
                    y: t.y;
                    source: t.tile;
                    width: MapSettings.tile-size;
                    height: MapSettings.tile-size;
                }

                flicked => {
                    root.flicked(flickable.viewport-x, flickable.viewport-y);
                }

                TouchArea {
                    scroll-event(e) => {
                        if e.delta-y > 0 {
                            root.zoom-in(self.mouse-x + flickable.viewport-x, self.mouse-y + flickable.viewport-y);
                            return accept;
                        } else if e.delta-y < 0 {
                            root.zoom-out(self.mouse-x + flickable.viewport-x, self.mouse-y + flickable.viewport-y);
                            return accept;
                        }
                        return reject;
                    }
                }
            }

            HorizontalLayout {
                spacing: 10px;
                padding: 5px;

                Text {
                    text: "Zoom:";
                    vertical-alignment: center;
                }

                zoom-slider := Slider {
                    minimum: MapSettings.min-zoom;
                    maximum: MapSettings.max-zoom;
                    horizontal-stretch: 1;

                    released => {
                        root.zoom-changed(self.value);
                    }
                }

                Text {
                    text: round(zoom-slider.value);
                    vertical-alignment: center;
                    min-width: 30px;
                }
            }
        }

        // Loading indicator
        if state.is-loading: Rectangle {
            x: parent.width - 50px;
            y: 10px;
            width: 40px;
            height: 40px;
            background: #3498db80;
            border-radius: 20px;

            Text {
                text: "...";
                color: white;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }

        // Attribution
        Text {
            text: "Map data from OpenStreetMap";
            x: flickable.x + flickable.width - self.width - 3px;
            y: flickable.y + flickable.height - self.height - 3px;
            font-size: 10px;
            color: #666;

            TouchArea {
                clicked => { root.link-clicked(); }
            }
        }
    }
}
