// 🔗 CHAIN MANAGER - Orquestrador principal do Chain-of-Caralho
// Claudao gerencia Geminho e Venz como um verdadeiro senior

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::collections::HashMap;
use crate::task_management::{
    task::{TarefaFinha, TarefaFinhaBatch},
    team_profiles::{DeveloperProfile, Team},
    commit_system::{CommitQueue, CommitInfo},
    review_system::ReviewEngine
};

#[derive(Debug, Clone)]
struct TarefaFinhaConfig {
    titulo: String,
    descricao: String,
    complexity: crate::task_management::task::Complexity,
    priority: crate::task_management::task::Priority,
    estimated_minutes: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCaralhoManager {
    pub team: Team,
    pub commit_queue: CommitQueue,
    pub review_engine: ReviewEngine,
    pub active_batches: Vec<TarefaFinhaBatch>,
    pub completed_batches: Vec<TarefaFinhaBatch>,
    pub metrics: ChainMetrics,
    pub execution_mode: ExecutionMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    Sequential,  // Uma por vez
    Parallel,    // Múltiplas simultâneas
    Hybrid,      // Inteligente
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetrics {
    pub total_tarefinhas_created: usize,
    pub total_tarefinhas_completed: usize,
    pub average_completion_time: f32,
    pub developer_performance: HashMap<String, DeveloperMetrics>,
    pub batch_success_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperMetrics {
    pub tarefinhas_completed: usize,
    pub average_score: f32,
    pub success_rate: f32,
    pub total_time_minutes: u64,
    pub specialties_completed: Vec<String>,
}

impl ChainOfCaralhoManager {
    pub fn new() -> Self {
        let team = Team::dream_team();
        let senior = team.get_member("Claudao").unwrap().clone();

        Self {
            team: team.clone(),
            commit_queue: CommitQueue::new(),
            review_engine: ReviewEngine::new(senior),
            active_batches: vec![],
            completed_batches: vec![],
            metrics: ChainMetrics {
                total_tarefinhas_created: 0,
                total_tarefinhas_completed: 0,
                average_completion_time: 0.0,
                developer_performance: HashMap::new(),
                batch_success_rate: 0.0,
            },
            execution_mode: ExecutionMode::Hybrid,
        }
    }

    /// 🎯 CRIAR BATCH DE TAREFINHAS A PARTIR DE UM OBJETIVO
    pub fn create_batch_from_goal(&mut self, goal: String) -> Result<String> {
        println!("🎯 Analisando objetivo: {}", goal);
        println!("🧠 Claudao decompondo em tarefinhas impossíveis de errar...");

        let batch_id = self.generate_tarefinhas_for_goal(&goal)?;

        println!("✅ Batch {} criado com {} tarefinhas", batch_id,
                self.active_batches.last().unwrap().tarefinhas.len());

        Ok(batch_id)
    }

    fn generate_tarefinhas_for_goal(&mut self, goal: &str) -> Result<String> {
        let mut batch = TarefaFinhaBatch::new(goal.to_string(), "Claudao".to_string());

        // Análise inteligente do objetivo e decomposição
        let tarefinhas = self.decompose_goal(goal);

        for tarefinha_config in tarefinhas {
            let assignee = self.assign_tarefinha(&tarefinha_config.complexity);
            let tarefinha = TarefaFinha::new(
                tarefinha_config.titulo,
                tarefinha_config.descricao,
                assignee,
                tarefinha_config.priority,
                tarefinha_config.complexity,
                tarefinha_config.estimated_minutes,
            );

            batch.add_tarefa(tarefinha);
        }

        let batch_id = batch.batch_id.clone();
        let tarefinhas_count = batch.tarefinhas.len();
        self.active_batches.push(batch);
        self.metrics.total_tarefinhas_created += tarefinhas_count;

        Ok(batch_id)
    }

    fn decompose_goal(&self, goal: &str) -> Vec<TarefaFinhaConfig> {
        let mut tarefinhas = vec![];

        // Tarefinhas padrão para qualquer objetivo
        if goal.to_lowercase().contains("modulo") || goal.to_lowercase().contains("sistema") {
            tarefinhas.extend_from_slice(&[
                TarefaFinhaConfig {
                    titulo: "Criar estrutura de diretórios".to_string(),
                    descricao: "Criar módulo principal com arquivos básicos".to_string(),
                    complexity: crate::task_management::Complexity::Junior,
                    priority: crate::task_management::Priority::Critical,
                    estimated_minutes: 5,
                },
                TarefaFinhaConfig {
                    titulo: "Implementar estrutura de dados".to_string(),
                    descricao: "Definir structs e enums principais".to_string(),
                    complexity: crate::task_management::Complexity::Pleno,
                    priority: crate::task_management::Priority::High,
                    estimated_minutes: 15,
                },
                TarefaFinhaConfig {
                    titulo: "Implementar lógica de negócio".to_string(),
                    descricao: "Criar funções principais do sistema".to_string(),
                    complexity: crate::task_management::Complexity::Senior,
                    priority: crate::task_management::Priority::High,
                    estimated_minutes: 30,
                },
                TarefaFinhaConfig {
                    titulo: "Adicionar testes unitários".to_string(),
                    descricao: "Criar testes para funcionalidades principais".to_string(),
                    complexity: crate::task_management::Complexity::Pleno,
                    priority: crate::task_management::Priority::Medium,
                    estimated_minutes: 20,
                },
                TarefaFinhaConfig {
                    titulo: "Documentar código".to_string(),
                    descricao: "Adicionar docs e comentários".to_string(),
                    complexity: crate::task_management::Complexity::Junior,
                    priority: crate::task_management::Priority::Low,
                    estimated_minutes: 10,
                },
            ]);
        }

        // Se for sobre chain-of-caralalo específico
        if goal.to_lowercase().contains("chain") || goal.to_lowercase().contains("hierarquia") {
            tarefinhas.push(TarefaFinhaConfig {
                titulo: "Implementar sistema de delegação".to_string(),
                descricao: "Criar sistema de delegação automática de tarefinhas".to_string(),
                complexity: crate::task_management::Complexity::Senior,
                priority: crate::task_management::Priority::Critical,
                estimated_minutes: 45,
            });
        }

        tarefinhas
    }

    fn assign_tarefinha(&self, complexity: &crate::task_management::Complexity) -> DeveloperProfile {
        self.team.get_best_candidate(complexity)
            .unwrap_or_else(|| self.team.get_member("Claudao").unwrap())
            .clone()
    }

    /// 🚀 EXECUTAR PRÓXIMA TAREFINHA DISPONÍVEL
    pub fn execute_next_tarefinha(&mut self) -> Result<Option<(TarefaFinha, CommitInfo)>> {
        // Encontrar próxima tarefinha executável
        let (batch_index, tarefinha_index) = self.find_next_executable_tarefinha()?;

        let tarefinha = {
            let batch = &mut self.active_batches[batch_index];
            batch.tarefinhas.remove(tarefinha_index)
        };

        println!("🚀 Executando: {}", tarefinha.titulo);
        println!("👤 Assignee: {}", tarefinha.assignee.nickname);

        // Simular execução
        let mut tarefinha = tarefinha;
        self.simulate_tarefinha_execution(&mut tarefinha)?;

        // Criar commit
        let commit = CommitInfo::from_tarefinha(&tarefinha, &tarefinha.assignee);

        // Adicionar ao batch novamente (agora completed)
        {
            let batch = &mut self.active_batches[batch_index];
            batch.tarefinhas.push(tarefinha.clone());
        }

        Ok(Some((tarefinha, commit)))
    }

    fn find_next_executable_tarefinha(&self) -> Result<(usize, usize)> {
        for (batch_idx, batch) in self.active_batches.iter().enumerate() {
            for (tarefinha_idx, tarefinha) in batch.tarefinhas.iter().enumerate() {
                if matches!(tarefinha.status, crate::task_management::TarefaStatus::Pending) {
                    return Ok((batch_idx, tarefinha_idx));
                }
            }
        }

        Err(anyhow::anyhow!("Nenhuma tarefinha disponível para execução"))
    }

    fn simulate_tarefinha_execution(&self, tarefinha: &mut TarefaFinha) -> Result<()> {
        tarefinha.start();

        // Simular tempo de execução baseado no developer
        let actual_minutes = (tarefinha.estimated_minutes as f32 *
            tarefinha.assignee.get_time_multiplier(&tarefinha.complexity)) as u16;

        println!("⏱️ Executando por {} minutos (estimado: {})",
                actual_minutes, tarefinha.estimated_minutes);

        // Simular completion
        let artifacts = vec![
            format!("src/{}.rs", tarefinha.titulo.to_lowercase().replace(" ", "_")),
            format!("tests/test_{}.rs", tarefinha.titulo.to_lowercase().replace(" ", "_")),
        ];

        tarefinha.complete(artifacts);

        // Simular possíveis erros baseados no developer
        match tarefinha.assignee.nickname.as_str() {
            "Geminho" => {
                if rand::random::<f32>() < 0.3 { // 30% chance de erro
                    return self.simulate_tarefinha_failure(tarefinha);
                }
            }
            "Venz" => {
                if rand::random::<f32>() < 0.15 { // 15% chance de erro
                    return self.simulate_tarefinha_failure(tarefinha);
                }
            }
            "Claudao" => {
                if rand::random::<f32>() < 0.05 { // 5% chance de erro
                    return self.simulate_tarefinha_failure(tarefinha);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn simulate_tarefinha_failure(&self, tarefinha: &mut TarefaFinha) -> Result<()> {
        let error = tarefinha.assignee.get_error_message();
        tarefinha.fail(error);

        // Tentar novamente se for Geminho (ele aprende com erros)
        if tarefinha.assignee.nickname == "Geminho" && tarefinha.attempts < 3 {
            println!("🔄 Geminho tentando novamente...");
            return self.simulate_tarefinha_execution(tarefinha);
        }

        Err(anyhow::anyhow!("Tarefinha falhou: {}", tarefinha.error_message.as_ref().unwrap_or(&"Erro desconhecido".to_string())))
    }

    /// 🔄 PROCESSAR BATCH COMPLETO
    pub async fn process_batch(&mut self, batch_id: &str) -> Result<()> {
        println!("\n🔄 PROCESSANDO BATCH: {}", batch_id);
        println!("{}", "═".repeat(60));

        let batch_idx = self.active_batches
            .iter()
            .position(|b| b.batch_id == batch_id)
            .context("Batch não encontrado")?;

        let mut completed_tarefinhas = 0;
        let total_tarefinhas = self.active_batches[batch_idx].tarefinhas.len();

        while completed_tarefinhas < total_tarefinhas {
            match self.execute_next_tarefinha() {
                Ok(Some((tarefinha, commit))) => {
                    // Commit
                    self.commit_queue.add_commit(commit.clone());
                    self.commit_queue.process_next_commit()?;

                    // Review
                    self.review_engine.submit_for_review(tarefinha.clone(), commit);
                    self.review_engine.process_next_review()?;

                    completed_tarefinhas += 1;

                    println!("📊 Progresso: {}/{} tarefinhas completas", completed_tarefinhas, total_tarefinhas);
                }
                Ok(None) => {
                    println!("ℹ️ Nenhuma tarefinha disponível");
                    break;
                }
                Err(e) => {
                    println!("❌ Erro na execução: {}", e);
                    break;
                }
            }
        }

        // Mover batch para completed
        let batch = self.active_batches.remove(batch_idx);
        self.completed_batches.push(batch);

        println!("\n✅ BATCH {} CONCLUÍDO!", batch_id);
        self.update_metrics();

        Ok(())
    }

    /// 📊 ATUALIZAR MÉTRICAS
    fn update_metrics(&mut self) {
        self.metrics.total_tarefinhas_completed = self.completed_batches
            .iter()
            .map(|b| b.tarefinhas.len())
            .sum();

        // Atualizar métricas por developer
        for batch in &self.completed_batches {
            for tarefinha in &batch.tarefinhas {
                let dev_metrics = self.metrics.developer_performance
                    .entry(tarefinha.assignee.nickname.clone())
                    .or_insert(DeveloperMetrics {
                        tarefinhas_completed: 0,
                        average_score: 0.0,
                        success_rate: 0.0,
                        total_time_minutes: 0,
                        specialties_completed: vec![],
                    });

                dev_metrics.tarefinhas_completed += 1;
                if let Some(score) = tarefinha.review_score {
                    dev_metrics.average_score = (dev_metrics.average_score + score as f32) / 2.0;
                }

                if matches!(tarefinha.status, crate::task_management::TarefaStatus::Approved) {
                    dev_metrics.success_rate += 0.1;
                }
            }
        }

        self.metrics.batch_success_rate = if !self.completed_batches.is_empty() {
            self.completed_batches.iter().map(|b| b.get_completion_rate()).sum::<f32>()
                / self.completed_batches.len() as f32
        } else {
            0.0
        };
    }

    /// 📈 MOSTRAR DASHBOARD COMPLETO
    pub fn show_dashboard(&self) {
        println!("\n🔥 CHAIN-OF-CARALHO DASHBOARD");
        println!("{}", "═".repeat(60));

        println!("📊 GERAL:");
        println!("   📋 Tarefinhas Criadas: {}", self.metrics.total_tarefinhas_created);
        println!("   ✅ Tarefinhas Completas: {}", self.metrics.total_tarefinhas_completed);
        println!("   📈 Taxa de Sucesso: {:.1}%", self.metrics.batch_success_rate * 100.0);

        println!("\n👥 PERFORMANCE POR DEVELOPER:");
        for (nickname, metrics) in &self.metrics.developer_performance {
            println!("   {}:", nickname);
            println!("     📋 Completas: {}", metrics.tarefinhas_completed);
            println!("     ⭐ Score Médio: {:.1}/10", metrics.average_score);
            println!("     ✅ Taxa Sucesso: {:.1}%", metrics.success_rate * 100.0);
        }

        println!("\n🔄 BATCHES ATIVOS: {}", self.active_batches.len());
        println!("✅ BATCHES CONCLUÍDOS: {}", self.completed_batches.len());

        self.review_engine.show_metrics_dashboard();
        self.commit_queue.get_processed_count();
    }

    pub fn get_status(&self) -> ChainStatus {
        ChainStatus {
            active_batches: self.active_batches.len(),
            completed_batches: self.completed_batches.len(),
            pending_commits: self.commit_queue.get_pending_count(),
            pending_reviews: self.review_engine.get_pending_count(),
            total_tarefinhas: self.metrics.total_tarefinhas_created,
            completion_rate: self.metrics.batch_success_rate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStatus {
    pub active_batches: usize,
    pub completed_batches: usize,
    pub pending_commits: usize,
    pub pending_reviews: usize,
    pub total_tarefinhas: usize,
    pub completion_rate: f32,
}