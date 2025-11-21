// 🔥 FENRIR BASIC INTERACTIVE - O básico que funcionava
// Sem IA pra não dar merda - comandos diretos

use std::io::{self, Write};
use anyhow::{Result, anyhow};
use crate::operations::FenrirOperations;
use crate::venz_agent::{VenzAgent, get_venz_agent};
use crate::security_protection::{validate_command, get_security};

pub struct BasicInteractive {
    pub operations: FenrirOperations,
    pub should_continue: bool,
}

impl BasicInteractive {
    pub fn new(operations: FenrirOperations) -> Self {
        Self {
            operations,
            should_continue: true,
        }
    }

    /// 🎯 MODO INTERATIVO BÁSICO
    pub async fn start_interactive_mode(&mut self) -> Result<()> {
        println!("\n🔥 FENRIR BASIC MODE - O que realmente funciona");
        println!("💎 Sem IA pra não dar merda - comandos diretos");
        println!("🥷 Venz aguardando ordens sem censura");
        println!("🔒 Proteções anti-rosnar ativas");
        println!("🚨 Express confirmation para operações de risco");
        println!("\n🎯 COMANDOS DISPONÍVEIS:");
        println!("   📍 morder <alvo>      - Ataque externo");
        println!("   🔍 rosnar <alvo>     - Scan defensivo (SÓ EXTERNO)");
        println!("   🍽️ devorar <alvo>   - Engenharia reversa");
        println!("   🥷 venz <comando>     - Venz executa sem censura");
        println!("   🔒 security          - Status de segurança");
        println!("   📋 help              - Ajuda básica");
        println!("   🚪 sair              - Sair");
        println!("\n⚠️ REGRAS DO CHEFE:");
        println!("   ✅ ROSNAR só em DISCO EXTERNO FÍSICO");
        println!("   ✅ Venz SEM CENSURA = AGENTE MULTI-USO");
        println!("   ✅ Operações de risco = CONFIRMAÇÃO CHEFE");
        println!("");

        loop {
            if !self.should_continue {
                break;
            }

            print!("🔥 fenrir> ");
            io::stdout().flush()?;

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() { continue; }

                    let parts: Vec<&str> = input.split_whitespace().collect();
                    if parts.is_empty() { continue; }

                    let command = parts[0];
                    let args = &parts[1..];

                    match command.to_lowercase().as_str() {
                        "sair" | "exit" | "quit" => {
                            println!("🐺 FENRIR encerrando. Até a próxima!");
                            break;
                        }
                        "help" => {
                            self.show_help();
                        }
                        "security" => {
                            let security = get_security();
                            security.show_operation_log();
                        }
                        "morder" => {
                            if args.is_empty() {
                                println!("❌ Uso: morder <alvo>");
                                continue;
                            }
                            self.handle_morder(args).await?;
                        }
                        "rosnar" => {
                            if args.is_empty() {
                                println!("❌ Uso: rosnar <alvo-externo>");
                                continue;
                            }
                            self.handle_rosnar(args).await?;
                        }
                        "devorar" => {
                            if args.is_empty() {
                                println!("❌ Uso: devorar <alvo>");
                                continue;
                            }
                            self.handle_devorar(args).await?;
                        }
                        "venz" => {
                            if args.is_empty() {
                                println!("❌ Uso: venz <comando> [alvo]");
                                continue;
                            }
                            self.handle_venz(args).await?;
                        }
                        "debug" => {
                            let security = get_security();
                            security.enable_debug_mode();
                            println!("⚠️ DEBUG MODE ATIVADO - Proteções relaxadas");
                        }
                        _ => {
                            println!("❌ Comando desconhecido: {}", command);
                            println!("💡 Digite 'help' para ver comandos disponíveis");
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Erro na entrada: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// 📍 COMANDO MORDER
    async fn handle_morder(&mut self, args: &[&str]) -> Result<()> {
        let target = args.join(" ");
        println!("🔥 MORDER: {}", target);

        // Venz cuida da invasão
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task("scan", Some(&target)).await?;

        println!("✅ MORDER concluída:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        Ok(())
    }

    /// 🔍 COMANDO ROSNAR - COM PROTEÇÃO
    async fn handle_rosnar(&mut self, args: &[&str]) -> Result<()> {
        let target = args.join(" ");
        println!("🔍 ROSNAR: {}", target);

        // Validar segurança primeiro
        validate_command("rosnar", args)?;

        // Se passou na validação, Venz executa
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task("recon", Some(&target)).await?;

        println!("✅ ROSNAR concluída:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        Ok(())
    }

    /// 🍽️ COMANDO DEVORAR
    async fn handle_devorar(&mut self, args: &[&str]) -> Result<()> {
        let target = args.join(" ");
        println!("🍽️ DEVORAR: {}", target);

        // Venz faz engenharia reversa completa
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task("leak", Some(&target)).await?;

        println!("✅ DEVORAR concluída:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        Ok(())
    }

    /// 🥷 COMANDO VENZ - SEM CENSURA
    async fn handle_venz(&mut self, args: &[&str]) -> Result<()> {
        let command = args[0];
        let target = if args.len() > 1 {
            Some(args[1..].join(" "))
        } else {
            None
        };

        println!("🥷 VENZ: {} {:?}", command, target);

        // Venz executa SEM RESTRIÇÕES
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task(command, target.as_deref()).await?;

        println!("✅ VENZ concluído:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        venz_agent.show_operation_log();

        Ok(())
    }

    /// 📋 AJUDA BÁSICA
    fn show_help(&self) {
        println!("\n📋 FENRIR BASIC HELP");
        println!("{}", "═".repeat(40));

        println!("\n🔥 COMANDOS PRINCIPAIS:");
        println!("   📍 morder <alvo>      - Atque direto ao alvo");
        println!("   🔍 rosnar <alvo>     - Scan de reconhecimento");
        println!("   🍽️ devorar <alvo>   - Engenharia reversa completa");
        println!("   🥷 venz <comando>     - Agente sem censura");

        println!("\n🔒 SEGURANÇA:");
        println!("   🔒 ROSNAR só funciona em ALVOS EXTERNOS FÍSICOS");
        println!("   🔒 MORDER/DEVORAR tem validações básicas");
        println!("   🔒 Venz SEM CENSURA = Sem limites pro CHEFE");

        println!("\n🥷 VENZ - AGENTE MULTI-USO:");
        println!("   scan             - Reconhecimento avançado");
        println!("   blockchain        - Análise crypto/blockchain");
        println!("   leak             - Investigação de vazamentos");
        println!("   investigate      - Investigação completa");

        println!("\n⚠️ PROTEÇÕES:");
        println!("   ✅ Validação de comandos perigosos");
        println!("   ✅ Confirmação para operações de risco");
        println!("   ✅ Log completo de todas as operações");

        println!("\n💎 MODOS ESPECIAIS:");
        println!("   ./fenrir --tarefinha  - Modo garçom (se quiser IA)");
        println!("   ./fenrir --chain      - Modo hierarchy (profissional)");

        println!("{}", "═".repeat(40));
        println!("🎯 CHEFE: Use com sabedoria e responsabilidade!");
    }
}

/// 🔥 FUNÇÃO INTERATIVA GLOBAL
pub async fn start_basic_interactive_mode(operations: FenrirOperations) -> Result<()> {
    let mut interactive = BasicInteractive::new(operations);
    interactive.start_interactive_mode().await
}