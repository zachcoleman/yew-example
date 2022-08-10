/*
File has two main sections:
    - Grid: the data structure and implementation
    - GridView: the Yew props and component
*/

use crate::cell;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<cell::Cell>,
}

impl Grid {
    pub fn new(m: usize, n: usize) -> Self {
        Self {
            m: m,
            n: n,
            cells: (0..m * n).map(|_| cell::Cell { clicked: false }).collect(),
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new(self.m, self.n);
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GridViewProps {
    pub grid: Grid,
    pub reset_cb: Callback<()>,
    pub clicked_cell: Callback<(usize, usize)>,
}

pub enum Msg {
    Reset,
}
pub struct GridView;

impl Component for GridView {
    type Message = Msg;
    type Properties = GridViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // sends () to callback so App (parent) can reset grid
            Msg::Reset => {
                ctx.props().reset_cb.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut grid_rows = vec![];
        for row_num in 0..ctx.props().grid.m {
            grid_rows.push(html! {
                <p style="margin:0; padding:0;" >
                    {
                        ctx.props().grid.cells[row_num*ctx.props().grid.m..(row_num+1)*ctx.props().grid.n]
                            .iter()
                            .enumerate()
                            .map(|(j, cell)| html!{
                                < cell::CellView
                                    cell={cell.clone()}
                                    pos={(row_num, j)}
                                    clicked_cell={ctx.props().clicked_cell.clone()}
                                />
                        }).collect::<Html>()
                    }
                </p>
            });
        }
        html! {
            <div>
                { grid_rows }
                <button onclick={link.callback(|_| Msg::Reset)}>{ "Reset Grid" }</button>
            </div>
        }
    }
}
