# Progresso do Molt

**Estado:** planejamento e auditoria inicial. Ainda não há implementação Rust do Molt.
**Alvo escolhido:** GeckoLib **5.5.7**, Minecraft **26.3**.
**Parte atual:** Parte 0 — escopo e proveniência, conforme [PLANO.md](PLANO.md).

## Feito

- Confirmado o objetivo: biblioteca Rust para Horizon inspirada nas funcionalidades do GeckoLib 5.5.7.
- Revisada a organização geral do GeckoLib: núcleo compartilhado mais integrações específicas de loaders.
- Identificadas no Horizon APIs/caminhos de renderização para modelos e renderização de entidades e block entities; o formato de modelo do Horizon não substitui automaticamente a leitura de geometria GeckoLib.
- Criado o plano em partes no arquivo [PLANO.md](PLANO.md).

## Pendente nesta etapa

A revisão exata do código-fonte/artefato que corresponde à versão 5.5.7 ainda não foi fixada. Não assumir que a branch `main` é essa versão e não começar uma cópia de código antes de conferir a tag/commit e registrar a proveniência. O usuário definiu 5.5.7 como alvo; essa versão não deve ser alterada sem alinhamento.

## Próximo trabalho para Claude ou outro agente

1. Ler este arquivo e [PLANO.md](PLANO.md) antes de modificar o projeto.
2. Localizar no repositório oficial do GeckoLib o commit, tag ou artefato verificável correspondente exatamente a 5.5.7; anotar URL, hash/referência e licença.
3. Auditar o código compartilhado dessa revisão (especialmente geometria, animação, controllers, Molang e renderização) sem confundir código comum com adaptadores Forge/Fabric/NeoForge.
4. Criar uma matriz com recurso, caminho de origem no GeckoLib, API equivalente no Horizon, lacuna e proposta de implementação.
5. Atualizar este arquivo com os achados, links/caminhos precisos e próximo passo concreto. Se começar a Parte 1, registrar exatamente quais arquivos/crates foram criados e como verificar o carregamento mínimo.
6. Preservar avisos/licença MIT em qualquer código derivado do GeckoLib e documentar as adaptações necessárias ao Horizon.

## Registro de atividade

- 2026-11-02: planejamento inicial criado; nenhuma funcionalidade implementada ainda.
