# OX-DX TUI Command Bar

The command bar exposes actions without pretending future interaction vocabulary already exists in the CLI.

## CURRENT CLI

Observed current commands:
- `levels`
- `inspect <level>`
- `validate`
- `discover <workspace>`
- `parse <language> <file>`
- `self [root]`

Global:
- `--registry <path>`

These names must be preserved when the TUI invokes current CLI behavior.

## FUTURE TUI VOCABULARY

Design vocabulary:
- inspect
- discover
- trace
- resolve
- observe
- verify
- query

Future vocabulary is not advertised as executable CLI commands until implemented.

## Command surface

```text
› inspect L22
```

ASCII:

`> inspect L22`

## Suggestions

Group suggestions:

```text
CURRENT
  inspect 22
  levels
  validate

TUI
  trace focused relation
  resolve finer
```

“TUI” actions are interaction intents, not shell commands.

## Mutation boundary

Search/navigation/read-only actions must remain distinguishable from any future mutation.

## History

Command history may exist later.

Do not automatically persist sensitive input without an explicit policy.

## Help

`?` opens contextual help.

No huge command dump by default.
