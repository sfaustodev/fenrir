// --- ARQUIVOS DE MÓDULO ---
// FENRIR GOD MODE - Sistema operacional completo
mod executor;
mod oraculo;
mod ferramentas;
mod terminal;
mod starship;
mod config;
mod operations;
mod grok_coordinator;
mod interactive_trinity;
mod multi_ai_coordinator;
mod venice_client;
mod task_management;
mod security_protection;
mod venz_agent;
mod basic_interactive;

// --- IMPORTS (use) ---
// Agora a gente chama as funções dos *nossos* módulos.



use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use terminal::{bootstrap_terminal_interface, detect_terminal_capabilities};
use executor::{ask_for_confirmation, handle_execute_command, handle_open_editor, log_task, FenrirTask};
use starship::{initialize_fenrir_starship, FenrirStarship};
use config::FenrirConfig;
use operations::FenrirOperations;
use interactive_trinity::InteractiveTrinity;
use multi_ai_coordinator::MultiAICoordinator;
use venice_client::VeniceClient;
use task_management::{chain_coordinator::ChainOfCaralhoManager, tarefinha_mode::TarefinhaMode};
use basic_interactive::start_basic_interactive_mode;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pb = ProgressBar::new_spinner(); // Spinner pra gente ver rodando

    // 🔴 FENRIR GOD MODE - Inicialização automática
    let mut fenrir_config = match FenrirConfig::load() {
        Ok(config) => {
            println!("🔴 FENRIR GOD MODE - Configuração carregada!");
            config
        }
        Err(e) => {
            println!("⚠️ Erro ao carregar config FENRIR: {}", e);
            println!("🚀 Usando configuração padrão GOD MODE!");
            FenrirConfig::default()
        }
    };

    // Ativar GOD Mode automaticamente se configurado
    if fenrir_config.should_activate_god_mode_automatically() {
        println!("🔴🔴🔴 FENRIR GOD MODE ATIVADO AUTOMATICAMENTE 🔴🔴🔴");
        println!("💀 PODERES DIVINOS CONCEDIDOS AO LOBO DEVORADOR!");
    }

    // Inicializar operações táticas
    let mut fenrir_ops = match FenrirOperations::new() {
        Ok(ops) => {
            println!("🚀 Operações táticas FENRIR inicializadas!");
            ops
        }
        Err(e) => {
            println!("⚠️ Erro ao inicializar operações: {}", e);
            return;
        }
    };

    // Detectar capabilities do terminal
    let (has_ghostty, _has_colors, _has_unicode) = detect_terminal_capabilities();

    // Inicializar Fenrir-Starship
    let mut fenrir_starship = initialize_fenrir_starship();

    if has_ghostty {
        println!("🐺 Ghostty + Starship detectados! Inicializando interface divina...");
    } else {
        println!("⚠️  Ghostty não encontrado. Use 'brew install --cask ghostty' para experiência completa.");
        println!("🌟 Starship Fenrir carregado mesmo assim!");
    }

    // Inicializar Sistema Multi-IA
    let multi_ai = match MultiAICoordinator::new() {
        Ok(coordinator) => {
            println!("🔴 SISTEMA MULTI-IA FENRIR INICIALIZADO");
            Some(coordinator)
        }
        Err(e) => {
            println!("⚠️ Erro ao inicializar Multi-IA: {}", e);
            println!("🚀 Continuando sem coordenação multi-IA...");
            None
        }
    };

    // Inicializar Cliente Venice (Trabalho Sujo)
    let venice = match VeniceClient::new() {
        Ok(client) => {
            println!("💀 Cliente Venice (Trabalho Sujo) PRONTO");
            Some(client)
        }
        Err(e) => {
            println!("⚠️ Erro ao inicializar Venice: {}", e);
            println!("💀 Operações sujas serão limitadas...");
            None
        }
    };

    if args.len() > 1 && args[1] == "--multi-ia" {
        // Modo Multi-IA - Hierarquia completa
        println!("🔴🔴🔴 FENRIR MULTI-IA - HIERARQUIA COMPLETA 🔴🔴🔴");
        if let Some(coordinator) = multi_ai {
            if let Err(e) = start_multi_ia_mode(coordinator, venice).await {
                eprintln!("❌ Erro no modo Multi-IA: {}", e);
            }
        } else {
            eprintln!("❌ Sistema Multi-IA não disponível");
        }
        return;
    } else if args.len() > 1 && args[1] == "--chain" {
        // Modo Chain-of-Caralho - Sistema de tarefinhas
        println!("🔥🔥🔥 FENRIR CHAIN-OF-CARALHO - SISTEMA HIERÁRQUICO 🔥🔥🔥");
        let mut chain = ChainOfCaralhoManager::new();
        if let Err(e) = start_chain_mode(&mut chain).await {
            eprintln!("❌ Erro no modo Chain: {}", e);
        }
    } else if args.len() > 1 && args[1] == "--tarefinha" {
        // Modo Tarefinha - Garçom Claudão
        println!("🎯🍽️ FENRIR TAREFINHA MODE - GARÇOM CLAUDÃO 🍽️🎯");
        let mut tarefinha_mode = TarefinhaMode::new();
        if let Err(e) = tarefinha_mode.start_interactive_mode().await {
            eprintln!("❌ Erro no modo Tarefinha: {}", e);
        }
    } else if args.len() > 1 && args[1] == "--trinity" {
        // Modo "um comando e vaza"
        let consulta_completa = args[1..].join(" ");
        processar_solicitacao(&consulta_completa, &pb, &mut fenrir_ops).await;
    } else {
        // MODO PADRÃO - BÁSICO QUE FUNCIONA SEM MERDA
        println!("🔥 FENRIR BASIC MODE - O que realmente funciona");
        println!("💀 Sem IA pra não dar merda - comandos diretos");
        println!("🥷 Venz aguardando ordens sem censura");
        println!("🔒 Proteções anti-rosnar ativas");

        pb.finish_with_message("FENRIR BASIC READY!");

        // Iniciar modo interativo básico que funciona
        if let Err(e) = start_basic_interactive_mode(fenrir_ops).await {
            eprintln!("❌ Erro no modo interativo: {}", e);
        }
    }
}

