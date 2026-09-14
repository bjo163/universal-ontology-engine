# OX-DX Sensory Implementation Matrix

Every sensory effect has a practical fallback.

| Effect/Event | Cost | Complexity | PRIMARY | FALLBACK | LOW-POWER | SILENT |
|---|---|---|---|---|---|---|
| TRACE | low | low | segmented SVG/CSS persistence rail | static rail + source label | one thin relation line | same visual, no audio |
| SCAN | low–medium | medium | bounded field density/witness sequence | static “INSPECTING” field state | one witness indicator | no audio |
| REVEAL | low | low | anchor-led geometry reveal | immediate resolved layout | immediate layout | no audio |
| RESOLVE | medium | medium | persistence/detail convergence | two-state CSS/SVG swap | immediate final state | no audio |
| FRAGMENT | low | low | bounded dropout/offset | static broken geometry | single break | no audio |
| ALIGN | low | low | anchors move toward shared rail | static aligned state | final state only | no audio |
| CONNECT | low | low | rail persistence grows between endpoints | static labeled relation | single line + label | no audio |
| DISCONNECT | low | low | local relation persistence drops | remove/mark relation | static removed state | no audio |
| SIGNAL | low | low | short cadence/pulse | static indicator | label only | no audio |
| DISTORT | medium | medium | local bounded offset/dropout | static interference mark | single break | no audio |
| COMPRESS/EXPAND | low | low | contextual density transition | immediate layout change | static layout | no audio |
| FOCUS | very low | low | witness focus frame | standard accessible focus outline | platform focus | no audio |
| OBSERVE | low | low | witness bracket + trace | static witness bracket | label + outline | no audio |
| VERIFY | low | low | settle + confirmation geometry | static VERIFIED label/mark | text + mark | no audio |
| UNKNOWN | very low | low | open boundary / void | UNKNOWN text + break | text | default silence |
| ERROR | low | low | local interruption then stable error state | static error state | text + local boundary | audio optional/off |
| TRANSITION | low–medium | medium | resolution-derived layer/context transition | immediate context swap | no animation | no audio |
| Sonic logo | n/a runtime | audio production | produced WAV master + derivatives | no sound, visual mark | no sound | no sound |
| UI SFX | low | audio | short event-specific one-shot | visual/text only | no audio | no audio |
| Ambient | optional | medium | opt-in low-density field | none | none | off |

## Performance policy

- No basic interaction requires WebGL.
- No basic interaction requires video.
- No essential interaction requires audio.
- No visual SFX requires random particle simulation.
- Routine effects should be implementable with SVG/CSS/native transitions.
- Repeated SFX must be rate-limited.
- Ambient is never required for product correctness.

## Complexity gate

If an implementation needs significantly more complexity than the fallback and does not increase comprehension, choose the fallback.
