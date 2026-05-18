use leptos::prelude::*;

use view_rs::camera::Camera;

#[component]
fn App() -> impl IntoView {
    let mut camera = Camera::new();

    camera.zoom = 2.0;
    camera.x_offset = 100.0;
    camera.y_offset = 50.0;

    let transform = format!(
        "translate({} {}) scale({})",
        camera.x_offset, camera.y_offset, camera.zoom
    );

    view! {
        <svg width="800" height="600" style="border:1px solid black;">
            <g transform=transform>

                // background
                <rect
                    x="0"
                    y="0"
                    width="1000"
                    height="1000"
                    fill="#f5f5f5"
                    stroke="black"
                />

                // test circles
                <circle cx="100" cy="100" r="20" fill="red" />
                <circle cx="300" cy="200" r="20" fill="blue" />
                <circle cx="500" cy="400" r="20" fill="green" />

            </g>
        </svg>
    }
}

fn main() {
    mount_to_body(App);
}
