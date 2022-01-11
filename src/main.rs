mod kanban;
mod perguntas;
mod tasks;

use yew::prelude::*;
use tasks::Task;

struct Model {
    state: State,
}

struct State {
    tasks: Vec<Task>,
    state_resposta: Vec<Resposta_Status>,
}

pub struct Resposta_Status {
    pergunta_nome: usize,
    resposta_index: usize,
}

impl State {
    fn remove_responses_states(states: &mut Vec<Resposta_Status>, idx: usize) {
        let mut i = 0;
        while states.len() > i {
            if states[i].pergunta_nome == idx {
                states.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn increase_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status < 3).map(|e| {
            e.status = e.status + 1;
            State::remove_responses_states(&mut self.state_resposta, idx);
        });
    }

    fn decrease_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status > 1).map(|e| {
            e.status = e.status - 1;
            State::remove_responses_states(&mut self.state_resposta, idx);
        });
    }

    fn verificar_resposta(&mut self, task_idx: usize, resp_idx: usize) {
        if self
            .state_resposta
            .iter()
            .any(|f| f.pergunta_nome == task_idx && f.resposta_index == resp_idx)
        {
            return;
        }

        let task = self.tasks.get(task_idx);
        if task.is_none() {
            return;
        }

        let pergunta = &task.unwrap().pergunta;
        if pergunta.certa-1 != resp_idx {
            self.state_resposta.push(Resposta_Status {
                pergunta_nome: task_idx,
                resposta_index: resp_idx,
            });
        }else{
            self.increase_status(task_idx);
        }
    }
}

enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
    Responder { task_idx: usize, resp_idx: usize },
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        
        Model {
            state: State {
                tasks: tasks::Task::get_tasks(),
                state_resposta: vec![],
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::IncreaseStatus(idx) => self.state.increase_status(idx),
            Msg::DecreaseStatus(idx) => self.state.decrease_status(idx),
            Msg::Responder {
                task_idx: pergunta,
                resp_idx: resposta,
            } => self.state.verificar_resposta(pergunta, resposta),
        }
        true
    }


    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <section class="section">
                    <div class="container">
                        <h1 class="title">
                            { "Kanban Tutorial"}
                        </h1>
                    </div>
                </section>
                {self.main_content(ctx)}
            </>
        }
    }

  
}

impl Model {
    fn main_content(&self, ctx: &Context<Self>) -> Html {
        if self.state.tasks.iter().any(|e| e.status == 2) {
            html! {
                <div class="columns">
                    <div class="column">
                        { self.view_kanban(ctx)}
                    </div>
                    <div class="column" id="perguntas">
                        {self.view_perguntas(ctx)}
                    </div >
                </div>
            }
        } else {
            html! {
                {self.view_kanban(ctx)}
            }
        }
    }

    fn view_column(&self, ctx: &Context<Self>, status: u32, status_text: &str, tasks: &Vec<Task>) -> Html {
        html! {
            <div class={format!("column status-{}", status)}>
                <div class="tags has-addons">
                    <span class="tag">{ status_text }</span> <span class="tag is-dark">{ tasks.iter().filter(|e| e.status == status).count() }</span>
                </div>
                { for tasks.iter().enumerate().filter(|e| e.1.status == status).map(|e| self.view_task(ctx, e)) }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
