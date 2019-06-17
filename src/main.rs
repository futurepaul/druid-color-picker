use druid_shell::platform::WindowBuilder;
use druid_shell::win_main;

use druid::widget::{Button, Column, EventForwarder, Padding};
use druid::{UiMain, UiState};

use druid::Id;

pub mod widgets;
use widgets::{Slider, WhichHSL, HSL, Slider2D};

#[derive(Clone, Copy, Debug)]
struct AppState {
    h: f64,
    s: f64,
    l: f64,
}

fn pad(widget: Id, state: &mut UiState) -> Id {
    Padding::uniform(5.0).ui(widget, state)
}

#[derive(Debug, Clone)]
pub enum Action {
    SetH(f64),
    SetS(f64),
    SetL(f64),
    SetSL(f64, f64)
}

fn main() {
    druid_shell::init();

    let mut run_loop = win_main::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let mut state = UiState::new();

    let mut app = AppState {
        h: 0.5,
        s: 0.5,
        l: 0.5,
    };

    let color_grid = HSL::new().ui(&mut state);
    let color_grid_padded = pad(color_grid, &mut state);

    // let slider_hue = Slider::new(app.h).ui(&mut state);
    // let slider_hue = CustomSlider::new(app.h).ui(
    //     &[
    //         HSL::new().ui(&mut state),
    //         Button::new("Slide").ui(&mut state),
    //     ],
    //     &mut state,
    // );
    // let slider_hue = Slider2D::new(0.1, 0.5).ui(&mut state);
    // let slider_hue_padded = pad(slider_hue, &mut state);

    let slider_hue = Slider::new(app.h).ui(&mut state);
    let slider_hue_padded = pad(slider_hue, &mut state);

    let slider_saturation = Slider::new(app.s).ui(&mut state);
    let slider_sat_padded = pad(slider_saturation, &mut state);

    let slider_luminosity = Slider::new(app.l).ui(&mut state);
    let slider_lum_padded = pad(slider_luminosity, &mut state);

    let column = Column::new();
    let panel = column.ui(
        &[
            color_grid_padded,
            slider_hue_padded,
            slider_sat_padded,
            slider_lum_padded,
        ],
        &mut state,
    );

    state.add_listener(slider_hue, move |value: &mut f64, mut ctx| {
        ctx.poke_up(&mut Action::SetH(*value));
    });

    state.add_listener(slider_saturation, move |value: &mut f64, mut ctx| {
        ctx.poke_up(&mut Action::SetS(*value));
    });

    state.add_listener(slider_luminosity, move |value: &mut f64, mut ctx| {
        ctx.poke_up(&mut Action::SetL(*value));
    });

    state.add_listener(color_grid, move |value: &mut (f64, f64), mut ctx| {
        let (s, l) = value;
        ctx.poke_up(&mut Action::SetSL(*s, *l));
    });

    let forwarder = EventForwarder::<Action>::new().ui(panel, &mut state);

    state.add_listener(
        forwarder,
        move |action: &mut Action, mut ctx| match action {
            Action::SetH(h) => {
                app.h = *h;
                ctx.poke(color_grid, &mut WhichHSL::Hue(app.h));
            }
            Action::SetS(s) => {
                app.s = *s;
                ctx.poke(color_grid, &mut WhichHSL::Saturation(app.s));
            }
            Action::SetL(l) => {
                app.l = *l;
                ctx.poke(color_grid, &mut WhichHSL::Luminosity(app.l));
            }
            Action::SetSL(s,l) => {
                // dbg!(&h, &l);
                app.s = *s;
                app.l = *l;
                // dbg!(&app.l, &app.h);
                ctx.poke(slider_saturation, &mut app.s);
                ctx.poke(slider_luminosity, &mut app.l);
            }
        },
    );

    state.set_root(forwarder);
    builder.set_handler(Box::new(UiMain::new(state)));
    builder.set_title("Colors, but FREE");
    let window = builder.build().expect("built window");
    window.show();
    run_loop.run();
}
