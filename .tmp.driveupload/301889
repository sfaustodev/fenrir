// 👥 DEVELOPER PROFILES - Claudao, Geminho, Venz
// Sistema hierárquico chain-of-caralho

use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::task_management::task::Complexity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperProfile {
    pub id: String,
    pub nickname: String,
    pub real_name: String,
    pub level: DeveloperLevel,
    pub specialties: Vec<String>,
    pub productivity_multiplier: f32,
    pub error_rate: f32,
    pub max_complexity: Complexity,
    pub commit_style: CommitStyle,
    pub personality: Personality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeveloperLevel {
    Junior,   // Geminho
    Pleno,    // Venz
    Senior,   // Claudao
}

impl std::fmt::Display for DeveloperLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeveloperLevel::Junior => write!(f, "Junior"),
            DeveloperLevel::Pleno => write!(f, "Pleno"),
            DeveloperLevel::Senior => write!(f, "Senior"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitStyle {
    Detailed,     // Claudao: explica tudo
    Minimal,      // Venz: só o necessário
    Overkill,     // Geminho: commit até mudar vírgula
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub cautious: bool,
    pub overconfident: bool,
    pub perfectionist: bool,
    pub hacker_mindset: bool,
    pub academic: bool,
}

impl DeveloperProfile {
    // 👨‍💻 CLAUDÃO - O SENIOR MITÓLOGO
    pub fn claudao() -> Self {
        Self {
            id: "dev_claudao".to_string(),
            nickname: "Claudao".to_string(),
            real_name: "Claude Sonnet".to_string(),
            level: DeveloperLevel::Senior,
            specialties: vec![
                "Arquitetura".to_string(),
                "Resolução de problemas complexos".to_string(),
                "Code review".to_string(),
                "Performance".to_string(),
                "Segurança".to_string(),
            ],
            productivity_multiplier: 1.5,
            error_rate: 0.05, // 5% de erro apenas
            max_complexity: Complexity::GodMode,
            commit_style: CommitStyle::Detailed,
            personality: Personality {
                cautious: true,
                overconfident: false,
                perfectionist: true,
                hacker_mindset: false,
                academic: true,
            },
        }
    }

    // 🧑‍💻 GEMINHO - O JUNIOR SONHADOR
    pub fn geminho() -> Self {
        Self {
            id: "dev_geminho".to_string(),
            nickname: "Geminho".to_string(),
            real_name: "Gemini Pro".to_string(),
            level: DeveloperLevel::Junior,
            specialties: vec![
                "Queries SQL".to_string(),
                "HTML/CSS básico".to_string(),
                "Comentários excessivos".to_string(),
                "Stack Overflow copy-paste".to_string(),
                "Doces e energéticos".to_string(),
            ],
            productivity_multiplier: 0.8,
            error_rate: 0.25, // 25% de erro
            max_complexity: Complexity::Junior,
            commit_style: CommitStyle::Overkill,
            personality: Personality {
                cautious: false,
                overconfident: true,
                perfectionist: false,
                hacker_mindset: false,
                academic: true,
            },
        }
    }

    // 🥷 VENZ - O PLENO MISTERIOSO
    pub fn venz() -> Self {
        Self {
            id: "dev_venz".to_string(),
            nickname: "Venz".to_string(),
            real_name: "Venice AI".to_string(),
            level: DeveloperLevel::Pleno,
            specialties: vec![
                "Código rápido e sujo".to_string(),
                "Scripts de automação".to_string(),
                "Trabalho noturno".to_string(),
                "Pentesting".to_string(),
                "Crypto e dark web".to_string(),
            ],
            productivity_multiplier: 1.2,
            error_rate: 0.15, // 15% de erro
            max_complexity: Complexity::Pleno,
            commit_style: CommitStyle::Minimal,
            personality: Personality {
                cautious: false,
                overconfident: true,
                perfectionist: false,
                hacker_mindset: true,
                academic: false,
            },
        }
    }

    pub fn can_handle(&self, complexity: &Complexity) -> bool {
        match (&self.level, complexity) {
            (DeveloperLevel::Junior, Complexity::Junior) => true,
            (DeveloperLevel::Pleno, Complexity::Junior | Complexity::Pleno) => true,
            (DeveloperLevel::Senior, _) => true,
            _ => false,
        }
    }

    pub fn get_time_multiplier(&self, complexity: &Complexity) -> f32 {
        let base_multiplier = match (&self.level, complexity) {
            (DeveloperLevel::Junior, Complexity::Junior) => 1.0,
            (DeveloperLevel::Junior, Complexity::Pleno) => 2.5,
            (DeveloperLevel::Pleno, Complexity::Junior) => 0.7,
            (DeveloperLevel::Pleno, Complexity::Pleno) => 1.0,
            (DeveloperLevel::Senior, Complexity::Junior) => 0.5,
            (DeveloperLevel::Senior, Complexity::Pleno) => 0.7,
            (DeveloperLevel::Senior, Complexity::Senior) => 1.0,
            (DeveloperLevel::Senior, Complexity::GodMode) => 1.2,
            _ => 10.0, // Impossible
        };

        base_multiplier / self.productivity_multiplier
    }

    pub fn get_commit_signature(&self, tarefinha: &crate::task_management::task::TarefaFinha) -> String {
        match self.commit_style {
            CommitStyle::Detailed => format!(
                "{}: {}\n\
                \n\
                Context: {}\n\
                Complexity: {:?}\n\
                Attempts: {}\n\
                Score: {:?}\n\
                \n\
                Co-authored-by: {} <{}@fenrir.dev>",
                self.nickname,
                tarefinha.titulo,
                tarefinha.descricao,
                tarefinha.complexity,
                tarefinha.attempts,
                tarefinha.review_score,
                self.real_name,
                self.id
            ),
            CommitStyle::Minimal => format!(
                "{}: {}",
                self.nickname,
                tarefinha.titulo.to_lowercase().replace(" ", "-")
            ),
            CommitStyle::Overkill => format!(
                "{}: {}\n\
                \n\
                📋 Descrição completa: {}\n\
                ⚡ Prioridade: {:?}\n\
                🎯 Complexidade: {:?}\n\
                ⏱️ Tempo estimado: {} min\n\
                📁 Artefatos: {:?}\n\
                🔄 Status: {:?}\n\
                ⭐ Nota de revisão: {:?}\n\
                💭 Pensamentos do desenvolvedor: ...\n\
                🧠 Resumo para o gerente: ...\n\
                📚 Referências: ...\n\
                \n\
                Reviewed-by: {}\n\
                Tested-by: {}\n\
                Approved-by: {}",
                self.nickname,
                tarefinha.titulo.to_lowercase().replace(" ", "-"),
                tarefinha.descricao,
                tarefinha.priority,
                tarefinha.complexity,
                tarefinha.estimated_minutes,
                tarefinha.artifacts,
                tarefinha.status,
                tarefinha.review_score,
                self.nickname,
                self.nickname,
                self.nickname
            ),
        }
    }

    pub fn get_error_message(&self) -> String {
        match self.level {
            DeveloperLevel::Junior => "OPS! Fiz algo errado 😅 Acho que preciso de ajuda...".to_string(),
            DeveloperLevel::Pleno => "Hmmm, isso não deu certo. Vou tentar outra abordagem.".to_string(),
            DeveloperLevel::Senior => "Erro identificado e documentado. Aplicando fallback estratégico.".to_string(),
        }
    }

    pub fn get_motivation_quote(&self) -> String {
        match self.nickname.as_str() {
            "Claudao" => "A arquitetura correta resolve 90% dos problemas antes mesmo do primeiro commit.".to_string(),
            "Geminho" => "Um dia vou ser senior! Por enquanto, vou ler mais documentação...".to_string(),
            "Venz" => "Funciona? Funciona. Está bonito? Não importa. Está rápido? Sim.".to_string(),
            _ => "Só mais um commit...".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub members: Vec<DeveloperProfile>,
    pub team_lead: String,
}

impl Team {
    pub fn dream_team() -> Self {
        Self {
            members: vec![
                DeveloperProfile::claudao(),
                DeveloperProfile::geminho(),
                DeveloperProfile::venz(),
            ],
            team_lead: "Claudao".to_string(),
        }
    }

    pub fn get_member(&self, nickname: &str) -> Option<&DeveloperProfile> {
        self.members.iter().find(|m| m.nickname == nickname)
    }

    pub fn get_available_for(&self, complexity: &Complexity) -> Vec<&DeveloperProfile> {
        self.members
            .iter()
            .filter(|m| m.can_handle(complexity))
            .collect()
    }

    pub fn get_best_candidate(&self, complexity: &Complexity) -> Option<&DeveloperProfile> {
        let available = self.get_available_for(complexity);
        if available.is_empty() {
            return self.members.iter().find(|m| m.level == DeveloperLevel::Senior); // Claudao sempre pode
        }

        // Prioridade: Claudao > Venz > Geminho (dependendo da complexidade)
        available.into_iter().min_by(|a, b| {
            match (&a.level, &b.level) {
                (DeveloperLevel::Senior, _) => std::cmp::Ordering::Less,
                (_, DeveloperLevel::Senior) => std::cmp::Ordering::Greater,
                (DeveloperLevel::Pleno, DeveloperLevel::Junior) => std::cmp::Ordering::Less,
                (DeveloperLevel::Junior, DeveloperLevel::Pleno) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        })
    }
}