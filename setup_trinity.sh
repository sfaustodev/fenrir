#!/bin/bash

# 🔥 FENRIR TRINITY IA - SETUP SCRIPT
# Configuração do ambiente para modo Trinity IA

echo "🔴🔴🔴 FENRIR TRINITY IA - SETUP 🔴🔴🔴"
echo "Configurando ambiente para coordenação Gemini + Claude + Grok"
echo ""

# Verificar se GLI_KEY já está configurada
if [ -n "$GLI_KEY" ]; then
    echo "✅ GLI_KEY já está configurada"
    echo "🚀 Grok 4.1 Fast pronto para uso"
else
    echo "❌ GLI_KEY não encontrada"
    echo ""
    echo "💡 Para configurar a API key do Grok:"
    echo "   export GLI_KEY='sua_api_key_aqui'"
    echo ""
    echo "🔑 Ou adicione ao seu ~/.zshrc ou ~/.bashrc:"
    echo "   export GLI_KEY='sua_api_key_aqui'"
    echo ""
    echo "⚠️ Execute 'source ~/.zshrc' após configurar"
fi

echo ""
echo "🐺 Modos disponíveis:"
echo "   ./target/release/fenrir              - Modo GOD MODE padrão"
echo "   ./target/release/fenrir --trinity    - Modo Trinity IA (Chain of Thoughts)"
echo ""
echo "🧠 Trinity IA features:"
echo "   ✅ Coordenação Gemini + Claude + Grok"
echo "   ✅ Chain of Thoughts completo"
echo "   ✅ Sistema de consenso entre IAs"
echo "   ✅ Particionamento automático de tarefas"
echo "   ✅ Aprovação final FENRIR GOD MODE"
echo ""

# Testar se o binário existe
if [ -f "./target/release/fenrir" ]; then
    echo "✅ FENRIR Trinity compilado e pronto"
else
    echo "❌ FENRIR não encontrado"
    echo "💡 Execute: cargo build --release"
fi

echo ""
echo "🔥 FENRIR TRINITY IA - PRONTO PARA AÇÃO! 🔥"