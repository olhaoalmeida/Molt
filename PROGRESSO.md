# Progresso do Molt

**Estado:** Parte 1 construída e verificada (stage, check, test, build) com Rust 1.97; falta só a confirmação manual em jogo.
**Alvo:** GeckoLib **5.5.7**, Minecraft **26.3**.
**Snapshot fonte:** `7c96bef7e1658e4158453c17eb464f0fb7f92426`.
**Parte atual:** Parte 1 — confirmação manual em jogo; depois atualizar o pin do Horizon e seguir para geometria e recursos.

## 2026-11-04: pino atualizado para o Horizon `f40d4df` (branch `atualizar-pin-f40d4df`)
- Pinos: Horizon `f40d4df781963ed0a6e1e0fd41bf740f652e2e46` (API 359), horizon-mixins `ddd9700` (sem mudança); sha256 de `crates/horizon-client/src/game.rs` = `eef44118...28c0` (`compatibility.json`).
- Validado aqui (Rust `stable` 1.97): `stage`, `check`, `build --profile fast` (776 s) passam; o binário contém `[molt] Native hook reached`. O alvo `break_block(&mut self, pos: [i32; 3])` continua igual.
- Mudou o layout do Horizon: o jogo (`game`) agora está na biblioteca `horizon-client` e `main.rs` só chama `entry::main`. `tools/horizon.py test` passou a usar `--lib`; com `--bin` ele rodava **0 testes e saía com código 0**, o que parecia aprovação e não era. Agora: 1 teste (`game::molt::tests::message_preserves_signed_block_coordinates`) passa.
- Para rodar `stage` a ferramenta espera `../horizon-mixins` (link para `/home/claude/deps/horizon-mixins` nesta sessão) e `--horizon <checkout>`.
- Disco: a sessão tem ~16 GB livres; `target/horizon-cargo/debug` chega a 13 GB. Apague `debug/` se der "No space left on device" (se regenera).
- NÃO testado: o jogo no Windows do dono com este pino (o `.hnmod` entregue antes usa `68ff469`; precisa ser refeito com `tools/package.py`).

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

### Validação (2026-11-02)

Ambiente: Linux x86_64 (sandbox), Python 3.13, **Rust 1.97.0 (stable)**. O `rust-toolchain.toml` pede 1.95.0, mas
o servidor de distribuição do Rust estava inacessível neste ambiente, então tudo abaixo foi rodado com
`RUSTUP_TOOLCHAIN=stable` (1.97.0). **Não foi validado com 1.95.0.** Pins usados, sem alteração:
Horizon `ba6344a`, horizon-mixins `d9c83d7`, schema 6.

| Verificação | Resultado |
|---|---|
| `python3 tools/horizon.py stage` | passou; o hook (`crate::game::molt::before_break`) e o módulo `molt` aparecem no `game.rs` gerado |
| `python3 tools/horizon.py check` | passou (cerca de 3 min) |
| `python3 tools/horizon.py test` | passou: `game::molt::tests::message_preserves_signed_block_coordinates` roda dentro do binário de teste do Horizon (cerca de 5 min) |
| `python3 tools/horizon.py build` | passou (cerca de 16 min); gerou `target/horizon-cargo/fast/horizon-client` (70 MB) com a mensagem `[molt] Native hook reached at` embutida (confirmado por `strings`) |
| `cargo fmt --all --check` | passou (a formatação de `runtime.rs` estava errada e foi corrigida) |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | passou |
| `cargo test --workspace --locked --offline` | passou: 1 teste |
| `python3 tools/test_template.py` | passou: o bundle compila no fixture pequeno, o corpo original do método e o resultado são preservados, revisão incompatível e alvo ausente são rejeitados |
| `python3 tools/test_stage.py` | passou: estágio determinístico, checkout original intacto, auditoria de AST |

### O que estava faltando e foi corrigido

- `README.md` mandava rodar `check`/`build` sem baixar as dependências do Horizon, mas o script usa `--offline`, então
  falhava com "attempting to make an HTTP request, but --offline was specified". Foi adicionada a ação
  `python3 tools/horizon.py fetch` e o passo no README.
