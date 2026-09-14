# OX-DX System SFX Family v1 — SPEC_ONLY

Events:
- DISCOVER
- TRACE
- CONNECT
- RESOLVE
- VERIFY
- SUCCESS
- LOADING

## Shared DNA

Grain + Breath/Noise + Rail + Settle.

## Default behavior

- DISCOVER: 294 ms grain/noise organizing into rail
- TRACE: 294 ms grain → faint rail
- CONNECT: 294 ms paired rail settles
- RESOLVE: 294–490 ms instability reduces
- VERIFY: 196–294 ms short settle
- SUCCESS: optional 196 ms quiet settle
- LOADING: silent by default

## Boundary

Sound never upgrades evidence status.

VERIFY requires an actual named verification event.
