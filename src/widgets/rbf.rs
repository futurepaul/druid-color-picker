/// Derived from https://github.com/linebender/rbf-interp/blob/master/examples/mutator.rs
use druid::widget::Widget;
use druid::{BoxConstraints, Geometry, HandlerCtx, Id, LayoutCtx, LayoutResult, PaintCtx, Ui};

use piet::{ImageFormat, InterpolationMode, RenderContext};

use std::any::Any;

use nalgebra::DVector;
use rbf_interp::{Basis, Scatter};

const SAMPLES: &[([usize; 2], [u8; 3])] = &[
    ([11, 0], [128, 128, 128]),
    ([14, 2], [0, 0, 127]),
    ([6, 6], [51, 51, 51]),
    ([9, 6], [0, 0, 127]),
    ([12, 6], [129, 63, 0]),
    ([6, 8], [51, 51, 51]),
    ([14, 16], [129, 63, 0]),
];

fn scatter() -> Scatter {
    let locs = SAMPLES
        .iter()
        .map(|(loc, _color)| DVector::from_vec(vec![loc[0] as f64 + 0.5, loc[1] as f64 + 0.5]))
        .collect::<Vec<_>>();
    let colors = SAMPLES
        .iter()
        .map(|(_loc, color)| {
            DVector::from_vec(vec![color[0] as f64, color[1] as f64, color[2] as f64])
        })
        .collect::<Vec<_>>();

    Scatter::create(locs, colors, Basis::PolyHarmonic(2), 2)
}

fn rbf(x: f64, y: f64, u: f64, v: f64, scatter: &Scatter) -> (u8, u8, u8, u8) {
    //Not exactly sure what the constants in here do
    let u = ((x as f64) * (48. * u)).floor();
    let v = ((y as f64) * (48. * v)).floor();

    let interp = scatter.eval(DVector::from_vec(vec![u, v]));
    let r = interp[0].round().max(0.0).min(255.0) as u8;
    let g = interp[1].round().max(0.0).min(255.0) as u8;
    let b = interp[2].round().max(0.0).min(255.0) as u8;

    (r, g, b, 255)
}

fn xy_to_rbf(width: usize, height: usize, u: f64, v: f64) -> Vec<u8> {
    let mut image_data = vec![255; width * width * 4];

    // Do this once because it's expensive
    let scatter = scatter();
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            let x_ratio = x as f64 / width as f64;
            let y_ratio = y as f64 / width as f64;

            // Where the magic happens
            let color = rbf(x_ratio, y_ratio, u, v, &scatter);

            image_data[ix + 0] = color.0;
            image_data[ix + 1] = color.1;
            image_data[ix + 2] = color.2;
            image_data[ix + 3] = color.3;
        }
    }

    image_data
}

const BOX_HEIGHT: f64 = 320.;

pub struct RBF {
    u: f64,
    v: f64,
}

impl RBF {
    pub fn new() -> RBF {
        RBF { u: 1.0, v: 1.0 }
    }
    pub fn ui(self, ctx: &mut Ui) -> Id {
        ctx.add(self, &[])
    }
}

impl Widget for RBF {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, geom: &Geometry) {
        let (x, y) = geom.pos;
        let (x, y) = (x as f64, y as f64);

        let image_data = xy_to_rbf(BOX_HEIGHT as usize, BOX_HEIGHT as usize, self.u, self.v);

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
    }

    fn layout(
        &mut self,
        bc: &BoxConstraints,
        _children: &[Id],
        _size: Option<(f32, f32)>,
        _ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        LayoutResult::Size(bc.constrain((BOX_HEIGHT as f32, BOX_HEIGHT as f32)))
    }

    fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerCtx) -> bool {
        if let Some(value) = payload.downcast_ref::<(f64, f64)>() {
            self.u = value.0;
            self.v = value.1;
            ctx.invalidate();
            true
        } else {
            println!("downcast failed");
            false
        }
    }
}
