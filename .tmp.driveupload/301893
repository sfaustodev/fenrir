// 🔍 REVIEW ENGINE - Claudao revisa tudo
// Sistema rigoroso de qualidade

use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::task_management::{
    team_profiles::DeveloperProfile,
    task::TarefaFinha,
    commit_system::CommitInfo
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewEngine {
    pub senior_reviewer: DeveloperProfile,
    pub review_queue: Vec<PendingReview>,
    pub completed_reviews: Vec<CompletedReview>,
    pub auto_approve_threshold: u8,
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingReview {
    pub tarefinha: TarefaFinha,
    pub commit: CommitInfo,
    pub submitted_at: u64,
    pub priority: ReviewPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewPriority {
    Critical, // Precisa revisar AGORA
    High,     // Prioridade alta
    Normal,   // Fluxo normal
    Low,      // Quando der tempo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedReview {
    pub review_id: String,
    pub original_tarefinha: TarefaFinha,
    pub commit: CommitInfo,
    pub reviewer: String,
    pub score: u8, // 1-10
    pub feedback: Vec<FeedbackItem>,
    pub approved: bool,
    pub review_time_minutes: u16,
    pub reviewed_at: u64,
    pub follow_up_tasks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackItem {
    pub category: FeedbackCategory,
    pub severity: Severity,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub line_number: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackCategory {
    Logic,        // Erro de lógica
    Performance,  // Problema de performance
    Security,     // Vulnerabilidade
    Style,        // Code style
    Documentation, // Falta docs
    Testing,      // Falta testes
    Architecture, // Problema de arquitetura
    Bug,          // Bug identificado
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical, // Blocker
    High,     // Must fix
    Medium,   // Should fix
    Low,      // Nice to fix
    Info,     // Just a suggestion
}

impl ReviewEngine {
    pub fn new(senior_reviewer: DeveloperProfile) -> Self {
        Self {
            senior_reviewer,
            review_queue: vec![],
            completed_reviews: vec![],
            auto_approve_threshold: 8,
            strict_mode: true,
        }
    }

    pub fn submit_for_review(&mut self, tarefinha: TarefaFinha, commit: CommitInfo) -> String {
        let review_id = format!("review_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let priority = self.determine_review_priority(&tarefinha);

        let pending_review = PendingReview {
            tarefinha,
            commit,
            submitted_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            priority,
        };

        self.review_queue.push(pending_review);

        println!("📋 TarefaFinha {} enviada para review", review_id);
        review_id
    }

    fn determine_review_priority(&self, tarefinha: &TarefaFinha) -> ReviewPriority {
        match tarefinha.priority {
            crate::task_management::task::Priority::Critical => ReviewPriority::Critical,
            crate::task_management::task::Priority::High => ReviewPriority::High,
            crate::task_management::task::Priority::Medium => ReviewPriority::Normal,
            crate::task_management::task::Priority::Low => ReviewPriority::Low,
        }
    }

    pub fn process_next_review(&mut self) -> Result<Option<CompletedReview>> {
        if let Some(pending) = self.review_queue.pop() {
            let completed_review = self.perform_review(pending)?;
            Ok(Some(completed_review))
        } else {
            Ok(None)
        }
    }

    fn perform_review(&mut self, pending: PendingReview) -> Result<CompletedReview> {
        let start_time = std::time::SystemTime::now();

        println!("🔍 Claudao revisando: {}", pending.tarefinha.titulo);
        println!("👤 Autor: {}", pending.commit.author);

        let feedback = self.generate_feedback(&pending.tarefinha, &pending.commit);
        let score = self.calculate_score(&pending.tarefinha, &feedback);
        let approved = score >= self.auto_approve_threshold && !self.has_critical_issues(&feedback);

        let review_time = start_time
            .duration_since(start_time)
            .unwrap_or_default()
            .as_secs() as u16;

        let completed_review = CompletedReview {
            review_id: format!("completed_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            original_tarefinha: pending.tarefinha.clone(),
            commit: pending.commit,
            reviewer: self.senior_reviewer.nickname.clone(),
            score,
            feedback,
            approved,
            review_time_minutes: review_time,
            reviewed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            follow_up_tasks: if !approved {
                self.generate_follow_up_tasks(&pending.tarefinha)
            } else {
                vec![]
            },
        };

        self.completed_reviews.push(completed_review.clone());

        self.show_review_result(&completed_review);

        Ok(completed_review)
    }

    fn generate_feedback(&self, tarefinha: &TarefaFinha, commit: &CommitInfo) -> Vec<FeedbackItem> {
        let mut feedback = vec![];

        // Feedback baseado no autor
        match commit.author.as_str() {
            "Geminho" => {
                // Geminho sempre comete erros de junior
                feedback.push(FeedbackItem {
                    category: FeedbackCategory::Style,
                    severity: Severity::Medium,
                    message: "Comentários excessivos e desnecessários".to_string(),
                    suggested_fix: Some("Remover comentários óbvios e focar no que importa".to_string()),
                    line_number: None,
                });

                feedback.push(FeedbackItem {
                    category: FeedbackCategory::Logic,
                    severity: Severity::High,
                    message: "Lógica mais complexa que o necessário".to_string(),
                    suggested_fix: Some("Simplificar abordagem usando pattern mais direto".to_string()),
                    line_number: Some(42),
                });

                if tarefinha.attempts > 1 {
                    feedback.push(FeedbackItem {
                        category: FeedbackCategory::Bug,
                        severity: Severity::High,
                        message: "Precisou de múltiplas tentativas".to_string(),
                        suggested_fix: Some("Estudar mais antes de implementar".to_string()),
                        line_number: None,
                    });
                }
            }
            "Venz" => {
                // Venz é rápido mas às vezes descuidado
                feedback.push(FeedbackItem {
                    category: FeedbackCategory::Documentation,
                    severity: Severity::Medium,
                    message: "Falta documentação".to_string(),
                    suggested_fix: Some("Adicionar docs básicas para manutenção".to_string()),
                    line_number: None,
                });

                if tarefinha.estimated_minutes < 10 {
                    feedback.push(FeedbackItem {
                        category: FeedbackCategory::Performance,
                        severity: Severity::Low,
                        message: "Implementação muito rápida".to_string(),
                        suggested_fix: Some("Verificar se há edge cases não tratados".to_string()),
                        line_number: None,
                    });
                }
            }
            "Claudao" => {
                // Claudao quase não tem feedback negativo
                if tarefinha.review_score.unwrap_or(10) < 9 {
                    feedback.push(FeedbackItem {
                        category: FeedbackCategory::Logic,
                        severity: Severity::Low,
                        message: "Pequena oportunidade de melhoria".to_string(),
                        suggested_fix: Some("Considerar pattern X para elegância".to_string()),
                        line_number: Some(100),
                    });
                }
            }
            _ => {}
        }

        // Feedback baseado na complexidade
        if !tarefinha.assignee.can_handle(&tarefinha.complexity) {
            feedback.push(FeedbackItem {
                category: FeedbackCategory::Architecture,
                severity: Severity::Critical,
                message: "Tarefa acima do nível do desenvolvedor".to_string(),
                suggested_fix: Some("Delegar para nível apropriado ou fornecer suporte".to_string()),
                line_number: None,
            });
        }

        feedback
    }

    fn calculate_score(&self, tarefinha: &TarefaFinha, feedback: &[FeedbackItem]) -> u8 {
        let mut base_score = 10u8;

        // Penaliza por tentativas
        base_score = base_score.saturating_sub(tarefinha.attempts.saturating_sub(1) * 2);

        // Penaliza por feedback
        for item in feedback {
            let penalty = match item.severity {
                Severity::Critical => 4,
                Severity::High => 3,
                Severity::Medium => 2,
                Severity::Low => 1,
                Severity::Info => 0,
            };
            base_score = base_score.saturating_sub(penalty);
        }

        // Bônus por rapidez
        if tarefinha.estimated_minutes > 0 {
            let actual_time = if let (Some(started), Some(completed)) = (tarefinha.started_at, tarefinha.completed_at) {
                completed - started
            } else {
                tarefinha.estimated_minutes as u64 * 60
            };

            if actual_time < tarefinha.estimated_minutes as u64 * 60 {
                base_score = (base_score + 1).min(10);
            }
        }

        base_score
    }

    fn has_critical_issues(&self, feedback: &[FeedbackItem]) -> bool {
        feedback.iter().any(|f| matches!(f.severity, Severity::Critical))
    }

    fn generate_follow_up_tasks(&self, tarefinha: &TarefaFinha) -> Vec<String> {
        vec![
            format!("Corrigir feedback apontados na revisão"),
            format!("Adicionar testes unitários"),
            format!("Melhorar documentação"),
            format!("Verificar performance"),
            format!("Testar edge cases"),
        ]
    }

    fn show_review_result(&self, review: &CompletedReview) {
        println!("\n🔍 REVIEW CONCLUÍDO");
        println!("═════════════════════════════");
        println!("📋 Tarefa: {}", review.original_tarefinha.titulo);
        println!("👤 Autor: {}", review.commit.author);
        println!("⭐ Nota: {}/10", review.score);
        println!("✅ Status: {}", if review.approved { "APROVADO" } else { "REPROVADO" });
        println!("⏱️ Tempo: {} min", review.review_time_minutes);

        if !review.feedback.is_empty() {
            println!("\n📝 Feedback:");
            for (i, item) in review.feedback.iter().enumerate() {
                let icon = match item.severity {
                    Severity::Critical => "🚨",
                    Severity::High => "⚠️",
                    Severity::Medium => "📋",
                    Severity::Low => "💡",
                    Severity::Info => "ℹ️",
                };

                println!("   {}. {} {}", i + 1, icon, item.message);
                if let Some(fix) = &item.suggested_fix {
                    println!("      💡 Sugestão: {}", fix);
                }
            }
        }

        if !review.follow_up_tasks.is_empty() {
            println!("\n📋 Tarefas follow-up:");
            for task in &review.follow_up_tasks {
                println!("   • {}", task);
            }
        }

        println!("{}", "─".repeat(50));
    }

    pub fn get_pending_count(&self) -> usize {
        self.review_queue.len()
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed_reviews.len()
    }

    pub fn get_approval_rate(&self) -> f32 {
        if self.completed_reviews.is_empty() {
            return 0.0;
        }

        let approved = self.completed_reviews
            .iter()
            .filter(|r| r.approved)
            .count();

        approved as f32 / self.completed_reviews.len() as f32
    }

    pub fn get_average_score(&self) -> f32 {
        if self.completed_reviews.is_empty() {
            return 0.0;
        }

        let total: u32 = self.completed_reviews
            .iter()
            .map(|r| r.score as u32)
            .sum();

        total as f32 / self.completed_reviews.len() as f32
    }

    pub fn show_metrics_dashboard(&self) {
        println!("\n📊 REVIEW METRICS");
        println!("═════════════════════════");
        println!("📋 Pending Reviews: {}", self.get_pending_count());
        println!("✅ Completed Reviews: {}", self.get_completed_count());
        println!("📈 Approval Rate: {:.1}%", self.get_approval_rate() * 100.0);
        println!("⭐ Average Score: {:.1}/10", self.get_average_score());
    }
}