// Modo interativo com interface Ghostty + Starship avançada
async fn interativo(pb: &ProgressBar, fenrir_terminal: &terminal::FenrirTerminal, fenrir_starship: &mut FenrirStarship, fenrir_ops: &mut FenrirOperations) {
    let stdin = io::stdin();
    let mut input_buffer = String::new();
    let mut last_command_status = 0;

    loop {
        // Atualizar contexto do Starship
        fenrir_starship.update_context();

        // Renderizar prompt Starship personalizado
        let prompt = fenrir_starship.render_for_terminal(fenrir_terminal.ghostty_available, last_command_status);
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        input_buffer.clear();
        match stdin.read_line(&mut input_buffer) {
            Ok(0) => break, // Fim da entrada (Ctrl+D)
            Ok(_) => {
                let trimado = input_buffer.trim().to_lowercase();
                if trimado.is_empty() {
                    continue;
                }
                if trimado == "sair" || trimado == "exit" || trimado == "quit" {
                    println!("\n🐺 Falou, parceiro! O Lobo está descansando...");
                    break;
                }
                if trimado == "ghostty" {
                    println!("\n🎯 Ghostty Status: {}",
                        if fenrir_terminal.ghostty_available { "ATIVO ✅" } else { "NÃO DISPONÍVEL ❌" });
                    last_command_status = 0;
                    continue;
                }
                if trimado == "status" {
                    println!("\n📊 STATUS DO FENRIR-STARSHIP:");
                    println!("   🐺 Interface: Ghostty {}",
                        if fenrir_terminal.ghostty_available { "✅" } else { "❌" });
                    println!("   🌟 Starship: ATIVO ✅");
                    println!("   🎨 Tema: {}", fenrir_terminal.config.theme);
                    println!("   🔤 Fonte: {} ({:.1}px)",
                        fenrir_terminal.config.font_family,
                        fenrir_terminal.config.font_size);
                    last_command_status = 0;
                    continue;
                }
                if trimado == "starship" {
                    println!("\n🌟 FENRIR-STARSHIP CONFIGURATION:");
                    println!("   🎯 Formato: {}", fenrir_starship.config.format);
                    println!("   📦 Módulos: {:?}", fenrir_starship.config.modules);
                    println!("   🐺 Símbolo Fenrir: {}", fenrir_starship.config.fenrir.symbol);
                    last_command_status = 0;
                    continue;
                }
                if trimado == "godmode" {
                    println!("\n🔴 FENRIR GOD MODE ATIVADO!");
                    println!("💀 Poders divinos concedidos ao Lobo Devorador!");
                    last_command_status = 0;
                    continue;
                }

                // 🐺 COMANDOS FENRIR GOD MODE - OPERAÇÕES TÁTICAS
                if trimado.starts_with("rosnar") {
                    println!("\n🐺💀 FENRIR ROSNANDO - MODO ANTIVÍRUS EVOLUTIVO!");
                    println!("🔥 O Lobo está farejando ameaças internas... PREPARANDO DEFESA!");

                    // Extrair alvo opcional
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    let alvo = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };

                    if let Some(ref target) = alvo {
                        println!("🎯 ALVO ESPECÍFICO DETECTADO: {}", target);
                    } else {
                        println!("🔍 ESCANEAMENTO COMPLETO - BUSCANDO AMEAÇAS INTERNA");
                    }

                    println!("⚡ INICIANDO OPERAÇÃO ROSNAR...");

                    // Executar operação ROSNAR
                    match fenrir_ops.execute_rosnar(alvo.as_deref()).await {
                        Ok(_) => {
                            println!("✅ FENRIR ROSNADO - Sistema limpo e monitorado!");
                            last_command_status = 0;
                        }
                        Err(e) => {
                            eprintln!("❌ ERRO NA OPERAÇÃO ROSNAR: {}", e);
                            last_command_status = 1;
                        }
                    }
                    continue;
                }

                if trimado.starts_with("morder") {
                    println!("\n💀🔥 FENRIR MORDENDO - MODO OFENSIVO EXTERNO!");
                    println!("🔥 O Lobo está preparando ataque brutal... ALVO EXTERNO!");

                    // Extrair alvo obrigatório para MORDER
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    let alvo = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };

                    if let Some(ref target) = alvo {
                        println!("🎯 ALVO EXTERNO SELECIONADO: {}", target);
                        println!("⚡ INICIANDO OPERAÇÃO MORDER...");

                        // Executar operação MORDER
                        match fenrir_ops.execute_morder(&target).await {
                            Ok(_) => {
                                println!("💀 FENRIR MORDIDO - {} devastado!", target);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("❌ ERRO NA OPERAÇÃO MORDER: {}", e);
                                last_command_status = 1;
                            }
                        }
                    } else {
                        eprintln!("❌ MORDER exige um alvo externo! Ex: 'morder bitcoin2000'");
                        last_command_status = 1;
                    }
                    continue;
                }

                if trimado.starts_with("devorar") {
                    println!("\n💀🔥 FENRIR DEVORANDO - ENGENHARIA REVERSA COMPLETA!");
                    println!("🔥 O Lobo vai devorar e recriar o alvo em RUST!");

                    // Extrair alvo obrigatório para DEVORAR
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    let alvo = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };

                    if let Some(ref target) = alvo {
                        println!("🎯 ALVO PARA DEVORAÇÃO: {}", target);
                        println!("⚡ INICIANDO OPERAÇÃO DEVORAR...");

                        // Executar operação DEVORAR
                        match fenrir_ops.execute_devorar(&target).await {
                            Ok(_) => {
                                println!("💀 FENRIR DEVORADO - {} dominado e recriado em Rust!", target);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("❌ ERRO NA OPERAÇÃO DEVORAR: {}", e);
                                last_command_status = 1;
                            }
                        }
                    } else {
                        eprintln!("❌ DEVORAR exige um alvo! Ex: 'devorar explorer.exe'");
                        last_command_status = 1;
                    }
                    continue;
                }

                // Limpar área de entrada antes de processar
                let _ = fenrir_terminal.clear_input_area();

                // Se não for comando especial, é pro Oráculo!
                processar_solicitacao(&trimado, pb, fenrir_ops).await;

                // Simular status do comando (no mundo real, viria do comando executado)
                last_command_status = 0; // Sucesso

                // Pausa antes do próximo prompt
                println!("\n⏳ Pressione Enter para continuar...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            Err(e) => {
                eprintln!("❌ Erro lendo entrada: {}", e);
                last_command_status = 1; // Erro
                break;
            }
        }
    }
}

