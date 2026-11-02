# Plano do Molt: GeckoLib 5.5.7 para Horizon

## Objetivo

Criar o Molt como uma biblioteca nativa em Rust para Horizon que reproduza as principais capacidades de animação e renderização do GeckoLib 5.5.7. O foco é portar formatos e comportamento úteis aos autores de mods, integrando-os às APIs do Horizon. Isso não torna mods Java existentes compatíveis automaticamente: mods que dependem do GeckoLib ainda precisam ser portados ou adaptados.

## Alvo e decisões

- GeckoLib: **5.5.7**, para Minecraft **26.3**.
- Snapshot de código-fonte identificado: [`7c96bef7e1658e4158453c17eb464f0fb7f92426`](https://github.com/bernie-g/geckolib/commit/7c96bef7e1658e4158453c17eb464f0fb7f92426). O commit contém o changelog 5.5.7 e o catálogo Gradle declara GeckoLib 5.5.7/Minecraft 26.3.
- Não foi encontrada tag ou GitHub Release chamada exatamente `5.5.7`; portanto, o hash acima é o snapshot verificável adotado, não uma tag oficial com esse nome. Verificar novamente antes de atualizar a referência.
- Horizon: auditoria inicial usa a documentação em [`d146c591589aae395cbc6aadd65c2a4b96005655`](https://github.com/horizon-rust/horizon/tree/d146c591589aae395cbc6aadd65c2a4b96005655). A revisão que o Molt realmente compilará deve ser escolhida e fixada quando a Parte 1 começar.
- Linguagem: Rust.
- Licença do Molt: manter a licença do repositório; qualquer código derivado do GeckoLib deve preservar os avisos e cumprir a licença MIT upstream.
- Não trocar silenciosamente o alvo por `main` ou outra versão do GeckoLib.

## Partes do trabalho

| Parte | Escopo | Critério de conclusão |
|---|---|---|
| 0. Escopo e proveniência | Fixar snapshot do GeckoLib 5.5.7; mapear módulos compartilhados e licença; levantar APIs Horizon relevantes. | Revisão verificável registrada, mapa de módulos/fontes e lacunas iniciais registrados em `PROGRESSO.md`. |
| 1. Base do Molt e integração Horizon | Estrutura da crate Rust, configuração/build, ciclo de vida do mod e adaptadores às APIs de renderização do Horizon. | Molt carrega no Horizon e possui um exemplo mínimo executável. |
| 2. Geometria e recursos | Ler JSON de geometria estilo Bedrock: ossos, hierarquia, pivôs, cubos, UVs e locators; validar e armazenar modelos; recarga/cache de recursos. | Um modelo multi-osso é carregado e desenhado com transformações corretas. |
| 3. Animação | Ler definições de animação e avaliar keyframes de posição, rotação e escala; loops, interpolação/easing e transições. | Animação em loop reproduz keyframes em tempo de jogo. |
| 4. Runtime e controllers | Estado por objeto animável, cache/instâncias, controladores, prioridades, condições, disparos e animações concorrentes. | Controllers podem selecionar e controlar animações sem compartilhar estado indevido. |
| 5. Renderizadores | Integrar primeiro entidades/jogadores e block entities; depois itens, armaduras, camadas e demais caminhos suportados pelo Horizon. | Exemplos renderizam corretamente em cada categoria suportada e limitações ficam documentadas. |
| 6. Molang e eventos | Implementar avaliação Molang necessária; eventos de som, partículas e eventos customizados em keyframes; sincronização cliente/servidor quando aplicável. | Fixtures de expressões/eventos passam e a sincronização tem comportamento definido. |
| 7. Compatibilidade e publicação | Mod de exemplo, documentação para autores, matriz de recursos do GeckoLib e compatibilidade com APIs do Horizon. | Usuário consegue integrar o Molt seguindo documentação e exemplos reproduzíveis. |

## Marcos de entrega

1. **MVP visual:** um modelo multi-osso estático com uma animação simples em uma entidade do Horizon.
2. **Runtime utilizável:** controllers, estado por animável, transições e recursos de geometria/animation JSON.
3. **Cobertura ampla:** Molang, eventos, renderizadores adicionais e sincronização, priorizados conforme as APIs disponíveis.
4. **Compatibilidade documentada:** exemplos, limitações conhecidas e matriz comparativa com GeckoLib 5.5.7.

## Regras de implementação

- Separar o núcleo independente de plataforma dos adaptadores Horizon.
- Não portar glue de Forge/Fabric/NeoForge quando o Horizon oferece outro ciclo de vida ou render API.
- Tratar suporte a JSON GeckoLib como compatibilidade de formato/comportamento, sem prometer compatibilidade binária com a API Java.
- Registrar cada diferença deliberada em relação ao GeckoLib e cada limitação das APIs do Horizon.
- Manter fixtures pequenas e casos de referência para geometria, keyframes, controllers e Molang.
