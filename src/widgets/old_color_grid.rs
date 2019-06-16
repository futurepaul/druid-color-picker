// use druid::widget::Widget;
// use druid::{BoxConstraints, Geometry, Id, LayoutCtx, HandlerCtx, LayoutResult, PaintCtx, Ui};

// use kurbo::Rect;
// use piet::{FillRule, ImageFormat, InterpolationMode, RenderContext};

// use std::any::Any;

// const BOX_HEIGHT: f64 = 320.;

// /// Takes a closure that converts 2D coords to a RGBA color
// /// X and Y are on a scale of 0.0 to 1.0
// pub struct Grid<F>
// where
//     F: Fn(f64, f64) -> (u8, u8, u8, u8),
// {
//     converter: F,
// }

// // fn split_rgba(rgba: u32) -> (u8, u8, u8, u8) {
// //     (
// //         (rgba >> 24) as u8,
// //         ((rgba >> 16) & 255) as u8,
// //         ((rgba >> 8) & 255) as u8,
// //         (rgba & 255) as u8,
// //     )
// // }

// impl<F: 'static> Grid<F>
// where
//     F: Fn(f64, f64) -> (u8, u8, u8, u8),
// {
//     pub fn new(converter: F) -> Grid<F> {
//         Grid { converter }
//     }
//     pub fn ui(self, ctx: &mut Ui) -> Id {
//         ctx.add(self, &[])
//     }
// }

// impl<F: 'static> Widget for Grid<F>
// where
//     F: Fn(f64, f64) -> (u8, u8, u8, u8),
// {
//     fn paint(&mut self, paint_ctx: &mut PaintCtx, geom: &Geometry) {
//         let (x, y) = geom.pos;
//         let (x, y) = (x as f64, y as f64);

//         let size = BOX_HEIGHT as usize;

//         let mut image_data = vec![255; size * size * 4];
//         dbg!("redrawing");
//         for y in 0..size {
//             for x in 0..size {
//                 let ix = (y * size + x) * 4;
//                 let x_ratio = x as f64 / size as f64;
//                 let y_ratio = y as f64 / size as f64;
//                 // dbg!(x_ratio, y_ratio);
//                 let color = (self.converter)(x_ratio, y_ratio);
//                 image_data[ix + 0] = color.0;
//                 image_data[ix + 1] = color.1;
//                 image_data[ix + 2] = color.2;
//                 image_data[ix + 3] = color.3;
//             }
//         }

//         let image = paint_ctx
//             .render_ctx
//             .make_image(size, size, &image_data, ImageFormat::RgbaSeparate)
//             .unwrap();

//         paint_ctx.render_ctx.draw_image(
//             &image,
//             (
//                 (x, y),
//                 (x + BOX_HEIGHT, y + BOX_HEIGHT),
//             ),
//             InterpolationMode::NearestNeighbor,
//         );
//     }

//     fn layout(
//         &mut self,
//         bc: &BoxConstraints,
//         _children: &[Id],
//         _size: Option<(f32, f32)>,
//         _ctx: &mut LayoutCtx,
//     ) -> LayoutResult {
//         LayoutResult::Size(bc.constrain((BOX_HEIGHT as f32, BOX_HEIGHT as f32)))
//     }

//     // fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerCtx) -> bool {
//     //     if let Some(value) = payload.downcast_ref::<F>() {
//     //         self.converter = *value;
//     //         ctx.invalidate();
//     //         true
//     //     } else {
//     //         println!("downcast failed");
//     //         false
//     //     }
//     // }
// }
