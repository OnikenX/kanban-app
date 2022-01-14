pub struct Task {
    pub assignee: String, //person to do it
    pub estimate: u32,    //estimated time(in minutes) to do a question
    pub status: u32,      //goes from 1 to 3
    pub pergunta: Pergunta,
}

pub struct Pergunta {
    pub questao: String,
    pub respostas: Vec<String>,
    pub certa: usize,
}

impl Task {
    pub fn get_tasks() -> Vec<Task> {
        vec![Task {
            assignee: "🐇".to_string(),
            estimate: 2,
            status: 1,
            pergunta: Pergunta {
                questao: "O Kanban é uma metodologia:".to_string(),
                respostas: vec![
                    "Ágil".to_string(),
                    "Tradicional".to_string(),
                    "Híbrida".to_string(),
                    "Evolucionária".to_string(),
                ],
                certa: 1,
            },
        },
        // Task {
        //     assignee: "🐯".to_string(),
        //     estimate: 3,
        //     status: 1,
        //     pergunta: Pergunta {
        //         questao: "Um feedback loop é:".to_string(),
        //         respostas: vec![
        //             "Uma reunião no fim do projeto para retrospetiva".to_string(),
        //             "Uma reunião com os stakeholders".to_string(),
        //             "Um ciclo de trabalho com vista à conclusão de determinada tarefa".to_string(),
        //             "Uma reunião diária para sincronização da equipa".to_string(),
        //         ],
        //         certa: 4,
        //     },
        // },
        Task {
            assignee: "🐫".to_string(),
            estimate: 30,
            status: 1,
            pergunta: Pergunta {
                questao: "O controlo de fluxo permite:".to_string(),
                respostas: vec![
                    "Aumentar o tempo do ciclo médio de produção".to_string(),
                    "Minimizar o número de pessoas da equipa em determinada tarefa".to_string(),
                    "Minimizar o tempo do ciclo médio de produção".to_string(),
                    "Aumentar o número de pessoas da equipa em determinada tarefa".to_string(),
                ],
                certa: 3,
            },
        },
        Task {
            assignee: "🐼".to_string(),
            estimate: 16,
            status: 1,
            pergunta: Pergunta {
                questao: "O que é o limite WIP?".to_string(),
                respostas: vec![
                    "Limite de membros de uma equipa".to_string(),
                    "Limite de tarefas em em desenvolvimento no quadro Kanban".to_string(),
                    "Limite de tarefas a realizar no quadro Kanban".to_string(),
                    "Limite de tarefas concluídas no quadro Kanban".to_string(),
                ],
                certa: 2,
            },
        },
        Task {
            assignee: "🦉".to_string(),
            estimate: 10,
            status: 1,
            pergunta: Pergunta {
                questao: "Kanban é um método sobretudo:".to_string(),
                respostas: vec![
                    "Visual".to_string(),
                    "Documentado".to_string(),
                    "Estático".to_string(),
                ],
                certa: 1,
            },
        },


        // Task {
        //     assignee: "🦉".to_string(),
        //     estimate: 10,
        //     status: 1,
        //     pergunta: Pergunta {
        //         questao: "Qual destas vantagens o Kanban pode adicionar ao Scrum, quando utilizados em conjunto?".to_string(),
        //         respostas: vec![
        //             "Facilidade na realização do Product Backlog".to_string(),
        //             "Facilidade na definição dos requisitos".to_string(),
        //             "Visualização da produtividade individual de cada membro".to_string(),
        //         ],
        //         certa: 3,
        //     },
        // },

        Task {
            assignee: "🦂".to_string(),
            estimate: 15,
            status: 1,
            pergunta: Pergunta {
                questao: "Em qual destas situações podemos afirmar que o uso do Kanban está a ser eficiente?".to_string(),
                respostas: vec![
                    "tarefas em “to do” e “doing” > tarefas em “done”".to_string(),
                    "tarefas em “done” e “to do” > tarefas em “doing”".to_string(),
                    "tarefas em “to do” e “doing” < tarefas em “done”".to_string(),
                    "tarefas em “doing” > tarefas em “to do”".to_string(),
                ],
                certa: 3,
            },
        },
        Task {
            assignee: "🦅".to_string(),
            estimate: 8,
            status: 1,
            pergunta: Pergunta {
                questao: "Quantas colunas constituem, no mínimo, um quadro Kanban?".to_string(),
                respostas: vec![
                    "4".to_string(),
                    "3".to_string(),
                    "2".to_string(),
                    "1".to_string(),
                ],
                certa: 2,
            },
        },
        ]
    }
}
