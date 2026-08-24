# FinTrack Rust

FinTrack Rust é uma aplicação web fullstack para gestão inteligente de carteiras de investimentos. Permite cadastrar ativos e movimentações, acompanhar patrimônio, rentabilidade, distribuição e metas de alocação. Desenvolvida em Rust com Axum, PostgreSQL, SQLx, Askama, JWT e Docker, com foco em segurança, organização e clareza visual.

## O que o projeto faz

- cadastro e autenticação de pessoas usuárias;
- sessão com JWT armazenada em cookie HttpOnly;
- senhas protegidas com Argon2;
- cadastro, edição e remoção de ativos;
- registro de compras e vendas;
- recálculo automático de quantidade e preço médio;
- bloqueio de venda acima da posição disponível;
- histórico de movimentações;
- cálculo de patrimônio, valor investido, lucro/prejuízo e rentabilidade;
- distribuição da carteira por categoria;
- metas de alocação com comparação entre planejado e atual;
- API local para ativos e analytics;
- dashboard responsivo com identidade Futuristic Finance em preto, azul e dourado.

## Tecnologias

- Rust
- Axum
- Tokio
- PostgreSQL
- SQLx
- Askama
- JWT + cookies
- Argon2
- Docker
- HTML + CSS
- GitHub Actions

## Arquitetura

```text
Browser
  ↓
Axum
  ↓
Autenticação / Handlers
  ↓
Portfolio Engine + Analytics
  ↓
SQLx
  ↓
PostgreSQL
```

Mais detalhes em `docs/ARCHITECTURE.md`.

## Evolução

- `v0.1.0` — Foundation: Cargo, Axum, health check e estrutura inicial.
- `v0.2.0` — Database: PostgreSQL, Docker, SQLx e migrations.
- `v0.3.0` — Assets Core: CRUD de ativos.
- `v0.4.0` — Authentication: usuários, JWT, cookies e Argon2.
- `v0.5.0` — Portfolio Engine: compras, vendas, preço médio e histórico.
- `v0.6.0` — Dashboard: Askama + Futuristic Finance.
- `v0.7.0` — Analytics: distribuição e metas de alocação.
- `v0.8.0` — Quality: testes automatizados e CI.
- `v0.9.0` — Product Polish: UX, responsividade, acessibilidade e documentação.
- `v1.0.0` — versão final para entrega.

Cada etapa principal foi separada em branch própria para tornar a evolução do projeto fácil de acompanhar no GitHub.

## Como executar

### 1. Clone o projeto

```powershell
git clone https://github.com/JhonataFontoura/Projeto-fullstack-rust.git
cd Projeto-fullstack-rust
git switch v1.0.0-final
```

### 2. Crie o arquivo de ambiente

```powershell
Copy-Item .env.example .env
```

Troque o valor de `JWT_SECRET` por um segredo local forte.

### 3. Inicie o PostgreSQL

```powershell
docker compose up -d
```

### 4. Execute a aplicação

```powershell
cargo run
```

Acesse:

```text
http://127.0.0.1:3000
```

## Como testar

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

A branch também possui workflow de CI no GitHub Actions para executar essas verificações automaticamente.

## Melhorias implementadas em relação ao projeto base

O projeto deixou de ser apenas um CRUD de ativos e passou a possuir autenticação, isolamento de carteira por usuário, regras reais de compra e venda, histórico de movimentações, analytics, metas de alocação, testes automatizados e uma identidade visual própria.

## Design

A interface segue o conceito **Futuristic Finance**:

- fundo preto e azul profundo;
- azul tecnológico para ações e interação;
- dourado para patrimônio e informações de destaque;
- verde para ganhos;
- vermelho para perdas;
- cards com hierarquia clara;
- responsividade para desktop e dispositivos menores;
- estados vazios, foco visível e suporte a `prefers-reduced-motion`.

## O que aprendi

Durante o desafio foram praticados organização modular em Rust, ownership aplicado a regras de negócio, `Result`, validação, programação assíncrona com Tokio, rotas com Axum, SQLx e PostgreSQL, migrations, autenticação, cookies, JWT, hashing de senha, templates Askama, Docker, testes e integração contínua.

## Aviso

O FinTrack é um projeto educacional de acompanhamento de carteira. As metas de alocação são definidas pela própria pessoa usuária e não constituem recomendação de investimento.
