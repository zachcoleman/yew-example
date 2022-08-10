/*
File has two main sections:
    - Cell: the data structure and implementation
    - CellView: the Yew props and component
*/

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Cell {
    pub clicked: bool,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CellViewProps {
    pub cell: Cell,
    pub pos: (usize, usize),
    pub clicked_cell: Callback<(usize, usize)>,
}

pub enum Msg {
    Click,
}

pub struct CellView;

impl Component for CellView {
    type Message = Msg;
    type Properties = CellViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                // sends cell coordinates through callback to App (parent)
                // so that App can change boolean of the cell in grid
                ctx.props().clicked_cell.emit(ctx.props().pos);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <svg width="90" height="90">
                if ctx.props().cell.clicked{
                    <rect x="0" y="0" width="80" height="80"
                        onclick={link.callback(|_| Msg::Click)}
                        style="fill:green;fill-opacity:0.3;"
                    />
                } else{
                    <rect x="0" y="0" width="80" height="80"
                        onclick={link.callback(|_| Msg::Click)}
                        style="fill:grey;fill-opacity:0.3;"
                    />
                }
            </svg>
        }
    }
}
