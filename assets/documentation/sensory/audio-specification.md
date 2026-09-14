# OX-DX Audio Specification — SPEC_ONLY v1.0

No actual audio masters are committed in this task.

These are practical production defaults for future implementation.

## Master format

- Container: **WAV**
- Codec: uncompressed PCM
- Sample rate: **48 kHz**
- Bit depth: **24-bit**
- UI one-shots: **mono by default**
- Sonic logo: mono-compatible; stereo allowed only for subtle width
- Ambient/installations: stereo permitted when context justifies it

Do not use lossy masters.

## Distribution formats

Web/application distribution:
- OGG as preferred compressed open format where supported;
- MP3 as broad compatibility fallback when needed.

Keep WAV as the source master.

## Loudness

### UI one-shots
Do not hard-normalize every transient to one LUFS number.

Production reference:
- roughly **-20 LUFS short-term** as a starting point for standard SFX;
- true peak generally **≤ -6 dBTP**;
- verify by perceptual matching inside the actual product mix.

### Sonic logo
Reference:
- roughly **-18 LUFS integrated/short-program**;
- true peak **≤ -3 dBTP**.

### Ambient
Reference:
- roughly **-24 LUFS integrated**;
- true peak **≤ -6 dBTP**;
- ambient must sit below interaction speech/SFX.

These are production targets, not semantic brand tokens.

## Fade behavior

For one-shots:
- eliminate clicks with very short attack/release ramps;
- do not add long reverb tails to simulate importance;
- leave silence after the event when possible.

## Frequency safety

Avoid:
- piercing narrow high-frequency tones;
- excessive sub-bass;
- sharp resonances that become painful on headphones;
- extreme stereo phase effects.

## Spatial behavior

- preserve mono compatibility;
- never encode unique meaning only through pan;
- avoid hard-left/hard-right confirmation;
- immersive spatial versions are optional derivatives.

## File naming

Audio:

`ox-dx-sfx-{event}-v{version}.wav`

Sonic identity:

`ox-dx-sonic-{name}-v{version}.wav`

Never use:
- final;
- latest;
- new;
- test2;
- newest.

## Quality control

Before promotion:
- no clipping;
- no DC offset;
- clean start/end;
- loudness compared within the full family;
- works on laptop speakers and headphones;
- meaning still available with audio disabled.
