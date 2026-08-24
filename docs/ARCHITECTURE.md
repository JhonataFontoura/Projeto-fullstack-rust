# Arquitetura do FinTrack Rust

## Fluxo principal

```text
Browser
  ↓
Axum Router
  ↓
Handlers / regras de aplicação
  ↓
Services de carteira e analytics
  ↓
SQLx
  ↓
PostgreSQL
```

## Camadas

- **Web:** Axum, formulários, cookies e respostas HTML.
- **Templates:** Askama para renderização server-side.
- **Autenticação:** JWT em cookie HttpOnly e senhas com Argon2.
- **Portfolio Engine:** regras de compra, venda, quantidade e preço médio.
- **Analytics:** patrimônio, rentabilidade, distribuição por categoria e comparação com metas.
- **Persistência:** PostgreSQL com migrations SQLx.
- **Infraestrutura:** Docker Compose para banco local.

## Decisões importantes

1. O banco mantém ativos separados por `user_id`.
2. Compras e vendas atualizam posição e histórico dentro da mesma transação SQL.
3. Venda acima da posição atual é rejeitada.
4. Metas de alocação são definidas pelo próprio usuário e não representam recomendação de investimento.
5. O frontend segue a identidade Futuristic Finance: preto, azul e dourado, priorizando legibilidade e hierarquia.
