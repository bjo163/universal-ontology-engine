# OX-DX Responsive Terminal System

Terminal responsiveness changes information architecture, not font scale.

## 80×24 or smaller

Preserve:
- context;
- selected/current object;
- current level;
- primary evidence;
- critical status;
- navigation/command line.

Hide/collapse:
- secondary metadata;
- graph neighborhood;
- full provenance;
- full help;
- adjacent zone maps.

Use on-demand surfaces.

## 100×30

Add:
- relation summary;
- compact provenance;
- current zone resolution spine.

## 120×40

Add:
- secondary inspector rail;
- source + classification together;
- graph neighborhood or provenance beside primary.

## 160×50

Add:
- graph + evidence coexistence;
- full current-zone spine;
- provenance/trace rail;
- diagnostic details.

## 200×60+

Add:
- deeper relation context;
- comparison;
- secondary evidence;
- persistent provenance;
- extended source.

Do not:
- enlarge all padding;
- create more decorative borders;
- add empty dashboard panels.

## Height pressure

If height is constrained:
- status remains one line;
- command remains one line;
- primary view scrolls;
- help becomes modal/on-demand;
- secondary context collapses first.

## Resize stability

Keep when possible:
- focus;
- selection;
- scroll position;
- resolution;
- mode;
- filter.

A resize should not silently change semantic context.
