// use leptos::{ev::MouseEvent, prelude::*};
// use view_rs::{camera::Camera, math::Vec2};

// #[component]
// fn App() -> impl IntoView {
//     let (camera, set_camera) = signal(Camera::new());

//     let (is_dragging, set_is_dragging) = signal(false);

//     let (last_mouse, set_last_mouse) = signal(Vec2::new(0.0, 0.0));

//     let transform = move || {
//         let cam = camera.get();
//          format!(
//             "translate({} {}) scale({})",
//             cam.x_offset,
//             cam.y_offset,
//             cam.zoom
//         )
//     };

//     let on_mouse_down = move |ev: MouseEvent| {
//         set_is_dragging.set(true);

//         set_last_mouse.set(Vec2::new(
//             ev.client_x() as f32,
//             ev.client_y() as f32,
//         ));
//     };

//     let on_mouse_up = move |_| {
//         set_is_dragging.set(false);
//     };

//     let on_mouse_move = move |ev: MouseEvent| {
//         if !is_dragging.get() {
//             return;
//         }

//         let current = Vec2::new(
//             ev.client_x() as f32,
//             ev.client_y() as f32,
//         );

//         let last = last_mouse.get();

//         let dx = current.x - last.x;
//         let dy = current.y - last.y;

//         set_camera.update(|cam| {
//             cam.pan(dx, dy);
//         });

//         set_last_mouse.set(current);
//     };

//     view! {

//         <div>
//         <p>
//                 {move || {
//                     let cam = camera.get();

//                     format!(
//                         "zoom: {} | offset: ({}, {})",
//                         cam.zoom,
//                         cam.x_offset,
//                         cam.y_offset
//                     )
//                 }}
//             </p>
//         <svg width="800" height="600" style="border:1px solid black;"
        
//         on:mousedown=on_mouse_down
//                 on:mouseup=on_mouse_up
//                 on:mousemove=on_mouse_move>
//             <g transform=transform>

//                 // background
//                 <rect
//                     x="0"
//                     y="0"
//                     width="1000"
//                     height="1000"
//                     fill="#f5f5f5"
//                     stroke="black"
//                 />

//                 // test circles
//                 <circle cx="100" cy="100" r="20" fill="red" />
//                 <circle cx="300" cy="200" r="20" fill="blue" />
//                 <circle cx="500" cy="400" r="20" fill="green" />

//             </g>
//         </svg>
//         </div>
//     }
// }

// fn main() {
//     mount_to_body(App);
// }


use leptos::ev::{MouseEvent};
use leptos::prelude::*;

use view_rs::{camera::Camera, math::Vec2};

#[component]
fn App() -> impl IntoView {
    let (camera, set_camera) = signal(Camera::new());

    let (is_dragging, set_is_dragging) = signal(false);

    let (last_mouse, set_last_mouse) = signal(Vec2::new(0.0, 0.0));

    let transform = move || {
        let cam = camera.get();

        format!(
            "translate({} {}) scale({})",
            cam.x_offset,
            cam.y_offset,
            cam.zoom
        )
    };

    let on_mouse_down = move |ev: MouseEvent| {
        set_is_dragging.set(true);

        set_last_mouse.set(Vec2::new(
            ev.client_x() as f32,
            ev.client_y() as f32,
        ));
    };

    let on_mouse_up = move |_| {
        set_is_dragging.set(false);
    };

    let on_mouse_move = move |ev: MouseEvent| {
        if !is_dragging.get() {
            return;
        }

        let current = Vec2::new(
            ev.client_x() as f32,
            ev.client_y() as f32,
        );

        let last = last_mouse.get();

        let dx = current.x - last.x;
        let dy = current.y - last.y;

        set_camera.update(|cam| {
            cam.pan(dx, dy);
        });

        set_last_mouse.set(current);
    };

    view! {
        <div>
            <p>
                {move || {
                    let cam = camera.get();

                    format!(
                        "zoom: {} | offset: ({}, {})",
                        cam.zoom,
                        cam.x_offset,
                       cam.y_offset,

                    )
                }}
            </p>

            <svg
                width="800"
                height="600"
                style="border:1px solid black; cursor: grab;"

                on:mousedown=on_mouse_down
                on:mouseup=on_mouse_up
                on:mousemove=on_mouse_move
            >
                <g transform=transform>

                    <rect
                        x="0"
                        y="0"
                        width="1000"
                        height="1000"
                        fill="#f5f5f5"
                        stroke="black"
                    />

                    <circle cx="100" cy="100" r="20" fill="red" />
                    <circle cx="300" cy="200" r="20" fill="blue" />
                    <circle cx="500" cy="400" r="20" fill="green" />

                </g>
            </svg>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}