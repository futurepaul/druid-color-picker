use druid_shell::platform::WindowBuilder;
use druid_shell::win_main;

use druid::widget::{Column, EventForwarder, Label, Padding, Slider};
use druid::{UiMain, UiState};

use druid::Id;

pub mod widgets;
use widgets::{HSL, RBF};

#[derive(Clone, Copy, Debug)]
struct AppState {
    u: f64,
    v: f64,
}

fn pad(widget: Id, state: &mut UiState) -> Id {
    Padding::uniform(5.0).ui(widget, state)
}

#[derive(Debug, Clone)]
pub enum Action {
    SetU(f64),
    SetV(f64),
}

fn main() {
    druid_shell::init();

    let mut run_loop = win_main::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let mut state = UiState::new();

    let mut app = AppState { u: 1.0, v: 1.0 };

    let slider_1 = Slider::new(app.u).ui(&mut state);
    let slider1_padded = pad(slider_1, &mut state);

    let slider_2 = Slider::new(app.v).ui(&mut state);
    let slider2_padded = pad(slider_2, &mut state);

    let grid = widgets::RBF::new().ui(&mut state);
    let grid_padded = pad(grid, &mut state);

    let column = Column::new();
    let panel = column.ui(&[grid_padded, slider1_padded, slider2_padded], &mut state);

    state.add_listener(slider_1, move |value: &mut f64, mut ctx| {
        ctx.poke_up(&mut Action::SetU(*value));
    });

    state.add_listener(slider_2, move |value: &mut f64, mut ctx| {
        ctx.poke_up(&mut Action::SetV(*value));
    });

    let forwarder = EventForwarder::<Action>::new().ui(panel, &mut state);

    state.add_listener(
        forwarder,
        move |action: &mut Action, mut ctx| match action {
            Action::SetU(u) => {
                app.u = *u;
                ctx.poke(grid, &mut (app.u, app.v));
            }
            Action::SetV(v) => {
                app.v = *v;
                ctx.poke(grid, &mut (app.u, app.v));
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
