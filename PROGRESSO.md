# Progresso do Molt

**Estado:** Parte 0 concluída; Parte 1 preparada, sem código Rust do Molt ainda.
**Alvo:** GeckoLib **5.5.7**, Minecraft **26.3**.
**Snapshot fonte:** `7c96bef7e1658e4158453c17eb464f0fb7f92426`.
**Parte atual:** Parte 1 — base do Molt e integração Horizon.

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

Este é um mapa de escopo, não uma equivalência completa de cada classe. A matriz detalhada recurso→API Horizon→lacuna será construída incrementalmente na Parte 1.

### APIs e limites iniciais do Horizon

- A documentação de mods nativos descreve bundles Rust de mixins que se integram a hook points do jogo e precisam indicar compatibilidade com uma revisão/API do Horizon.
- Renderização disponível documentada inclui `native_mob_submit`, `native_block_entity_submit`, desenho de modelos standalone (`native_standalone_models` / `native_draw_model`, API 151) e hooks de camadas de entidade/jogador. Isso oferece pontos de integração para o MVP.
- Modelos standalone são modelos no formato do Horizon; não há indicação de que o jogo leia automaticamente o JSON de geometria GeckoLib. Molt precisa implementar parser, hierarquia/poses e avaliação de animação próprios.
- A documentação descreve renderização e pontos de extensão, mas a auditoria ainda não demonstrou uma API pronta para substituir o runtime completo do GeckoLib. Tratar cache, pose de ossos, controllers, Molang, eventos e sincronização como trabalho do Molt até prova em contrário.
- Mods nativos não são sandboxed e rodam como código nativo com os direitos do jogador; isso deve constar na documentação de confiança/instalação.

## Parte 1 — base e integração

### Template oficial auditado

- Template: [`horizon-rust/horizon-native-mod-template`](https://github.com/horizon-rust/horizon-native-mod-template), snapshot atual consultado em 2026-11-02: [commit `cc8ce2d896782099abfa9f0abf26fa80c9e4bddb`](https://github.com/horizon-rust/horizon-native-mod-template/commit/cc8ce2d896782099abfa9f0abf26fa80c9e4bddb).
- Ele confirma que o caminho para código com hooks/renderização é um native mixin bundle que gera um executável Horizon próprio. Para conteúdo somente de dados `.hmod`, o template recomenda `cargo horizon-mod new`; isso não basta para renderizar animações.
- O template fixa, para o próprio exemplo, Horizon `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`, horizon-mixins `d9c83d7096b7ea5b83678da2efdfd6161f74997e` e schema 6. Esses pins são referência do template, **não foram escolhidos como compatibilidade final do Molt**.
- Requisito informado pelo template: Linux, Rust 1.95.0, Python 3.12+ e Git. Windows/macOS não são validados; o script de staging usa `fcntl`. A validação do Molt deve ocorrer em Linux/WSL ou adaptar-se oficialmente à plataforma antes de alegar suporte.
- O template implementa staging via `horizon-mixins plan/apply`, mantendo a revisão upstream intacta, e compila um pequeno módulo Rust no cliente Horizon. O primeiro smoke test do Molt deve seguir esse fluxo; o hook de render deve ser adicionado somente após escolher e verificar uma revisão que contenha os pontos necessários.
- O scaffold ainda não foi copiado/criado no Molt; nenhum teste ou build foi executado nesta etapa.

### Próximas ações

1. Adotar/adaptar os arquivos mínimos do template (Cargo, `compatibility.json`, manifest do mixin e runtime), preservando avisos MIT do template quando aplicável.
2. Escolher um Horizon pin que inclua as APIs de render que o Molt vai usar; calcular/registrar hash de fonte e `NATIVE_HOOKS_API`. Não copiar automaticamente o pin antigo do exemplo.
3. Definir o primeiro smoke test de carregamento. Para o MVP visual, escolher entidade ou block entity e ligar o hook de desenho a um modelo standalone simples.
4. Registrar os comandos reproduzíveis e ambiente Linux/WSL necessário, sem afirmar validação antes de executar.

## Registro de atividade

- 2026-11-02: plano inicial criado; sem implementação.
- 2026-11-02: Parte 0 auditada. Snapshot do GeckoLib 5.5.7 identificado por hash; módulos comuns, licença MIT, APIs iniciais e lacunas do Horizon registrados.
- 2026-11-02: Parte 1 investigada. Template oficial e requisitos/pins de referência identificados; scaffold Molt ainda não criado e nenhum build/teste alegado.
