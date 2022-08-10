use yew::prelude::*;

mod cell;
mod grid;

pub enum Msg {
    ClickedCell { pos: (usize, usize) },
    Reset,
}

pub struct App {
    grid: grid::Grid,
}

impl Component for App {
    /// this type needs to encompass all possible callbacks to this component
    type Message = Msg;

    /// App is highest-level component so nothing to pass it really
    type Properties = ();

    /// App will manage all state in this example and child components
    /// will use callbacks to message changes that should be made to
    /// the state of the application
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            grid: grid::Grid::new(10, 10),
        }
    }

    /// Process and act on all the callbacks  
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                log::info!("reset grid");
                self.grid.reset();
                true
            }
            Msg::ClickedCell { pos: (a, b) } => {
                log::info!("pos {:?} clicked", (a, b));
                self.grid.cells[a * self.grid.n + b].clicked =
                    !self.grid.cells[a * self.grid.n + b].clicked;
                true
            }
        }
    }

    /// App level html view
    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset_grid = ctx.link().callback(|_| Msg::Reset);
        let clicked_cell = ctx
            .link()
            .callback(|(a, b)| Msg::ClickedCell { pos: (a, b) });
        html! {
            <>
                < grid::GridView
                    reset_cb={reset_grid}
                    clicked_cell={clicked_cell}
                    grid={self.grid.clone()}
                />
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
