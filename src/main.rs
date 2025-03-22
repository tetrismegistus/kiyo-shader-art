use kiyo::app::app::{App, AppConfig};
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};
use kiyo::app::audio_orch::{AudioConfig};

fn main() {

    // Here you can configure general settings for the application
    let app_config = AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: false,
        fullscreen: false,
    };

    // The drawconfig specifies how the shaders get executed
    let config = DrawConfig {
        // A list of images to read/write to. The last image in the list is displayed to the screen.
        images: Vec::from([
            ImageConfig {
                // Here you can disable clearing by using ClearConfig::None. This will let you write to images over multiple frames.
                clear: ClearConfig::Color(0.0, 0.0, 0.0)
            },
        ]),
        passes: Vec::from([
            Pass {
                shader: "shaders/colors.comp".to_string(),
                // DispatchConfig::FullScreen ensures that there WILL be 1 shader invocation per pixel.
                // You can however specify a custom dispatch count. A dispatch of (1, 1, 1) invokes 1 workgroup of 32x32 shader invocations.
                dispatches: DispatchConfig::FullScreen,
                // Input and output resources. These get passed as arguments to the shader and lets you configure which image is read/written to. This is WIP.
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
        ])
    };

    // Run kiyo, optionally run programmable audio (WIP)
    App::run(app_config, config, AudioConfig::None);
}
