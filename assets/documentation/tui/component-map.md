# OX-DX TUI Component Map

Future conceptual components:

## Shell
Owns terminal-level composition and active regions.

## Context Bar
Scope, mode, selected object summary, current resolution.

## Resolution Spine
49-level navigation/context.

## Evidence View
Source-first evidence + classification + relation.

## Source View
Native source/span.

## Graph View
Terminal-native typed topology.

## Trace View
Ordered evidence/relation traversal.

## Observation View
Observation kept separate from structure.

## Provenance View
Source/origin context.

## Diagnostics
Operational info/notice/warning/error/fatal.

## Status Bar
Quiet one-line orientation.

## Command Bar
Input/actions/current-vs-future command vocabulary.

## Help Surface
Contextual key/action help.

## Focus Marker
Current interaction location.

## Resolution Marker
Current canonical level.

## Composition

Components are responsibilities.

They are not required to map one-to-one to Rust structs/widgets.

A future implementation may combine responsibilities when the contract remains clear.
