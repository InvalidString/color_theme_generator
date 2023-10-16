mod stolen_math;
use std::ffi::{CStr, CString};

use stolen_math::*;

use raylib::prelude::*;


const PALETTE_COUNT: u32 = 6;


fn lch_to_color(lightness: f32, chroma: f32, hue: f32) -> Color{
    let [r, g, b] = LchRepresentation::lch_to_nonlinear_srgb(lightness, chroma, hue)
        .map(|x| (x * 255.0) as u8);
    Color::new(r, g, b, 255)
}

fn print_col(color: Color){
    print!("#");
    for c in [color.r, color.b, color.b]{
        print!("{:02x}", c);
    }
    println!();
}

fn main() {


    let window_size = Vector2{x: 600.0, y: 1000.0};


    let (mut rl, t) = init()
        .vsync()
        .msaa_4x()
        .undecorated()
        .size(window_size.x as i32, window_size.y as i32)
        .build();
    rl.gui_load_style(Some(&CString::new("terminal.rgs").unwrap()));


    let mut lightness = 0.5;
    let mut chroma = 0.7;
    let mut hue_base = 0.0;
    let mut hue_off = 2.0*PI as f32* 0.1;
    let mut chroma_off = -0.2;
    let mut lightness_off = 0.2;
    let mut image = Image::gen_image_color(360, 1, Color::BLACK);
    let mut texture: Texture2D = rl.load_texture_from_image(&t, &image).unwrap();


    let hue_circle_shader = rl.load_shader(&t, None, Some("src/hue_circle.fs")).unwrap();


    while !rl.window_should_close() {


        hue_off += rl.get_mouse_wheel_move() * 0.01;

        for x in 0..360 {
            let hue = x as f32;
            let color = lch_to_color(lightness, chroma, hue);
            image.draw_rectangle(x, 0, 1, 1, color);
        }


        let data = image.get_image_data();
        let data = unsafe{
            let ptr: *const u8 = data.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, data.len() * std::mem::size_of::<Color>())
        };
        texture.update_texture(data);


        let mut g = rl.begin_drawing(&t);
        g.clear_background(Color::BLACK);


        let size = window_size.x.min(window_size.y) - 250.0;
        let rect_center = Rectangle{
            x: (window_size.x - size) * 0.5,
            y: (window_size.y - size) * 0.5 - 150.0,
            width: size,
            height: size,
        };

        let center = 
            Vector2::new(
                rect_center.x + 0.5 * rect_center.width, 
                rect_center.y + 0.5 * rect_center.height, 
        );

        let mouse_pos = g.get_mouse_position();
        if g.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON){
            if rect_center.check_collision_point_rec(mouse_pos){
                let rel_mpos = mouse_pos - center;
                hue_base = f32::atan2(rel_mpos.y, rel_mpos.x);
            }
        }
        let src_rect = Rectangle{x: 0.0, y: 0.0, width: texture.width as f32, height: texture.height as f32};
        {
            let mut g = g.begin_shader_mode(&hue_circle_shader);
            g.draw_texture_pro(&texture, src_rect, rect_center, Vector2{x:0.0,y:0.0}, 0.0, Color::WHITE);
        }



        // selected hue
        {
            let mut draw_hue_marker = |angle: f32|{
                let hue_dir = Vector2::new(f32::cos(angle), f32::sin(angle));
                g.draw_line_ex(center + hue_dir * rect_center.width * 0.5, 
                               center + hue_dir * rect_center.width * 0.52,
                               3.0, Color::WHITE);
            };


            for i in 0..PALETTE_COUNT{
                draw_hue_marker(hue_base + i as f32 * hue_off);
            }
        }

        // sliders
        {
            let padding = 50.0;
            let width = window_size.x - padding;
            let left = padding*0.5;

            let height = 20.0;


            lightness = g.gui_slider(
                Rectangle {
                    x: left,
                    y: padding,
                    width,
                    height,
                },
                None,
                None,
                lightness,
                0.0,
                1.5);
            chroma = g.gui_slider(
                Rectangle {
                    x: left,
                    y: padding * 2.0 + height,
                    width,
                    height,
                },
                None,
                None,
                chroma,
                0.0,
                1.5);


            lightness_off = g.gui_slider(
                Rectangle {
                    x: left,
                    y: 750.0,
                    width,
                    height,
                },
                None,
                None,
                lightness_off,
                -0.5,
                0.5);

            chroma_off = g.gui_slider(
                Rectangle {
                    x: left,
                    y: 800.0,
                    width,
                    height,
                },
                None,
                None,
                chroma_off,
                -0.5,
                0.5);


        }


        let should_print = g.is_key_pressed(KeyboardKey::KEY_P);
        if should_print{
            println!("Palette: ");
        }

        // palette
        {
            for i in 0..PALETTE_COUNT{
                let hue = RAD2DEG as f32 * (hue_base + hue_off * i as f32);


                let color = lch_to_color(lightness, chroma, hue);
                if should_print{
                    print_col(color);
                }

                let rec = Rectangle{
                    x: 50.0 + i as f32 * 60.0,
                    y: 600.0,
                    width: 50.0,
                    height: 50.0,
                };
                g.draw_rectangle_rec(rec, color);

                let rec2 = Rectangle{
                    y: rec.y + 60.0,
                    ..rec
                };

                let color2 = lch_to_color(lightness + lightness_off, chroma + chroma_off, hue);
                if should_print{
                    print_col(color2);
                }
                g.draw_rectangle_rec(rec2, color2);
            }
        }

    }
}
