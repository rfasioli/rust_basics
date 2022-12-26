# rust_basics
studing rust programing language basic concepts

> References: 
> - https://www.tutorialspoint.com/rust
> - https://learnxinyminutes.com/docs/rust/

## Tecnologias
- Rust

## Compilar
``` bash
# Para Desenvolvimento
cargo build

# Gerar release
cargo build -r
```

## Conteúdo
| Caminho             | Descrição                                                |
|---------------------|----------------------------------------------------------|
| src/main.rs         | Exemplos / guia rápido                                   |
| src/Arrays          | Uso de arrays                                            |
| src/Collections     | Exemplos com vector, Hashmap, Hashset                    |
| src/DataTypesApp    | Tipos de dados                                           |
| src/FunctionsApp    | Exemplos de funções                                      |
| src/HelloWorldApp   | App hello world                                          |
| src/Modules         | Testando módulos com cargo                               |
| src/Ownership       | Alocação e apontamentos de memória (Ownership/Borrowing) |
| src/Slices          | Apontamento com Slices                                   |
| src/Structure       | Exemplos com Estrutura de dados                          |
| src/Tupples         | Exemplos com tupples                                     |
| src/ErrorHandling   | Tratando erros                                           |
| src/Generics        | Exemplos com generics                                    |
| src/IO              | Exemplos lendo e gravando streams (console e arquivo)    |
| guess-game-app      | Projeto de exemplo com Cargo                             |
| src/Iterator        | Exemplos de uso do Iterator                              |

## Package Manager

> **Definições:**
> - **Cargo** é o gerenciador de pacotes para o RUST.
> - **Crate** é uma unidade de compilação para o RUST.
> - **Module** é o agrupamento lógico do código em RUST.

| **Comandos comuns** | Descrição                      |
|---------------------|--------------------------------|
| `cargo build`       | compila um código RUST         |
| `cargo check`       | analisa o projeto              |
| `cargo run`         | compila e executa o rc/main.rs |
| `cargo clean`       | remove artefatos da compilação |
| `cargo update`      | atualiza dependencias          |
| `cargo new`         | cria um novo projeto           |

### Exemplos de uso
``` bash
# Create a binary crate
cargo new project_name --bin

# Create a library crate
cargo new project_name --lib

# To check the current version of cargo, execute the following command −
cargo --version
```

### Problemas comuns

- Ao executar o comando `cargo build` apresenta a mensagem *"Blocking waiting for file lock on package cache"*. Pode ocorrer por uma execução paralela, neste caso é só aguardar o término, ou por uma falha em um build anterior. Para este caso basta executar os comandos:
```
rm ~/.cargo/.package-cache
rm -rf ~/.cargo/registry/cache/*
rm -rf ~/.cargo/registry/index/*
```
