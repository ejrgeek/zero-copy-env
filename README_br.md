<p align="center">
  <img
    src="https://erlondnjr.com.br/assets/photo/erlon2.png"
    alt="ejrgeek Logo"
    width="86"
  />
</p>

<span align="center">

[**Página em Inglês**](README.md) | [**Crates.io**](https://crates.io/crates/zero_copy_env) | [**Docs.rs**](https://docs.rs/zero_copy_env/latest/zero_copy_env/)

</span>

[![CI](https://github.com/ejrgeek/zero-copy-env/actions/workflows/ci.yml/badge.svg)](https://github.com/ejrgeek/zero-copy-env/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/zero_copy_env.svg)](https://crates.io/crates/zero-copy-env)
[![Docs.rs](https://docs.rs/zero_copy_env/badge.svg)](https://docs.rs/zero_copy_env)
[![License](https://img.shields.io/crates/l/zero_copy_env.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/zero_copy_env.svg)](https://crates.io/crates/zero_copy_env)

# Zero-Copy-Env

Acesso a variáveis ​​de ambiente sem `copy`, utilizando a memória do processo fornecida pelo sistema operacional.

## Visão geral

Este crate lê variáveis ​​de ambiente diretamente da memória do processo (`environ` em sistemas Unix), constrói um snapshot no momento da inicialização e fornece pesquisas rápidas O(1) durante o tempo de execução.

## Modelo

O sistema funciona em três fases:

1. O ambiente do processo é fornecido pelo sistema operacional.
2. Um `snapshot` é criado no primeiro acesso.
3. Todas as pesquisas são atendidas a partir de um `map` em memória.

Após a inicialização:
- Sem chamadas de sistema
- Sem varredura de ambiente
- Sem alocação por consulta

## Características

- `Snapshot` na inicialização (`lazy`, `thread-safe`)
- Complexidade média de tempo O(1) (`HashMap-based backend`)
- Sem alocação por consulta
- Projetado para um desempenho de tempo de execução previsível.

## Segurança

Este crate pressupõe que a memória do ambiente permaneça estável durante toda a duração do processo.

Pode tornar-se inválido se:
- O processo modificar variáveis ​​de ambiente em tempo de execução (`std::env::set_var`, `remove_var`)
- Uma FFI externa insegura modificar a memória de ambiente do processo

Esse comportamento não é verificado em tempo de execução.

## Modelo de Desempenho

- Custo de inicialização: O(n) (varredura do ambiente)
- Custo de busca: O(1) em média
- Custo de tempo de execução após a inicialização: acesso constante à memória apenas

## Dica de Uso

```rust
use zero_copy_env::get;

fn main() {
    let path = get("PATH");
    println!("{:?}", path);
}
```

## Testes

Executar testes:

```bash
cargo test
```

Executar testes de desempenho:

```bash
cargo bench
```

## Contribuição

Contribuições são bem-vindas.

Por favor, siga estas diretrizes:

**1. Mantenha o modelo de cópia zero intacto.**

Qualquer contribuição deve respeitar os princípios fundamentais do projeto:

- Sem alocações desnecessárias em caminhos críticos
- Sem chamadas de sistema por chamada
- Preservação da arquitetura baseada em snapshots

**2. Keep the API stable**

Evite alterações que quebrem a compatibilidade, a menos que sejam estritamente necessárias. Se necessário:

- Prefira alterações aditivas
- Use `features flags` para comportamento experimental


**3. O desempenho importa**

Se você propuser alterações que afetem o desempenho:

- Incluir benchmarks (`criterion`)
- Fornecer comparações de antes e depois, quando possível

**4. Estilo de código**

- Siga a formatação padrão do Rust (`cargo fmt`)
- Prefira blocos `unsafe` explícitos com justificativa
- Mantenha `modules minimal` e focados

## Roadmap

Melhorias planejadas:

- Backend opcional baseado em Vec para otimização da localidade do cache
- Refinamento opcional do suporte a no-std
- Conjunto de benchmarks ampliado para diversos ambientes

## [Licença MIT]
