# FinTrack Rust

FinTrack Rust é uma aplicação web fullstack para gestão inteligente de carteiras de investimentos. Permite cadastrar ativos e movimentações, acompanhar patrimônio, rentabilidade, distribuição e metas de alocação. Desenvolvida em Rust com Axum, PostgreSQL, SQLx, Askama, JWT e Docker, com foco em segurança e organização.

## Demonstração da aplicação web

O **FinTrack Rust** é uma aplicação Full Stack com backend em **Rust/Axum**, templates **Askama** e persistência em **PostgreSQL**. Diferentemente de um HTML estático, o projeto precisa do servidor e do banco de dados em execução para disponibilizar login, cadastro e dashboard.

**▶️ [Como executar e visualizar o FinTrack no navegador](#como-executar)**

Após iniciar a aplicação localmente, acesse:

- **Aplicação:** [http://127.0.0.1:3000](http://127.0.0.1:3000)
- **Cadastro:** [http://127.0.0.1:3000/register](http://127.0.0.1:3000/register)
- **Login:** [http://127.0.0.1:3000/login](http://127.0.0.1:3000/login)
- **Health check:** [http://127.0.0.1:3000/health](http://127.0.0.1:3000/health)

### Vídeo de apresentação do FinTrack

Gravação de aproximadamente **49 segundos**, preparada para demonstrar a aplicação web em funcionamento.

**Arquivo da demonstração:** `media/fintrack-demo.mp4`

> **Publicação pendente:** o vídeo foi fornecido pelo autor, mas ainda precisa ser enviado para a pasta `media/` deste repositório. Assim que o arquivo estiver publicado, o link abaixo ficará disponível.

[▶ Assistir à demonstração do FinTrack Rust](./media/fintrack-demo.mp4)

Para publicar o vídeo, no GitHub abra este repositório, selecione **Add file → Upload files**, envie o MP4 com o nome `fintrack-demo.mp4` dentro da pasta `media` e confirme o commit na branch `main`. Não é necessário alterar o README novamente se o caminho e o nome forem mantidos.

### O que explorar

1. **Cadastro e autenticação:** crie uma conta e entre no sistema.
2. **Dashboard:** acompanhe ativos e indicadores da carteira.
3. **Gestão de ativos:** cadastre, edite e exclua registros.
4. **Integração com banco de dados:** os dados são armazenados no PostgreSQL.

> **Disponibilidade da demonstração:** o repositório não informa uma URL pública de deploy e não contém um vídeo de demonstração hospedado. Os links `127.0.0.1` funcionam **somente no computador em que o projeto estiver rodando**. Um visualizador de HTML estático não executa a aplicação Rust nem substitui o PostgreSQL. Para disponibilizar um botão “Testar online” para recrutadores, será necessário publicar a aplicação em uma hospedagem e configurar o banco de dados.

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
