# FinTrack Rust

FinTrack Rust é uma aplicação web fullstack para gestão inteligente de carteiras de investimentos. Permite cadastrar ativos e movimentações, acompanhar patrimônio, rentabilidade, distribuição e metas de alocação. Desenvolvida em Rust com Axum, PostgreSQL, SQLx, Askama, JWT e Docker, com foco em segurança e organização.

## Visão do projeto

O objetivo é evoluir do CRUD básico para uma experiência de portfólio mais completa, com regras financeiras, autenticação, histórico de movimentações, metas de alocação, testes e uma interface Futuristic Finance em preto, azul e dourado.

## Tecnologias

- Rust
- Axum
- Tokio
- PostgreSQL
- SQLx
- Askama
- JWT + cookies
- Docker
- HTML + CSS

## Roadmap de evolução

- v0.1.0 — Foundation: Cargo, Axum, health check e estrutura inicial.
- v0.2.0 — Database: PostgreSQL, Docker, SQLx e migrations.
- v0.3.0 — Assets Core: criação, listagem e atualização de ativos.
- v0.4.0 — Authentication: usuários, JWT, cookies e rotas protegidas.
- v0.5.0 — Portfolio Engine: compras, vendas, preço médio e patrimônio.
- v0.6.0 — Futuristic Finance UI: dashboard Askama, preto + azul + dourado.
- v0.7.0 — Portfolio Intelligence: rentabilidade, distribuição e metas.
- v0.8.0 — Quality: validações, erros e testes.
- v0.9.0 — Product Polish: UX, responsividade e documentação.
- v1.0.0 — Release final do desafio.

## Estrutura

```text
src/
├── main.rs
├── config/
├── models.rs
├── services.rs
├── routes/
├── handlers/
├── repositories/
├── auth/
└── errors/

templates/
static/
migrations/
tests/
docker-compose.yml
Cargo.toml
```

## Como executar

1. Copie as variáveis de ambiente:

```powershell
Copy-Item .env.example .env
```

2. Suba o PostgreSQL:

```powershell
docker compose up -d
```

3. Execute a aplicação:

```powershell
cargo run
```

4. Abra:

```text
http://127.0.0.1:3000
```

## Testes

```powershell
cargo test
```

## Melhorias implementadas

- dashboard de portfólio;
- cálculo de total investido, patrimônio e resultado;
- categorias de ativos;
- base para metas de alocação;
- persistência em PostgreSQL;
- interface responsiva com identidade visual Futuristic Finance;
- validação de dados e tratamento de erros.

## Aprendizados

O projeto exercita organização modular em Rust, Axum, SQLx, PostgreSQL, Docker, templates Askama, regras de negócio financeiras, serialização, tratamento de erros e desenvolvimento fullstack.
