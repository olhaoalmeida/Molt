# Progresso do Molt

**Estado:** Parte 0 concluída; nenhuma implementação Rust ainda.
**Alvo:** GeckoLib **5.5.7**, Minecraft **26.3**.
**Snapshot fonte:** `7c96bef7e1658e4158453c17eb464f0fb7f92426`.
**Parte atual:** Parte 1 — base do Molt e integração Horizon, após este registro.

## Parte 0 — escopo e proveniência

### Referências fixadas

- GeckoLib: [commit `7c96bef7e1658e4158453c17eb464f0fb7f92426`](https://github.com/bernie-g/geckolib/commit/7c96bef7e1658e4158453c17eb464f0fb7f92426). O catálogo `gradle/libs.versions.toml` declara GeckoLib 5.5.7, Minecraft 26.3 e Java 25; `changelog.md` contém a seção “GeckoLib v5.5.7”. Não há tag nem GitHub Release chamada exatamente `5.5.7` nas consultas feitas. Usar esse hash como snapshot identificado, não como alegação de tag oficial.
- Licença upstream: MIT; texto e copyright constam em [`LICENSE`](https://github.com/bernie-g/geckolib/blob/7c96bef7e1658e4158453c17eb464f0fb7f92426/LICENSE). Se código for derivado, manter avisos e incluir a licença/atribuição conforme exigido.
- Horizon: auditoria inicial da documentação/repositório no commit [`d146c591589aae395cbc6aadd65c2a4b96005655`](https://github.com/horizon-rust/horizon/tree/d146c591589aae395cbc6aadd65c2a4b96005655). A versão final de Horizon para build ainda deve ser pinada junto à integração.

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

Este é um mapa de escopo, não uma equivalência completa de cada classe. A matriz detalhada recurso→API Horizon→lacuna será construída antes/de modo incremental durante a Parte 1.

### APIs e limites iniciais do Horizon

- A documentação de mods nativos descreve bundles Rust de mixins que se integram a hook points do jogo e precisam indicar compatibilidade com uma revisão/API do Horizon.
- Renderização disponível documentada inclui `native_mob_submit`, `native_block_entity_submit`, desenho de modelos standalone (`native_standalone_models` / `native_draw_model`, API 151) e hooks de camadas de entidade/jogador. Isso oferece pontos de integração para o MVP.
- Modelos standalone são modelos no formato do Horizon; não há indicação de que o jogo leia automaticamente o JSON de geometria GeckoLib. Molt precisa implementar parser, hierarquia/poses e avaliação de animação próprios.
- A documentação descreve renderização e pontos de extensão, mas a auditoria ainda não demonstrou uma API pronta para substituir o runtime completo do GeckoLib. Tratar cache, pose de ossos, controllers, Molang, eventos e sincronização como trabalho do Molt até prova em contrário.
- Como mods nativos não são sandboxed e rodam como código nativo com os direitos do jogador, manter isso em mente ao definir escopo e documentação de confiança/instalação.

## Próximo trabalho: Parte 1

1. Ler [PLANO.md](PLANO.md) e este registro.
2. Inspecionar o template oficial de mod nativo Horizon e escolher uma revisão compatível; registrar hash e `NATIVE_HOOKS_API` usados.
3. Definir estrutura mínima de crate/bundle Molt e integração de carregamento, sem ainda implementar o runtime inteiro.
4. Fazer um exemplo mínimo que confirme o carregamento do mod e o ponto de desenho selecionado; registrar arquivos e comando de verificação.
5. Em paralelo, começar a matriz de APIs de render para o primeiro alvo do MVP (entidade ou block entity).

## Registro de atividade

- 2026-11-02: plano inicial criado; sem implementação.
- 2026-11-02: Parte 0 auditada. Snapshot do GeckoLib 5.5.7 identificado por hash; módulos comuns, licença MIT, APIs iniciais e lacunas do Horizon registrados. Próximo: Parte 1.
