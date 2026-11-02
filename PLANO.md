# Plano do Molt: GeckoLib 5.5.7 para Horizon

## Objetivo

Criar o Molt como uma biblioteca nativa em Rust para Horizon que reproduza as principais capacidades de animação e renderização do GeckoLib 5.5.7. O foco é portar formatos e comportamento úteis aos autores de mods, integrando-os às APIs do Horizon. Isso não torna mods Java existentes compatíveis automaticamente: mods que dependem do GeckoLib ainda precisam ser portados ou adaptados.

## Alvo e decisões

- GeckoLib: **5.5.7**, conforme definido pelo projeto.
- Minecraft: **26.3**.
- Plataforma: **Horizon**, com APIs e versão exata a fixar durante a auditoria técnica.
- Linguagem: Rust.
- Licença do Molt: manter a licença do repositório; qualquer código derivado do GeckoLib deve preservar os avisos e cumprir a licença MIT upstream.
- Não trocar silenciosamente o alvo por `main` ou outra versão do GeckoLib. Primeiro localizar e registrar o commit/tag ou artefato exato correspondente a 5.5.7.

## Partes do trabalho

| Parte | Escopo | Critério de conclusão |
|---|---|---|
| 0. Escopo e proveniência | Fixar revisão exata do GeckoLib 5.5.7; mapear módulos compartilhados e licença. | Revisão verificável registrada e matriz de módulos/fontes pronta. |
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
