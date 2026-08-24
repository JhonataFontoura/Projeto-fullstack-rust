# Changelog

## v1.0.0 — Final

Versão preparada para entrega do desafio fullstack em Rust.

### Incluído
- autenticação com Argon2, JWT e cookie HttpOnly;
- isolamento de dados por usuário;
- CRUD de ativos;
- Portfolio Engine com compra, venda e preço médio ponderado;
- bloqueio de venda acima da posição disponível;
- histórico de movimentações;
- patrimônio, valor investido, lucro/prejuízo e rentabilidade;
- distribuição da carteira por categoria;
- metas de alocação planejado x atual;
- API local de ativos e analytics;
- PostgreSQL + SQLx + migrations + Docker;
- Askama com interface Futuristic Finance;
- testes unitários e de regras financeiras;
- CI com rustfmt, Clippy e cargo test;
- melhorias de responsividade, foco, estados vazios e acessibilidade;
- documentação de arquitetura e README de entrega.

## v0.9.0 — Product Polish
- refinamento de UX e responsividade;
- foco visível e suporte a `prefers-reduced-motion`;
- documentação de arquitetura.

## v0.8.0 — Quality
- biblioteca exportando regras testáveis;
- testes de compra e venda;
- workflow de CI com format, Clippy e testes.

## v0.7.0 — Portfolio Intelligence
- distribuição por categoria;
- metas de alocação;
- comparação entre percentual atual e planejado;
- endpoint `/api/analytics`.

## v0.6.0 — Futuristic Finance Dashboard
- consolidação do dashboard Askama;
- identidade visual preto, azul e dourado.

## v0.5.0 — Portfolio Engine
- compras e vendas;
- recálculo de preço médio;
- histórico de movimentações;
- transação SQL atômica para posição + histórico.

## v0.4.0 — Authentication
- cadastro e login;
- Argon2;
- JWT + cookies;
- carteira isolada por usuário.

## v0.3.0 — Assets Core
- CRUD de ativos e validações iniciais.

## v0.2.0 — Database
- PostgreSQL, SQLx, Docker e migrations.

## v0.1.0 — Foundation
- Cargo, Tokio, Axum e health check.