// Modo interativo fallback quando Ghostty falha (mas Starship funciona!)
async fn interativo_fallback(pb: &ProgressBar, fenrir_starship: &mut FenrirStarship, fenrir_ops: &mut FenrirOperations) {
    let stdin = io::stdin();
    let mut input_buffer = String::new();
    let mut last_command_status = 0;

    println!("🌟 Iniciando modo Starship-only...");

    loop {
        // Atualizar contexto do Starship
        fenrir_starship.update_context();

        // Renderizar prompt Starship (sem terminal Ghostty)
        let prompt = fenrir_starship.render_prompt(last_command_status);
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        input_buffer.clear();
        match stdin.read_line(&mut input_buffer) {
            Ok(0) => break,
            Ok(_) => {
                let trimado = input_buffer.trim().to_lowercase();
                if trimado.is_empty() {
                    continue;
                }
                if trimado == "sair" || trimado == "exit" {
                    println!("\n🐺 Falou, parceiro! O Lobo está descansando...");
                    break;
                }
                if trimado == "starship" {
                    println!("\n🌟 FENRIR-STARSHIP MODO FALLBACK:");
                    println!("   ✅ Starship: ATIVO (modo standalone)");
                    println!("   ❌ Ghostty: NÃO DISPONÍVEL");
                    println!("   🐺 Modo: Fenrir-Starship puro");
                    last_command_status = 0;
                    continue;
                }
                if trimado == "godmode" {
                    println!("\n🔴 FENRIR-STARSHIP GOD MODE!");
                    println!("💀 Poderes do Starship intensificados!");
                    last_command_status = 0;
                    continue;
                }

                // 🐺 COMANDOS FENRIR GOD MODE - OPERAÇÕES TÁTICAS (Fallback)
                if trimado.starts_with("rosnar") {
                    println!("\n🐺💀 FENRIR ROSNANDO - MODO ANTIVÍRUS EVOLUTIVO!");
                    println!("🔥 O Lobo está farejando ameaças internas... PREPARANDO DEFESA!");

                    // Extrair alvo opcional
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    let alvo = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };

                    if let Some(ref target) = alvo {
                        println!("🎯 ALVO ESPECÍFICO DETECTADO: {}", target);
                    } else {
                        println!("🔍 ESCANEAMENTO COMPLETO - BUSCANDO AMEAÇAS INTERNA");
                    }

                    println!("⚡ INICIANDO OPERAÇÃO ROSNAR...");

                    // Executar operação ROSNAR
                    match fenrir_ops.execute_rosnar(alvo.as_deref()).await {
                        Ok(_) => {
                            println!("✅ FENRIR ROSNADO - Sistema limpo e monitorado!");
                            last_command_status = 0;
                        }
                        Err(e) => {
                            eprintln!("❌ ERRO NA OPERAÇÃO ROSNAR: {}", e);
                            last_command_status = 1;
                        }
                    }
                    continue;
                }

                if trimado.starts_with("morder") {
                    println!("\n💀🔥 FENRIR MORDENDO - MODO OFENSIVO EXTERNO!");
                    println!("🔥 O Lobo está preparando ataque brutal... ALVO EXTERNO!");

                    // Extrair alvo obrigatório para MORDER
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    let alvo = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };

                    if let Some(ref target) = alvo {
                        println!("🎯 ALVO EXTERNO SELECIONADO: {}", target);
                        println!("⚡ INICIANDO OPERAÇÃO MORDER...");

                        // Executar operação MORDER
                        match fenrir_ops.execute_morder(&target).await {
                            Ok(_) => {
                                println!("💀 FENRIR MORDIDO - {} devastado!", target);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("❌ ERRO NA OPERAÇÃO MORDER: {}", e);
                                last_command_status = 1;
                            }
                        }
                    } else {
                        eprintln!("❌ MORDER exige um alvo externo! Ex: 'morder bitcoin2000'");
                        last_command_status = 1;
                    }
                    continue;
                }

                if trimado.starts_with("devorar") {
                    println!("\n💀🔥 FENRIR DEVORANDO - ENGENHARIA REVERSA COMPLETA!");
                    println!("🔥 O Lobo vai devorar e recriar o alvo em RUST!");

                    // Extrair alvo obrigatório para DEVORAR
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    let alvo = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };

                    if let Some(ref target) = alvo {
                        println!("🎯 ALVO PARA DEVORAÇÃO: {}", target);
                        println!("⚡ INICIANDO OPERAÇÃO DEVORAR...");

                        // Executar operação DEVORAR
                        match fenrir_ops.execute_devorar(&target).await {
                            Ok(_) => {
                                println!("💀 FENRIR DEVORADO - {} dominado e recriado em Rust!", target);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("❌ ERRO NA OPERAÇÃO DEVORAR: {}", e);
                                last_command_status = 1;
                            }
                        }
                    } else {
                        eprintln!("❌ DEVORAR exige um alvo! Ex: 'devorar explorer.exe'");
                        last_command_status = 1;
                    }
                    continue;
                }

                processar_solicitacao(&trimado, pb, fenrir_ops).await;
                last_command_status = 0; // Sucesso simulado
            }
            Err(e) => {
                eprintln!("❌ Erro lendo entrada: {}", e);
                last_command_status = 1; // Erro
                break;
            }
        }
    }
}

