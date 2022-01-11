use yew::prelude::*;

use crate::{Model, tasks::Task, Msg};

//views related to the kanban

impl Model {
    pub fn view_kanban(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
        <section id="board" class="section">
            <div class="container">
                <div class="columns">
                    { self.view_column(ctx, 1, "To Do", &self.state.tasks) }
                    { self.view_column(ctx, 2, "In Progress", &self.state.tasks) }
                    { self.view_column(ctx, 3, "Done"  , &self.state.tasks) }
                </div>
            </div>
        </section>
        </>
        }
    }

    pub fn view_task(&self,ctx: &Context<Self>, (idx, task): (usize, &Task)) -> Html {
        let pergunta_nome = format! {"Pergunta {}", idx+1};
        let button = || {
            if task.status == 1{
                html!{
                    <a class="card-footer-item" onclick={ctx.link().callback(move |_| Msg::IncreaseStatus(idx))}>{ "▶︎︎" }</a>
                }
            }else if task.status == 2{
                html!{
                    <a  class="card-footer-item" onclick={ctx.link().callback(move |_| Msg::DecreaseStatus(idx))}>{ "◀︎︎" }</a>
                }
            }else{
                html!{}
            }
        };
        
        html! {
            <div class="card">
                <div class="card-content">
                    {pergunta_nome}
                </div>
                <footer class="card-footer">
                    <div class="card-footer-item">{ &task.assignee }</div>
                    <div class="card-footer-item">{ format!("{} minutes", &task.estimate) }</div>
                </footer>
                <footer class="card-footer">
                    {button()}
                </footer>
            </div>
        }
    }
}
