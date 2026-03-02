use cef::{rc::*, *};
use flume::Sender;

use crate::chromium::ChromiumEvent;

wrap_display_handler! {
    pub struct ChromiumDisplayHandler {
        sender: Sender<ChromiumEvent>,
    }

    impl DisplayHandler {
        fn on_fullscreen_mode_change(
            &self,
            _browser: Option<&mut Browser>,
            fullscreen: i32,
        ) {
            let state = fullscreen == 1;

            self.sender.send(ChromiumEvent::Fullscreen(state)).ok();
        }

        fn on_cursor_change(
            &self,
            _browser: Option<&mut Browser>,
            _cursor: ::std::os::raw::c_ulong,
            type_: CursorType,
            _custom_cursor_info: Option<&CursorInfo>,
        ) -> ::std::os::raw::c_int {
            let name = match type_ {
                CursorType::POINTER => "default",
                CursorType::CROSS => "crosshair",
                CursorType::HAND => "pointer",
                CursorType::IBEAM => "text",
                CursorType::WAIT => "wait",
                CursorType::HELP => "help",
                CursorType::EASTRESIZE => "e-resize",
                CursorType::NORTHRESIZE => "n-resize",
                CursorType::NORTHEASTRESIZE => "ne-resize",
                CursorType::NORTHWESTRESIZE => "nw-resize",
                CursorType::SOUTHRESIZE => "s-resize",
                CursorType::SOUTHEASTRESIZE => "se-resize",
                CursorType::SOUTHWESTRESIZE => "sw-resize",
                CursorType::WESTRESIZE => "w-resize",
                CursorType::NORTHSOUTHRESIZE => "ns-resize",
                CursorType::EASTWESTRESIZE => "ew-resize",
                CursorType::NORTHEASTSOUTHWESTRESIZE => "nesw-resize",
                CursorType::NORTHWESTSOUTHEASTRESIZE => "nwse-resize",
                CursorType::COLUMNRESIZE => "col-resize",
                CursorType::ROWRESIZE => "row-resize",
                CursorType::MOVE => "move",
                CursorType::VERTICALTEXT => "vertical-text",
                CursorType::CELL => "cell",
                CursorType::CONTEXTMENU => "context-menu",
                CursorType::ALIAS => "alias",
                CursorType::PROGRESS => "progress",
                CursorType::NODROP => "no-drop",
                CursorType::COPY => "copy",
                CursorType::NONE => "none",
                CursorType::NOTALLOWED => "not-allowed",
                CursorType::ZOOMIN => "zoom-in",
                CursorType::ZOOMOUT => "zoom-out",
                CursorType::GRAB => "grab",
                CursorType::GRABBING => "grabbing",
                _ => "default",
            };

            self.sender
                .send(ChromiumEvent::CursorChange(name.to_string()))
                .ok();

            true as _
        }
    }
}
