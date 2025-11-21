#!/bin/bash

# 🔥 FENRIR TRINITY IA - TESTE DE API GROK
# Script para testar se a API key do Grok está funcionando

echo "🔴🔴🔴 FENRIR TRINITY IA - TESTE DE API GROK 🔴🔴🔴"
echo "Testando conectividade com Grok 4.1 Fast API..."
echo ""

# Verificar se GLI_KEY está configurada
if [ -z "$GLI_KEY" ]; then
    echo "❌ GLI_KEY não está configurada!"
    echo ""
    echo "💡 Para configurar:"
    echo "   export GLI_KEY='sua_api_key_aqui'"
    echo ""
    echo "🔑 Ou adicione permanentemente ao ~/.zshrc:"
    echo "   echo 'export GLI_KEY=\"sua_api_key_aqui\"' >> ~/.zshrc"
    echo "   source ~/.zshrc"
    echo ""
    exit 1
fi

echo "✅ GLI_KEY encontrada"
echo "🔑 API Key: ${GLI_KEY:0:10}...${GLI_KEY: -10}"
echo ""

# Testar API com curl
echo "🚀 Testando API Grok 4.1 Fast..."
echo ""

API_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
  https://api.x.ai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $GLI_KEY" \
  -d '{
    "model": "grok-4.1-fast",
    "messages": [
      {
        "role": "user",
        "content": "FENRIR TEST - Responda apenas: GOD_MODE_ACTIVATED"
      }
    ],
    "max_tokens": 10,
    "temperature": 0
  }')

HTTP_CODE=$(echo "$API_RESPONSE" | tail -n1)
RESPONSE_BODY=$(echo "$API_RESPONSE" | head -n -1)

echo "📊 Status HTTP: $HTTP_CODE"

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ API GROK FUNCIONANDO!"
    echo ""
    echo "🤖 Resposta da API:"
    echo "$RESPONSE_BODY" | jq -r '.choices[0].message.content' 2>/dev/null || echo "$RESPONSE_BODY"
    echo ""
    echo "🔥 FENRIR TRINITY IA PRONTO PARA USAR!"
    echo "   Execute: ./target/release/fenrir --trinity"
else
    echo "❌ ERRO NA API GROK!"
    echo ""
    echo "📋 Detalhes do erro:"
    echo "$RESPONSE_BODY" | jq -r '.error.message' 2>/dev/null || echo "$RESPONSE_BODY"
    echo ""
    echo "💡 Possíveis soluções:"
    echo "   1. Verifique se a API key está correta"
    echo "   2. Verifique se a API key tem créditos"
    echo "   3. Verifique sua conexão com a internet"
    echo "   4. Verifique se o modelo grok-4.1-fast está disponível"
fi

echo ""
echo "🔥 FIM DO TESTE 🔥"