# Progresso do Molt

**Estado:** scaffold mínimo da Parte 1 implementado; build e carregamento em Horizon ainda não verificados.
**Alvo:** GeckoLib **5.5.7**, Minecraft **26.3**.
**Snapshot fonte:** `7c96bef7e1658e4158453c17eb464f0fb7f92426`.
**Parte atual:** Parte 1 — validação do scaffold nativo; depois seguir para geometria e recursos.

## Parte 0 — escopo e proveniência

### Referências fixadas

- GeckoLib: [commit `7c96bef7e1658e4158453c17eb464f0fb7f92426`](https://github.com/bernie-g/geckolib/commit/7c96bef7e1658e4158453c17eb464f0fb7f92426). O catálogo `gradle/libs.versions.toml` declara GeckoLib 5.5.7, Minecraft 26.3 e Java 25; `changelog.md` contém a seção “GeckoLib v5.5.7”. Não há tag nem GitHub Release chamada exatamente `5.5.7` nas consultas feitas. Usar esse hash como snapshot identificado, não como alegação de tag oficial.
- Licença upstream: MIT; texto e copyright constam em [`LICENSE`](https://github.com/bernie-g/geckolib/blob/7c96bef7e1658e4158453c17eb464f0fb7f92426/LICENSE). Se código for derivado, manter avisos e incluir a licença/atribuição conforme exigido.
- Horizon: auditoria inicial da documentação/repositório no commit [`d146c591589aae395cbc6aadd65c2a4b96005655`](https://github.com/horizon-rust/horizon/tree/d146c591589aae395cbc6aadd65c2a4b96005655). A versão usada no scaffold foi depois fixada no commit do template abaixo.

### Mapa inicial do GeckoLib compartilhado

O código de biblioteca comum fica em `common/src/main/java/com/geckolib/`; principais áreas identificadas:

| Área | Caminho no GeckoLib | Responsabilidade a portar |
|---|---|---|
| Objetos animáveis | `animatable/` | Contratos de entidade, item, block entity e objetos animáveis. |
| Runtime de animação | `animation/` | Controllers, estados, keyframes, avaliação e eventos. |
| Cache | `cache/` | Cache de modelos/animações e estado por instância. |
| Carregamento | `loading/` | Leitura de geometria/animações e recursos JSON. |
| Modelos | `model/` | Hierarquia de ossos, cubos, transformações e modelo animado. |
| Renderizadores | `renderer/` | Renderização comum para entidades, itens, blocos e camadas; separar chamadas específicas do Minecraft/loader. |
| Rede | `network/` | Sincronização de disparos/estados que exigem cliente-servidor. |
| Eventos e objetos | `event/`, `object/`, `util/`, `constant/` | Eventos, tipos de suporte e utilitários; revisar necessidade por necessidade. |
| Mixins e serviços | `mixin/`, `service/`, `GeckoLib*Services.java` | Pontos acoplados ao runtime Java/loader; não copiar cegamente, substituir por integração Horizon. |

Este é um mapa de escopo, não uma equivalência completa de cada classe. A matriz detalhada recurso→API Horizon→lacuna será construída incrementalmente nas próximas partes.

### APIs e limites iniciais do Horizon

- A documentação de mods nativos descreve bundles Rust de mixins que se integram a hook points do jogo e precisam indicar compatibilidade com uma revisão/API do Horizon.
- Renderização disponível documentada inclui `native_mob_submit`, `native_block_entity_submit`, desenho de modelos standalone (`native_standalone_models` / `native_draw_model`, API 151) e hooks de camadas de entidade/jogador.
- Modelos standalone são modelos no formato do Horizon; não há indicação de que o jogo leia automaticamente o JSON de geometria GeckoLib. Molt precisa implementar parser, hierarquia/poses e avaliação de animação próprios.
- A documentação descreve pontos de extensão, mas não uma API pronta para substituir o runtime completo do GeckoLib. Cache, pose de ossos, controllers, Molang, eventos e sincronização continuam no escopo do Molt.
- Mods nativos não são sandboxed e rodam como código nativo com os direitos do jogador; isso deve constar na documentação de confiança/instalação.

## Parte 1 — scaffold implementado

### Template e compatibilidade

- Template: [`horizon-rust/horizon-native-mod-template`](https://github.com/horizon-rust/horizon-native-mod-template), snapshot [`cc8ce2d896782099abfa9f0abf26fa80c9e4bddb`](https://github.com/horizon-rust/horizon-native-mod-template/commit/cc8ce2d896782099abfa9f0abf26fa80c9e4bddb).
- Horizon pin do scaffold: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`.
- horizon-mixins pin: `d9c83d7096b7ea5b83678da2efdfd6161f74997e`, schema 6.
- Esses pins vêm do template oficial e fornecem a base do smoke hook. O pin do Horizon deve ser atualizado antes de depender de hooks de renderização mais novos.
- O template requer Linux, Rust 1.95.0, Python 3.12+ e Git; seu script usa `fcntl`. Windows não é validado. No ambiente desta execução não há WSL/Linux nem Python 3.12, então o build real não foi executado.

### Arquivos criados/adaptados

- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`: crate compartilhada de runtime e toolchain.
- `compatibility.json`: commits exatos e hash de fonte do Horizon.
- `mixins/molt/horizon-mixin.toml`: bundle Molt, módulo Rust injetado e hook inicial.
- `mixins/molt/break.rs`: adaptador do hook.
- `mixins/molt/runtime.rs`: runtime smoke que escreve uma linha `[molt]` ao iniciar uma tentativa de quebrar bloco; não altera o resultado da ação.
- `tools/horizon.py`: staging determinístico do Horizon pinado e comandos `stage`, `check`, `build`, adaptados do template.
- `README.md`: instruções de build e clones/checkouts pinados.
- `THIRD_PARTY_NOTICES.md`: atribuição MIT do template.

### Critério e pendência

A base de código e o fluxo para produzir o executável estão preparados. Ainda não posso marcar a Parte 1 como concluída, pois o critério inclui confirmar o carregamento real em Horizon e isso não foi verificado. O smoke hook também é somente uma prova de integração nativa; **não desenha modelos nem animações GeckoLib**.

Para validar em Linux/WSL depois de instalar Python 3.12+ e Rust 1.95.0, seguir os comandos de build no [README.md](README.md). Confirmar que `stage`, `check` e `build` passam e que quebrar um bloco imprime `[molt]` no stderr; então registrar aqui resultado e ambiente, sem copiar logs gerados para o repositório.

## Próxima etapa após validação

Começar a matriz de hooks de renderização no Horizon pinado e escolher o primeiro alvo visual (entidade ou block entity), antes de implementar o parser de geometria GeckoLib.

## Registro de atividade

- 2026-11-02: planejamento e auditoria da Parte 0; snapshot 5.5.7, módulos, licença e APIs iniciais registrados.
- 2026-11-02: scaffold mínimo da Parte 1 adicionado ao Molt; build/carregamento ainda não verificados neste ambiente.
