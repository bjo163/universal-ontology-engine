# OX-DX TUI Sound Integration

Source: OX-DX Sensory Brand System v1.0.

Default TUI sound mode:

**SILENT**

Optional:

**MINIMAL**

Attention mode:

**NOTIFY**

## SILENT

No routine audio.

This is canonical and fully featured.

## MINIMAL

Optional sound only for explicit high-value events:
- user-triggered TRACE completion;
- VERIFY completion;
- meaningful ERROR.

No sound for:
- navigation;
- focus;
- selection;
- scroll;
- help;
- loading;
- unknown.

## NOTIFY

May add an alert cue for:
- actionable warning;
- background operation completion requiring attention;
- error/fatal condition.

Still no routine-navigation beeps.

## Event map

| TUI event | Sound |
|---|---|
| inspect | optional Witness in MINIMAL |
| trace | optional Trace cue in MINIMAL |
| observe | normally silent |
| resolve | optional Settle in MINIMAL |
| verify | optional Verify cue |
| unknown | silent |
| error | optional Error cue |
| loading | silent |

## Terminal bell

Do not use the terminal bell as routine OX-DX identity.

If used for accessibility/attention, it must be explicitly configurable and not the only alert channel.

## Failure

If audio is unavailable, no TUI meaning is lost.
