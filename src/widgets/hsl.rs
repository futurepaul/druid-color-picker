use druid::widget::Widget;
use druid::{
    BoxConstraints, Geometry, HandlerCtx, Id, LayoutCtx, LayoutResult, MouseEvent, PaintCtx, Ui,
};

use kurbo::Rect;
use piet::{ImageFormat, InterpolationMode, RenderContext};

use std::any::Any;

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0. {
        t += 1.
    }
    if t > 1. {
        t -= 1.
    };
    if t < 1. / 6. {
        return p + (q - p) * 6. * t;
    }
    if t < 1. / 2. {
        return q;
    }
    if t < 2. / 3. {
        return p + (q - p) * (2. / 3. - t) * 6.;
    }
    return p;
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8, u8) {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l; // achromatic
    } else {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };

        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 1. / 3.);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1. / 3.);
    }

    return (
        (r * 255.).round() as u8,
        (g * 255.).round() as u8,
        (b * 255.).round() as u8,
        255,
    );
}

pub enum Converter {
    HslH(f64),
    HslS(f64),
    HslL(f64),
    HslSL(f64, f64),
}

fn xy_to_rgb(width: usize, height: usize, converter: Converter) -> Vec<u8> {
    let mut image_data = vec![255; width * width * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            let x_ratio = x as f64 / width as f64;
            let y_ratio = y as f64 / width as f64;

            // Where the magic happens
            let color = match converter {
                Converter::HslH(hue) => hsl_to_rgb(hue, x_ratio, y_ratio),
                Converter::HslS(saturation) => hsl_to_rgb(x_ratio, saturation, y_ratio),
                Converter::HslL(luminosity) => hsl_to_rgb(x_ratio, y_ratio, luminosity),
                Converter::HslSL(saturation, luminosity) => {
                    hsl_to_rgb(x_ratio, saturation, luminosity)
                }
            };
            // let color = (converter)(x_ratio, y_ratio);

            image_data[ix + 0] = color.0;
            image_data[ix + 1] = color.1;
            image_data[ix + 2] = color.2;
            image_data[ix + 3] = color.3;
        }
    }

    image_data
}

const BOX_HEIGHT: f64 = 256.;

pub struct HSL {
    hue: f64,
    saturation: f64,
    luminosity: f64,
    cursor: (f64, f64),
}

pub enum WhichHSL {
    Hue(f64),
    Saturation(f64),
    Luminosity(f64),
}

impl HSL {
    pub fn new() -> HSL {
        HSL {
            hue: 0.5,
            saturation: 0.5,
            luminosity: 0.5,
            cursor: (7., 5.),
        }
    }
    pub fn ui(self, ctx: &mut Ui) -> Id {
        ctx.add(self, &[])
    }
}

impl Widget for HSL {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, geom: &Geometry) {
        let (x, y) = geom.pos;
        let (x, y) = (x as f64, y as f64);

        // Draw color box
        let image_data = xy_to_rgb(
            BOX_HEIGHT as usize,
            BOX_HEIGHT as usize,
            Converter::HslH(self.hue),
        );

        let image = paint_ctx
            .render_ctx
            .make_image(
                BOX_HEIGHT as usize,
                BOX_HEIGHT as usize,
                &image_data,
                ImageFormat::RgbaSeparate,
            )
            .unwrap();

        paint_ctx.render_ctx.draw_image(
            &image,
            ((x, y), (x + BOX_HEIGHT, y + BOX_HEIGHT)),
            InterpolationMode::NearestNeighbor,
        );

        // Draw cursor
        let white = 0xff_ff_ff_ff;
        let black = 0x00_00_00_ff;

        let white_brush = paint_ctx.render_ctx.solid_brush(white).unwrap();
        let black_brush = paint_ctx.render_ctx.solid_brush(black).unwrap();

        let (x, y) = geom.pos;
        let (x, y) = (x as f64, y as f64);

        // TODO: constrain the cursor!
        let (_width, _height) = geom.size;

        let (cursor_x, cursor_y) = (x + self.cursor.0, y + self.cursor.1);
        let outer_rect = Rect::new(cursor_x, cursor_y, cursor_x + 6., cursor_y + 6.);

        let inner_rect = Rect::new(cursor_x + 1., cursor_y + 1., cursor_x + 5., cursor_y + 5.);

        paint_ctx
            .render_ctx
            .stroke(outer_rect, &black_brush, 1., None);

        paint_ctx
            .render_ctx
            .stroke(inner_rect, &white_brush, 1., None);
    }

    fn mouse(&mut self, event: &MouseEvent, ctx: &mut HandlerCtx) -> bool {
        dbg!(event);

        if event.count == 1 {

            ctx.set_active(true);

            //Feels better to move this a bit up and to the left of the click
            self.cursor = (event.x as f64 - 5., event.y as f64 - 5.);

            let x_ratio = event.x as f64 / BOX_HEIGHT as f64;
            let y_ratio = event.y as f64 / BOX_HEIGHT as f64;

            self.saturation = x_ratio;
            self.luminosity = y_ratio;
            ctx.send_event((self.saturation, self.luminosity));
        } else {
            ctx.set_active(false);
        }
        ctx.invalidate();
        // dbg!(hsl_to_rgb(self.hue, self.saturation, self.luminosity));
        true
    }

    fn mouse_moved(&mut self, x: f32, y: f32, ctx: &mut HandlerCtx) {
        if ctx.is_active() {
            //Feels better to move this a bit up and to the left of the click
            self.cursor = (x as f64 - 5., y as f64 - 5.);

            let x_ratio = x as f64 / BOX_HEIGHT as f64;
            let y_ratio = y as f64 / BOX_HEIGHT as f64;

            self.saturation = x_ratio;
            self.luminosity = y_ratio;
            ctx.send_event((self.saturation, self.luminosity));
            ctx.invalidate();
        }
    }

    fn layout(
        &mut self,
        bc: &BoxConstraints,
        _children: &[Id],
        size: Option<(f32, f32)>,
        _ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        match size {
            Some(size) => LayoutResult::Size(bc.constrain(size)),
            None => LayoutResult::Size((BOX_HEIGHT as f32, BOX_HEIGHT as f32))
        }
    }

    fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerCtx) -> bool {
        if let Some(value) = payload.downcast_ref::<WhichHSL>() {
            match value {
                WhichHSL::Hue(val) => self.hue = *val,
                WhichHSL::Saturation(val) => self.saturation = *val,
                WhichHSL::Luminosity(val) => self.luminosity = *val,
            }
            self.cursor = (self.saturation * BOX_HEIGHT, self.luminosity * BOX_HEIGHT);
            ctx.invalidate();
            dbg!(hsl_to_rgb(self.hue, self.saturation, self.luminosity));
            true
        } else {
            println!("downcast failed");
            false
        }
    }
}
