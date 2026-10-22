# FinTrack Rust

![Rust](https://img.shields.io/badge/Rust-Backend-000000?logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-Web-2563EB)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Database-4169E1?logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Ambiente-2496ED?logo=docker&logoColor=white)

O **FinTrack Rust** é um projeto full stack que desenvolvi para colocar em prática meus estudos de Rust e construção de aplicações web. A proposta é reunir, em um só lugar, o cadastro e o acompanhamento de ativos de uma carteira de investimentos, com uma interface simples de utilizar e uma estrutura organizada no backend.

Neste projeto, trabalhei com autenticação de usuários, persistência de dados, operações de cadastro e atualização de ativos e cálculos de indicadores da carteira.

## Demonstração

[▶ Ver demonstração animada do FinTrack](./fintrack-demo.gif)

_A prévia animada será exibida aqui após a substituição do arquivo GIF no repositório._

[▶ Assistir ao vídeo completo da aplicação (MP4)](./screen_1792636521859.mp4)

## Funcionalidades

- Cadastro de usuários e autenticação com sessões.
- Login e logout.
- Dashboard com resumo da carteira.
- Cadastro, edição, listagem e exclusão de ativos.
- Cálculo de valor investido, valor atual e resultado da carteira.
- Armazenamento dos dados em PostgreSQL.
- API para consulta dos ativos do usuário autenticado.

## Tecnologias utilizadas

| Camada | Tecnologias |
| --- | --- |
| Backend | Rust, Axum, Tokio |
| Banco de dados | PostgreSQL, SQLx |
| Interface | HTML, CSS, Askama |
| Autenticação | JWT, cookies, Argon2 |
| Ambiente | Docker Compose, Cargo |

## Estrutura do projeto

```text
Projeto-fullstack-rust/
├── src/
│   ├── main.rs
│   ├── auth.rs
│   ├── analytics.rs
│   ├── config/
│   ├── models.rs
│   └── services.rs
├── templates/
│   ├── dashboard.html
│   ├── login.html
│   └── register.html
├── static/
├── migrations/
├── docker-compose.yml
├── Cargo.toml
└── .env.example
```

## Como executar localmente

É necessário ter **Rust**, **Cargo** e **Docker Desktop** instalados.

Clone o repositório e entre na pasta:

```bash
git clone https://github.com/JhonataFontoura/Projeto-fullstack-rust.git
cd Projeto-fullstack-rust
```

Crie o arquivo de configuração a partir do exemplo:

```powershell
Copy-Item .env.example .env
```

Confira a conexão com o PostgreSQL no arquivo `.env` e inicie o banco:

```bash
docker compose up -d
```

Depois, execute a aplicação:

```bash
cargo run
```

Com o servidor iniciado, acesse **http://127.0.0.1:3000** no navegador.

## Sobre o desenvolvimento

O FinTrack foi uma oportunidade de aprofundar meu conhecimento em Rust além dos exercícios de terminal. Ao desenvolver a aplicação, pude trabalhar com rotas HTTP, autenticação, consultas ao banco de dados, renderização de páginas e organização das responsabilidades do código.

Também procurei construir uma interface visual consistente, com um dashboard que apresenta as informações financeiras de forma clara.

O projeto continua sendo um espaço para aprimorar a arquitetura, as funcionalidades e a experiência de uso.

---

**Desenvolvido por [Jhonata Fontoura](https://github.com/JhonataFontoura)** · Ciência da Computação