- Não havia teste unitário no `runtime.rs` (o template oficial tem). Adicionado, e com ele a ação `test`.
- Faltavam o CI (`.github/workflows/ci.yml`), os testes de fixture (`tools/test_template.py`, `tools/test_stage.py`) e a
  auditoria de AST (`tools/audit`), todos do template oficial; foram adicionados, adaptados ao nome Molt, e o
  `Cargo.toml` ganhou o workspace `tools/audit`.
- `runtime.rs` não passava em `cargo fmt --check`.

### Critério e pendência

O critério da Parte 1 pede que o Molt carregue em um executável Horizon gerado e tenha um exemplo mínimo executável.
**Verificado:** o Molt compila no Horizon fixado, o executável é gerado e contém o hook, e o teste do runtime passa
dentro do jogo. **Não verificado:** executar o jogo e ver a linha `[molt]` ao quebrar um bloco. O sandbox não tem
tela, GPU nem os dados do Minecraft, então isso é um teste manual:

1. Em Linux/WSL, rodar `python3 tools/horizon.py build --horizon ../horizon` (ou usar o executável gerado).
2. Iniciar o jogo com os recursos preparados, em uma pasta de dados temporária (`HORIZON_HOME`, ver o `CLAUDE.md` do
   Horizon), abrir um mundo e quebrar um bloco.
3. Confirmar que `[molt] Native hook reached at x, y, z` aparece no stderr. Registrar aqui o resultado.

Até esse passo, a Parte 1 fica **concluída exceto pela confirmação manual em jogo**.

### Observações

- O Horizon fixado (`ba6344a`) é de 2026-11-01; o Horizon atual (`897fbb4` em 2026-11-02) tem a API de hooks em 153 e
  reorganizou os hooks em "fragmentos". Atualizar o pin é uma tarefa à parte (Parte 2 precisa dos hooks de
  renderização, e o próprio `native_draw_model` é API 151), e não foi tentada.
- O pin é o mesmo do template oficial, validado por ele em 2026-10-30.

### Teste no Windows (pendente de você)

Não foi possível testar no Windows aqui. O caminho é o mesmo de qualquer mod nativo: o launcher do Horizon
compila o mod na sua própria máquina, com o toolchain dele (Rust 1.99.1 e MinGW para `windows-gnu`).

1. Gerar o arquivo: `python3 tools/package.py` (em Linux/WSL) produz `target/molt-0.1.0.hnmod`. O arquivo também
   foi enviado como anexo. Seu SHA-256 está no próprio resultado do comando.
2. No launcher, com o jogo 26.3 instalado: ligar **Native mods (experimental)** na instância.
3. No PowerShell, rodar `horizon-launcher native-build C:\caminho\molt-0.1.0.hnmod --trust-native-code`.
   Ele mostra o aviso "native mods run with full access to this computer" e só compila com essa flag.
4. Abrir o jogo **a partir do PowerShell**, para ver o stderr. O hook escreve com `eprintln!`, então a linha
   `[molt]` não aparece numa janela aberta pelo ícone.
5. Criar ou abrir um mundo, quebrar um bloco e procurar `[molt] Native hook reached at x, y, z`.

Riscos que ainda não foram verificados: o pin `68ff469` pode não existir no launcher que você tem (nesse caso
o build falha dizendo qual commit falta); e a linha `[molt]` pode não aparecer se o jogo for iniciado pelo
launcher e não pelo terminal. Se algo falhar, o log do build fica em `native/build.log`, conforme o launcher.

## Próxima etapa após validação

Começar a matriz de hooks de renderização no Horizon pinado e escolher o primeiro alvo visual (entidade ou block entity), antes de implementar o parser de geometria GeckoLib.

## Registro de atividade

- 2026-11-02: planejamento e auditoria da Parte 0; snapshot 5.5.7, módulos, licença e APIs iniciais registrados.
- 2026-11-02: scaffold mínimo da Parte 1 adicionado ao Molt; build/carregamento ainda não verificados neste ambiente.
- 2026-11-02: Parte 1 validada no sandbox (stage, check, test, build, fixture); corrigidos fetch offline, teste unitário, CI e formatação. Pendente: execução em jogo.
