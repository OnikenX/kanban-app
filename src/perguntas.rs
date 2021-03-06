use yew::prelude::*;
use yew::Context;

use crate::{
    tasks::{Pergunta, Task},
    Model, Msg,
};

impl Model {
    pub fn view_perguntas(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="section">
                {for self.state.tasks.iter().enumerate().filter(|e| e.1.status == 2).map(|e| self.view_pergunta(ctx, e.0, &e.1.pergunta))}
            </div>
        }
    }

    pub fn view_pergunta(&self, ctx: &Context<Self>, task_nome: usize, pergunta: &Pergunta) -> Html {
        html! {
            <div class="card animate__animated animate__fadeIn">
                <div class="card-content">
                    { &pergunta.questao }
                </div>
                <div>
                    {for pergunta.respostas.iter().enumerate().map(|e| self.view_resposta(ctx, task_nome, e.0, e.1))}
                </div>
            </div>
        }
    }
    pub fn view_resposta(&self, ctx : &Context<Self>, pergunta_nome: usize, idx: usize, resposta: &str) -> Html {

        let resposta_html = ||html!{
            <a class="card-footer-item" onclick={ctx.link().callback(move |_| Msg::Responder{task_idx:pergunta_nome, resp_idx:idx})}>{ resposta }</a>
        };
        if self
            .state
            .state_resposta
            .iter()
            .any(|e| e.pergunta_nome == pergunta_nome && e.resposta_index == idx)
        {
            html! {
                <footer class="card-footer" style="color:red;">
                    {resposta_html()}
                </footer>
            }
        } else {
            html! {
                <footer class="card-footer">
                    {resposta_html()}
                </footer>
            }
        }
    }
}
