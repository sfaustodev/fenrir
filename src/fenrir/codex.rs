// --- FENRIR CODEX MODULE ---
// Integração com o sistema Codex OpenAI para modo interativo ultra-devorador
// O Lobo agora conversa com Codex em tempo real

use std::process::Command;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::process::Command as AsyncCommand;

// Estrutura principal do Fenrir-Codex
#[derive(Debug, Clone)]
pub struct FenrirCodex {
    pub config: CodexConfig,
    pub auth: CodexAuth,
    pub interactive: bool,
}

// Configuração do Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexConfig {
    pub model: String,
    pub reasoning_effort: String,
    pub trust_level: String,
    pub api_endpoint: String,
}

// Autenticação Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexAuth {
    pub openai_api_key: String,
    pub session_id: String,
}

// Histórico de comandos Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexHistory {
    pub session_id: String,
    pub timestamp: u64,
    pub command: String,
    pub response: Option<String>,
}

impl FenrirCodex {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("/Users/peluche/.codex/config.toml");
        let auth_path = Path::new("/Users/peluche/.codex/auth.json");

        // Carregar configuração
        let config_content = std::fs::read_to_string(config_path)?;
        let config: CodexConfig = toml::from_str(&config_content)?;

        // Carregar auth
        let auth_content = std::fs::read_to_string(auth_path)?;
        let auth: CodexAuth = serde_json::from_str(&auth_content)?;

        Ok(FenrirCodex {
            config,
            auth,
            interactive: false,
        })
    }

    // Ativar modo interativo ultra-devorador
    pub fn activate_interactive_mode(&mut self) {
        self.interactive = true;
        println!("🐺 FENRIR CODEX MODE ATIVADO - O Lobo está com fome de código!");
        println!("🔥 OpenAI GPT-5.1 Codex integrado ao sistema");
        println!("⚡ Modo reasoning: {}", self.config.reasoning_effort);
        println!("🎯 Trust level: {}", self.config.trust_level);
    }

    // Executar comando Codex
    pub async fn execute_codex_command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        if !self.interactive {
            return Err("Modo interativo não ativado".into());
        }

        println!("🐺 Fenrir-Codex processando: {}", command);

        // Construir comando codex
        let output = AsyncCommand::new("codex")
            .arg(command)
            .env("OPENAI_API_KEY", &self.auth.openai_api_key)
            .output()
            .await?;

        let response = String::from_utf8_lossy(&output.stdout).to_string();

        if output.stderr.is_empty() {
            Ok(response)
        } else {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("Codex error: {}", error).into())
        }
    }

    // Modo devorador: integrar comandos direto no terminal
    pub fn devour_mode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.activate_interactive_mode();

        println!("\n💀 FENRIR-CODEX DEVOUR MODE");
        println!("Digite comandos normais ou 'codex: <comando>' para IA");
        println!("Ctrl+D para sair\n");

        loop {
            print!("🐺 ");
            use std::io;
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let input = input.trim();

                    if input.starts_with("codex:") {
                        // Comando Codex
                        let codex_cmd = &input[6..].trim();
                        println!("🤖 Processando com Codex: {}", codex_cmd);

                        // Aqui você pode integrar chamadas reais ao Codex
                        match self.execute_codex_sync(codex_cmd) {
                            Ok(response) => println!("🔥 Resposta: {}", response),
                            Err(e) => println!("❌ Erro: {}", e),
                        }
                    } else if !input.is_empty() {
                        // Comando shell normal
                        match Command::new("sh").arg("-c").arg(input).output() {
                            Ok(output) => {
                                print!("{}", String::from_utf8_lossy(&output.stdout));
                                eprint!("{}", String::from_utf8_lossy(&output.stderr));
                            }
                            Err(e) => println!("❌ Erro: {}", e),
                        }
                    }
                }
                Err(e) => println!("❌ Erro: {}", e),
            }
        }

        println!("\n🐺 Fenrir-Codex modo devorador encerrado");
        Ok(())
    }

    // Versão síncrona para modo devorador
    fn execute_codex_sync(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("codex")
            .arg(command)
            .env("OPENAI_API_KEY", &self.auth.openai_api_key)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // Salvar histórico no formato Codex
    pub fn save_to_history(&self, command: &str, response: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let history_path = Path::new("/Users/peluche/.codex/history.jsonl");
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let entry = CodexHistory {
            session_id: self.auth.session_id.clone(),
            timestamp,
            command: command.to_string(),
            response: response.map(|s| s.to_string()),
        };

        let json_line = serde_json::to_string(&entry)?;
        std::fs::write(history_path, format!("{}\n", json_line))?;

        Ok(())
    }
}

// Função principal para invocar o modo Fenrir-Codex
pub fn summon_fenrir_codex() -> Result<(), Box<dyn std::error::Error>> {
    let mut fenrir_codex = FenrirCodex::new()?;

    println!("🌙 SUMMONING FENRIR-CODEX...");
    println!("O Lobo Devorador agora se une ao Codex OpenAI");

    fenrir_codex.devour_mode()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_codex_integration() {
        let codex = FenrirCodex::new().unwrap();
        assert_eq!(codex.config.model, "gpt-5.1-codex");
    }
}