// --- O CÉREBRO DO FENRIR ---
// O main.rs agora só "orquestra".
// Ele chama o Oráculo, depois chama o Executor.
async fn processar_solicitacao(consulta: &str, pb: &ProgressBar, fenrir_ops: &mut FenrirOperations) {
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["VAI", "CORNO!", "PENSE", "DESGRAÇA!", "...", "VAI", "LOGO", "CARALHO!", "(ノ°Д°）ノ", "┻━┻", "...", "VAI", "CORNO!"])
            .template("{spinner:.bold.yellow} {msg}")
            .unwrap(),
    );
    pb.set_message("Chamando o Oráculo (Gemini)...");
    pb.enable_steady_tick(Duration::from_millis(150));

    // 1. CHAMA O ORÁCULO (que agora tá em 'src/oraculo.rs')
    match oraculo::chamar_gemini_com_timeout(consulta).await {
        Ok(task) => {
            // Oráculo respondeu!
            pb.finish_with_message("! Oráculo respondeu!");

            // 2. CHAMA O EXECUTOR (log_task)
            if let Err(e) = executor::log_task(&task) {
                eprintln!("Xii, deu erro pra logar a tarefa: {}", e);
            }

            // 3. CHAMA O EXECUTOR (Freio de Mão)
            let acao_proposta = format!(
                "O Oráculo sugeriu: '{}' \nTipo: '{}' \nComando: '{}' \nArquivo: '{}'",
                task.ia_explanation,
                task.task_type,
                task.command_to_run.as_deref().unwrap_or("N/A"),
                task.target_path.as_deref().unwrap_or("N/A")
            );

            println!("\n--- PROPOSTA DO ORÁCULO ---");
            println!("{}", acao_proposta);
            println!("-----------------------------");

            let confirmacao = executor::ask_for_confirmation("Executar comando? (s/n):").await;

            if confirmacao {
                println!("Ok, segurando o volante...");

                // 4. CHAMA O EXECUTOR (As "Mãos")
                match task.task_type.as_str() {
                    "execute_command" => {
                        if let Some(cmd) = task.command_to_run {
                            executor::handle_execute_command(&cmd);
                        } else {
                            eprintln!("Erro: Oráculo mandou 'execute_command' mas não mandou o comando!");
                        }
                    }
                    "open_editor" => {
                        if let (Some(path), Some(app)) = (task.target_path, task.application) {
                            executor::handle_open_editor(&app, &path);
                        } else {
                            eprintln!("Erro: Oráculo mandou 'open_editor' mas faltou o app ou o arquivo!");
                        }
                    }
                    "unknown" | _ => {
                        println!("O Oráculo não entendeu o que fazer. (Disse: '{}')", task.ia_explanation);
                    }
                }
            } else {
                println!("Ação cancelada. Sabonetou!");
            }
        }
        Err(e) => {
            // Deu ruim no Oráculo
            pb.finish_with_message("! DEU RUIM!");
            eprintln!("Ops! Deu ruim na comunicação com o Oráculo: {}", e);
        }
    }
}

