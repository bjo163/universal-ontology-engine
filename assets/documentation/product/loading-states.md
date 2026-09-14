# OX-DX Loading and Process States

Loading communicates a real operation.

Potential labels:
- DISCOVERING
- INSPECTING
- MAPPING
- TRACING
- RESOLVING
- VERIFYING

Only use a label when it accurately describes the operation.

## Indeterminate

Use an indeterminate state when progress cannot be measured.

Do not invent percentages.

## Determinate

Use determinate progress only when the underlying operation exposes a meaningful numerator/denominator or completion measure.

## Preserve context

During loading:
- keep the target visible when practical;
- keep prior evidence readable;
- mark incoming regions as pending;
- do not blank the entire interface for local operations.

## Motion

Loading motion should be restrained.

Avoid:
- decorative spinners over every region;
- fake scan animations implying actual inspection stages;
- progress bars that advance by timer rather than evidence.

## Reduced motion

Provide static text or minimal state change under reduced-motion preference.

## Completion

When work finishes, transition to the final state without implying a semantic change beyond what the result supports.
