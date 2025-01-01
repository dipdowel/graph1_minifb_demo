use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::adapters::{rgba_to_0rgb, rgba_to_0rgb_unsafe};
use graph1::utils::color::palettes::RetroNeon;
use graph1_wasm_demo::demo::user_data::DemoUserData;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 480;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 240;

const NUM_THREADS: usize = 6;

fn main() {
    println!("Hello, world!");

    let mut width = WIN_WIDTH as usize;
    let mut height = WIN_HEIGHT as usize;

    // The window context
    let mut win_ctx = WindowContext::new(
        WIN_WIDTH,
        WIN_HEIGHT,
        Some(RetroNeon::CYBER_BLUE),
        Some(RetroNeon::LASER_LIME),
        // We can avoid using the color adapter for the output buffer if we adapt the input colors
        // That makes alpha blending impossible though.
        // Some(rgba_color_to_0rgb(RetroNeon::CYBER_BLUE)),
        // Some(rgba_color_to_0rgb(RetroNeon::LASER_LIME)),
    );

    let mut output_buf_0rgb: Vec<u32> = vec![win_ctx.background_color; win_ctx.get_num_pixels()];

    // Graph context
    let mut ctx: GraphContext<DemoUserData> =
        GraphContext::new(win_ctx, true, false, None, NUM_THREADS);
    // let mut ctx:GraphContext<DemoUserData> = GraphContext::new(win_ctx,  true,false, None, 1);

    ctx.user_data.bouncy.dx = 2;
    ctx.user_data.bouncy.dy = 2;
    ctx.alpha.method = graph1::core::context::alpha::AlphaMethod::Float;
    ctx.alpha.enabled = true;

    // Draw a rectangle of size 40x20 at the top-left corner of the window
    draw::rectangle::filled(&mut ctx, &RectArea::new(0, 0, 40, 20, None));

    let mut window_options: WindowOptions = WindowOptions::default();
    window_options.resize = true;
    window_options.scale_mode = minifb::ScaleMode::Center;

    let mut window = Window::new(
        "Graph1 - Minifb demo",
        ctx.win.w_usize,
        ctx.win.h_usize,
        window_options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    // MAIN LOOP
    // **************
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // println!(">>> frame_count: {} ", ani_ctx.frame_count);

        // =====================================================================================
        // Handle window resizing
        // =====================================================================================

        // Check if the window size has changed
        let (new_width, new_height) = window.get_size();

        if new_width != width || new_height != height {
            // Update dimensions and buffer
            width = new_width;
            height = new_height;
            ctx.resize(width as u32, height as u32);
            output_buf_0rgb.resize(ctx.win.get_num_pixels(), 0);

            println!("Window resized to: {}x{}", width, height);
        }

        // ===[ DEMO SELECTION ]===========================
        graph1_wasm_demo::demo::d_003_bouncy::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_001_basic_concepts_pt1::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_005_luminance_vs_intensity::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::desaturate::luminance_vs_intensity::render_frame(&mut ctx);

        // ===[ COLOR ADAPTER ]===========================
        // let start = Instant::now();
        let stats = rgba_to_0rgb(
            &mut output_buf_0rgb,
            &mut ctx.frame_buf,
            ctx.num_threads,
            false,
        );
        // let duration = start.elapsed();
        // println!(
        //     "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
        //     ctx.num_threads,
        //     duration.as_millis(),
        //     output_buf_0rgb.len()
        // );

        // println!("[ multithreaded ] stats: {:?}", stats);

        // rgba_to_0rgb_unsafe(&mut output_buf_0rgb, &mut ctx.frame_buf,false);

        /* REDRAW THE MAIN WINDOW
         ********************************************************************************************/
        window
            .update_with_buffer(&output_buf_0rgb, ctx.win.w_usize, ctx.win.h_usize)
            // Use the frame_buf directly for the output without the color adapter
            // .update_with_buffer(&ctx.frame_buf, ctx.win.w_usize, ctx.win.h_usize)
            .unwrap();

        ctx.frame_count += 1;
    } // main while loop
}