/// 🚀 MODO MULTI-IA - Hierarquia completa de IAs
async fn start_multi_ia_mode(
    coordinator: MultiAICoordinator,
    venice: Option<VeniceClient>
) -> anyhow::Result<()> {
    println!("\n🔴🔴🔴 FENRIR MULTI-IA - MODO HIERÁRQUICO ATIVO 🔴🔴🔴");
    println!("💀 Hierarquia: MENTE(Gemini) → CÉREBRO(Claude) → PENSAMENTO(GPT-4) → MÃOS(Venice)");
    println!("🚀 Digite comandos ou 'sair' para encerrar");
    println!("");

    // Mostrar status inicial
    coordinator.show_status();
    if let Some(ref v) = venice {
        v.show_status();
    }

    loop {
        print!("🧠🧠💭💀 Multi-IA> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() { continue; }

                match input.to_lowercase().as_str() {
                    "sair" | "exit" | "quit" => {
                        println!("\n🐺 Multi-IA FENRIR encerrando... Hierarquia desligada.");
                        break;
                    }
                    "status" => {
                        coordinator.show_status();
                        if let Some(ref v) = venice {
                            v.show_status();
                        }
                        continue;
                    }
                    "emergencia" => {
                        println!("🚨 MODO EMERGÊNCIA - PULANDO HIERARQUIA!");
                        if let Some(ref v) = venice {
                            let cmds = v.generate_recon_commands("emergency_target").await?;
                            for cmd in cmds {
                                println!("💀 {}", cmd);
                            }
                        }
                        continue;
                    }
                    _ => {
                        // Processar através da hierarquia completa
                        println!("\n🔥 PROCESSANDO: {}", input);

                        let result = coordinator.process_complete_task(input).await?;

                        if let Some(ref v) = venice {
                            println!("\n💀 VENICE: EXECUTANDO TRABALHO SUJO...");
                            let dirty_task = venice_client::DirtyTask {
                                task_id: format!("fenrir_{}", std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs()),
                                task_type: venice_client::DirtyTaskType::Reconnaissance,
                                target: "multi_ia_target".to_string(),
                                parameters: result.artifacts.clone(),
                                urgency_level: 8,
                            };

                            match v.execute_dirty_task(dirty_task).await {
                                Ok(exec_result) => {
                                    println!("✅ TRABALHO SUJO CONCLUÍDO");
                                    println!("📁 Comandos executados: {}", exec_result.commands.len());

                                    // Gerar relatório final
                                    let report = v.generate_dirty_report(vec![exec_result]).await?;
                                    println!("\n📊 {}", report);
                                }
                                Err(e) => {
                                    println!("❌ Erro no trabalho sujo: {}", e);
                                }
                            }
                        }

                        println!("\n🔥 OPERAÇÃO MULTI-IA CONCLUÍDA!\n");
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Erro: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// 🔥 MODO CHAIN-OF-CARALHO - Sistema hierárquico completo
async fn start_chain_mode(chain: &mut ChainOfCaralhoManager) -> anyhow::Result<()> {
    println!("\n🔥🔥🔥 FENRIR CHAIN-OF-CARALHO - MODO HIERÁRQUICO 🔥🔥🔥");
    println!("👥 Team: Claudao(Senior) + Venz(Pleno) + Geminho(Junior)");
    println!("🎯 Sistema: Um commit por tarefinha, revisão obrigatória");
    println!("🚀 Digite comandos ou 'sair' para encerrar");
    println!("");

    // Mostrar status inicial
    chain.show_dashboard();

    loop {
        print!("🔗 Chain-of-Caralho> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() { continue; }

                match input.to_lowercase().as_str() {
                    "sair" | "exit" | "quit" => {
                        println!("\n🔥 Chain-of-Caralho encerrando... Hierarquia desligada.");
                        break;
                    }
                    "status" => {
                        chain.show_dashboard();
                        continue;
                    }
                    "team" => {
                        show_team_info();
                        continue;
                    }
                    _ if input.starts_with("executar ") => {
                        let goal = input.strip_prefix("executar ").unwrap_or("");
                        println!("\n🎯 OBJETIVO: {}", goal);

                        let batch_id = chain.create_batch_from_goal(goal.to_string())?;
                        chain.process_batch(&batch_id).await?;

                        println!("\n✅ BATCH CONCLUÍDO COM SUCESSO!");
                        continue;
                    }
                    _ => {
                        // Se não for comando, tratar como objetivo
                        println!("\n🎯 PROCESSANDO OBJETIVO: {}", input);

                        let batch_id = chain.create_batch_from_goal(input.to_string())?;
                        chain.process_batch(&batch_id).await?;

                        println!("\n✅ OBJETIVO CONCLUÍDO!");
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Erro: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// 👥 MOSTRAR INFORMAÇÕES DO TEAM
fn show_team_info() {
    println!("\n👥 FENRIR DREAM TEAM");
    println!("{}", "═".repeat(40));
    println!("👨‍💻 CLAUDÃO (Senior):");
    println!("   🎯 Role: Arquiteto e revisor principal");
    println!("   ⭐ Score: 95% aprovação");
    println!("   🔥 Specialties: Complex systems, code review");
    println!("   💬 Quote: \"A arquitetura correta resolve 90% dos problemas\"");

    println!("\n🥷 VENZ (Pleno):");
    println!("   🎯 Role: Hacker e implementador rápido");
    println!("   ⭐ Score: 80% aprovação");
    println!("   🔥 Specialties: Scripts rápidos, automação, pentest");
    println!("   💬 Quote: \"Funciona? Funciona. Está bonito? Não importa.\"");

    println!("\n🧑‍💻 GEMINHO (Junior):");
    println!("   🎯 Role: Desenvolvedor aprendiz");
    println!("   ⭐ Score: 60% aprovação (melhorando!)");
    println!("   🔥 Specialties: Queries, HTML/CSS, documentação excessiva");
    println!("   💬 Quote: \"Um dia vou ser senior! Por enquanto, vou ler mais docs...\"");

    println!("\n🔗 Chain-of-Caralho Workflow:");
    println!("   1. Claudão quebra objetivo em tarefinhas");
    println!("   2. Delega para nível apropriado");
    println!("   3. Cada tarefinha = 1 commit individual");
    println!("   4. Claudão revisa TUDO");
    println!("   5. Feedback e melhorias contínuas");
    println!("   6. Zero bugs na produção (teoricamente)");
    println!("");
}