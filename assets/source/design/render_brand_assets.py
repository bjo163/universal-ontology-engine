from pathlib import Path
import math, re, json
from xml.etree import ElementTree as ET
import cairosvg
from PIL import Image
from matplotlib.textpath import TextPath
from matplotlib.font_manager import FontProperties
from matplotlib.path import Path as MplPath

ROOT=Path('.')

C={
 'void':'#07090A','charcoal':'#111418','graphite':'#20252B','white':'#F5F7F8','muted':'#99A2AA',
 'acid':'#8CFF00','cyan':'#00D8FF','ultraviolet':'#9C4DFF','paper':'#E8E4D8','black':'#000000'
}

for p in [
'assets/brand/logo','assets/brand/icon','assets/brand/wordmark','assets/brand/lockups','assets/brand/marks',
'assets/vector/primary','assets/vector/monochrome','assets/vector/inverse','assets/vector/variants',
'assets/raster/png','assets/raster/webp','assets/raster/favicon',
'assets/web/header','assets/web/hero','assets/web/footer','assets/web/backgrounds','assets/web/social',
'assets/documentation/github','assets/documentation/README','assets/documentation/technical',
'assets/source/design','assets/source/references']:
    (ROOT/p).mkdir(parents=True, exist_ok=True)

def svg(w,h,body,bg=None,viewbox=None):
    vb=viewbox or f'0 0 {w} {h}'
    bgrect=f'<rect width="100%" height="100%" fill="{bg}"/>' if bg else ''
    return f'''<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="{vb}" fill="none">\n{bgrect}\n{body}\n</svg>\n'''

def write(rel,content):
    p=ROOT/rel; p.parent.mkdir(parents=True, exist_ok=True); p.write_text(content,encoding='utf-8')

def symbol(stroke=C['white'], accent=None, sw=10, center_fill=None, detail=True):
    a=accent or stroke
    cf=center_fill or a
    s=f'''<g stroke="{stroke}" stroke-width="{sw}" stroke-linecap="square" stroke-linejoin="miter" fill="none">
  <path d="M18 24H66L92 80L66 136H18L48 80Z"/>
  <path d="M142 24H94L68 80L94 136H142L112 80Z"/>
</g>
<circle cx="80" cy="80" r="15" stroke="{a}" stroke-width="{max(4,sw*0.6):g}" fill="none"/>
<rect x="75" y="75" width="10" height="10" fill="{cf}" transform="rotate(45 80 80)"/>'''
    if detail:
        s += f'''\n<g stroke="{a}" stroke-width="2" opacity=".75">
  <path d="M80 4V18M80 142V156M4 80H18M142 80H156"/>
</g>'''
    return s

def wordmark(fill=C['white'], x=0, y=0, scale=1.0):
    # Custom chamfered industrial glyphs, width 560 x 100 baseline.
    # O outer + inner counter; X, hyphen, D, X
    parts=[]
    def p(d): parts.append(f'<path d="{d}" fill="{fill}" fill-rule="evenodd"/>')
    # O
    p('M0 20L20 0H95L115 20V80L95 100H20L0 80ZM24 25V75H91V25Z')
    # X
    p('M130 0H160L190 35L220 0H250L207 50L250 100H220L190 65L160 100H130L173 50Z')
    # hyphen
    p('M265 42H315V58H265Z')
    # D
    p('M330 0H405L440 20V80L405 100H330ZM354 24V76H397L416 65V35L397 24Z')
    # X
    p('M455 0H485L515 35L545 0H575L532 50L575 100H545L515 65L485 100H455L498 50Z')
    return f'<g transform="translate({x} {y}) scale({scale})">'+''.join(parts)+'</g>'

def textpath(text,x,y,size,fill=C['white'],family='DejaVu Sans Mono',weight='normal'):
    fp=FontProperties(family=family, weight=weight)
    tp=TextPath((0,0), text, size=size, prop=fp)
    verts=tp.vertices; codes=tp.codes
    d=[]; i=0
    for idx,(vx,vy) in enumerate(verts):
        c=codes[idx] if codes is not None else (MplPath.MOVETO if idx==0 else MplPath.LINETO)
        X=x+vx; Y=y-vy
        if c==MplPath.MOVETO: d.append(f'M{X:.2f},{Y:.2f}')
        elif c==MplPath.LINETO: d.append(f'L{X:.2f},{Y:.2f}')
        elif c==MplPath.CURVE3:
            # quadratic consumes current as control and next as end; handled in pair by peeking
            if idx+1 < len(verts):
                vx2,vy2=verts[idx+1]; X2=x+vx2; Y2=y-vy2
                d.append(f'Q{X:.2f},{Y:.2f} {X2:.2f},{Y2:.2f}')
        elif c==MplPath.CURVE4:
            if idx+2 < len(verts):
                vx2,vy2=verts[idx+1]; vx3,vy3=verts[idx+2]
                d.append(f'C{X:.2f},{Y:.2f} {x+vx2:.2f},{y-vy2:.2f} {x+vx3:.2f},{y-vy3:.2f}')
        elif c==MplPath.CLOSEPOLY: d.append('Z')
    # TextPath CURVE codes are sequential; duplicative commands are tolerated but can be invalid visually. Easier use polygons for text.
    polys=tp.to_polygons()
    pp=[]
    for poly in polys:
        pts=' '.join(f'{x+px:.2f},{y-py:.2f}' for px,py in poly)
        pp.append(f'<polygon points="{pts}"/>')
    return f'<g fill="{fill}" fill-rule="evenodd">'+''.join(pp)+'</g>'

def microgrid(w,h,step=32,opacity=.12):
    lines=[]
    for X in range(0,w+1,step): lines.append(f'<path d="M{X} 0V{h}"/>')
    for Y in range(0,h+1,step): lines.append(f'<path d="M0 {Y}H{w}"/>')
    return f'<g stroke="{C["muted"]}" stroke-width="1" opacity="{opacity}">'+''.join(lines)+'</g>'

def noise_lines(w,h):
    # deterministic grunge/signal scratches: no filters, stays vector.
    segs=[]
    for i in range(22):
        y=13+(i*37)%h; x=(i*83)%max(1,w-120); length=30+(i*29)%130
        segs.append(f'<path d="M{x} {y}h{length}"/>')
    for i in range(12):
        x=17+(i*71)%w; y=(i*53)%max(1,h-60)
        segs.append(f'<path d="M{x} {y}v{20+(i*11)%55}"/>')
    return f'<g stroke="{C["white"]}" opacity=".08" stroke-width="1">'+''.join(segs)+'</g>'

# Core brand SVGs
write('assets/brand/logo/ox-dx-logo-primary.svg', svg(800,180, f'{symbol(C["white"],C["cyan"],8,detail=False)}<g transform="translate(200 40)">{wordmark(C["white"],0,0,1.0)}</g>', bg=C['void'], viewbox='0 0 800 180'))
write('assets/brand/logo/ox-dx-logo-monochrome.svg', svg(800,180, f'<g transform="translate(0 10)">{symbol(C["black"],C["black"],8,detail=False)}</g><g transform="translate(200 40)">{wordmark(C["black"],0,0,1.0)}</g>', viewbox='0 0 800 180'))
write('assets/brand/logo/ox-dx-logo-white.svg', svg(800,180, f'<g transform="translate(0 10)">{symbol(C["white"],C["white"],8,detail=False)}</g><g transform="translate(200 40)">{wordmark(C["white"],0,0,1.0)}</g>', viewbox='0 0 800 180'))
write('assets/brand/logo/ox-dx-logo-black.svg', svg(800,180, f'<g transform="translate(0 10)">{symbol(C["black"],C["black"],8,detail=False)}</g><g transform="translate(200 40)">{wordmark(C["black"],0,0,1.0)}</g>', viewbox='0 0 800 180'))
write('assets/brand/logo/ox-dx-logo-dark.svg', svg(800,180, f'<g transform="translate(0 10)">{symbol(C["white"],C["cyan"],8,detail=False)}</g><g transform="translate(200 40)">{wordmark(C["white"],0,0,1.0)}</g>', bg=C['void'], viewbox='0 0 800 180'))
write('assets/brand/logo/ox-dx-logo-light.svg', svg(800,180, f'<g transform="translate(0 10)">{symbol(C["black"],C["ultraviolet"],8,detail=False)}</g><g transform="translate(200 40)">{wordmark(C["black"],0,0,1.0)}</g>', bg=C['white'], viewbox='0 0 800 180'))

write('assets/brand/icon/ox-dx-icon.svg', svg(160,160,symbol(C['white'],C['cyan'],10,detail=False),bg=C['void']))
write('assets/brand/icon/ox-dx-icon-monochrome.svg', svg(160,160,symbol(C['black'],C['black'],10,detail=False)))
write('assets/brand/icon/ox-dx-icon-inverse.svg', svg(160,160,symbol(C['white'],C['white'],10,detail=False),bg=C['black']))
write('assets/brand/icon/ox-dx-icon-transparent.svg', svg(160,160,symbol(C['white'],C['cyan'],10,detail=False)))
write('assets/brand/icon/ox-dx-favicon.svg', svg(64,64,'<g transform="scale(.4)">'+symbol(C['white'],C['cyan'],12,detail=False)+'</g>',bg=C['void']))

write('assets/brand/wordmark/ox-dx-wordmark.svg', svg(575,100,wordmark(C['white']),bg=C['void']))
write('assets/brand/lockups/ox-dx-lockup-horizontal.svg', svg(960,220, f'<g transform="translate(20 30)">{symbol(C["white"],C["cyan"],8,detail=False)}</g><g transform="translate(220 45)">{wordmark(C["white"],0,0,1.05)}</g>{textpath("Universal Ontology & Experience Engine",220,185,22,C["muted"])}',bg=C['void']))
write('assets/brand/lockups/ox-dx-lockup-stacked.svg', svg(720,520, f'<g transform="translate(280 30)">{symbol(C["white"],C["cyan"],8,detail=False)}</g><g transform="translate(72 250)">{wordmark(C["white"],0,0,1.0)}</g>{textpath("Universal Ontology & Experience Engine",98,420,22,C["muted"])}',bg=C['void']))
write('assets/brand/marks/ox-dx-mark.svg', svg(160,160,symbol(C['white'],C['cyan'],10,detail=True)))

# Canonical copies in vector dirs
write('assets/vector/primary/ox-dx-logo-primary.svg',(ROOT/'assets/brand/logo/ox-dx-logo-primary.svg').read_text())
write('assets/vector/monochrome/ox-dx-logo-monochrome.svg',(ROOT/'assets/brand/logo/ox-dx-logo-monochrome.svg').read_text())
write('assets/vector/inverse/ox-dx-logo-white.svg',(ROOT/'assets/brand/logo/ox-dx-logo-white.svg').read_text())

# Supporting vector language
res_grid = microgrid(800,480,40,.16)+noise_lines(800,480)+f'''<g transform="translate(320 160) scale(1.0)">{symbol(C['white'],C['cyan'],6,detail=True)}</g>
<g stroke="{C['cyan']}" fill="none" opacity=".75"><circle cx="400" cy="240" r="180"/><circle cx="400" cy="240" r="120"/><circle cx="400" cy="240" r="60"/></g>
<g fill="{C['white']}">''' + ''.join(f'<circle cx="{100+i*75}" cy="{360-(i%3)*35}" r="{max(2,9-i)}" opacity="{.9-i*.06:.2f}"/>' for i in range(8)) + '</g>'
write('assets/vector/primary/ox-dx-resolution-grid.svg', svg(800,480,res_grid,bg=C['void']))
relation = f'''<g stroke="{C['white']}" stroke-width="4" fill="none"><path d="M80 220L240 110L400 220L560 110L720 220"/><path d="M240 110L240 350L400 220L560 350V110"/></g><g fill="{C['cyan']}">'''+''.join(f'<circle cx="{x}" cy="{y}" r="12"/>' for x,y in [(80,220),(240,110),(240,350),(400,220),(560,110),(560,350),(720,220)])+'</g>'
write('assets/vector/primary/ox-dx-relation-node.svg',svg(800,460,relation,bg=C['void']))
layer = ''.join(f'<rect x="{80+i*35}" y="{60+i*28}" width="{520-i*70}" height="{320-i*56}" rx="0" stroke="{C["white"] if i%2==0 else C["cyan"]}" opacity="{.18+i*.12:.2f}"/>' for i in range(6))
write('assets/vector/primary/ox-dx-layer-pattern.svg',svg(800,460,f'<g fill="none" stroke-width="2">{layer}</g>',bg=C['void']))
obs=f'''<g stroke="{C['white']}" fill="none"><path d="M80 80H260M540 80H720M80 380H260M540 380H720"/><path d="M80 80V180M720 80V180M80 380V280M720 380V280"/></g><circle cx="400" cy="230" r="110" stroke="{C['cyan']}" fill="none"/><circle cx="400" cy="230" r="22" fill="{C['cyan']}"/>'''
write('assets/vector/primary/ox-dx-observation-frame.svg',svg(800,460,obs,bg=C['void']))
signal=f'''<g fill="{C['white']}">'''+''.join(f'<rect x="{60+i*22}" y="{230-(i%5)*12}" width="{10+(i%3)*4}" height="{2+(i%4)*3}" opacity="{.35+(i%5)*.1:.2f}"/>' for i in range(28))+f'''</g><path d="M70 230H720" stroke="{C['cyan']}" opacity=".7"/><circle cx="640" cy="230" r="28" stroke="{C['ultraviolet']}"/>'''
write('assets/vector/variants/ox-dx-signal-fragment.svg',svg(800,460,signal,bg=C['void']))
# ontology pattern visual only, no semantic labels
cells=[]
for r in range(7):
 for c in range(7):
  size=10+(6-r)*2
  cells.append(f'<rect x="{180+c*62}" y="{70+r*48}" width="{size}" height="{size}" transform="rotate(45 {180+c*62+size/2} {70+r*48+size/2})" fill="{C["white"]}" opacity="{0.12+0.025*(r+c):.2f}"/>')
write('assets/vector/variants/ox-dx-ontology-pattern.svg',svg(800,460,microgrid(800,460,46,.08)+''.join(cells),bg=C['void']))

# Backgrounds
bg_body=microgrid(1600,900,64,.08)+noise_lines(1600,900)+f'''<g opacity=".16" stroke="{C['cyan']}" fill="none"><circle cx="1180" cy="450" r="280"/><circle cx="1180" cy="450" r="190"/><circle cx="1180" cy="450" r="95"/></g>'''
write('assets/web/backgrounds/ox-dx-background-grid.svg',svg(1600,900,bg_body,bg=C['void']))

# Header
header_logo=f'<g transform="translate(20 20) scale(.5)">{symbol(C["white"],C["cyan"],9,detail=False)}</g><g transform="translate(120 34)">{wordmark(C["white"],0,0,.42)}</g>{textpath("Universal Ontology & Experience Engine",120,110,14,C["muted"])}'
write('assets/web/header/ox-dx-header-logo.svg',svg(520,130,header_logo,bg=C['void']))
write('assets/web/header/ox-dx-header-logo-dark.svg',svg(520,130,header_logo,bg=C['void']))
header_light=f'<g transform="translate(20 20) scale(.5)">{symbol(C["black"],C["ultraviolet"],9,detail=False)}</g><g transform="translate(120 34)">{wordmark(C["black"],0,0,.42)}</g>{textpath("Universal Ontology & Experience Engine",120,110,14,C["graphite"])}'
write('assets/web/header/ox-dx-header-logo-light.svg',svg(520,130,header_light,bg=C['white']))
write('assets/web/header/ox-dx-header-pattern.svg',svg(1440,160,microgrid(1440,160,40,.08)+noise_lines(1440,160),bg=C['void']))

# Hero desktop
flow_nodes=''.join(f'<circle cx="{x}" cy="520" r="{r}" fill="{C["white"]}" opacity="{op}"/>' for x,r,op in [(950,18,.9),(1020,14,.8),(1080,10,.7),(1130,7,.65),(1170,5,.6),(1205,3,.55),(1230,2,.5)])
hero_body=microgrid(1600,900,64,.07)+noise_lines(1600,900)+f'''<g transform="translate(100 160) scale(1.5)">{symbol(C['white'],C['cyan'],8,detail=True)}</g>
<g transform="translate(100 455)">{wordmark(C['white'],0,0,1.25)}</g>
{textpath("Universal Ontology & Experience Engine",105,610,24,C['muted'])}
{textpath("FROM STRUCTURE TO EXPERIENCE.",105,705,28,C['white'],weight='bold')}
{textpath("FROM UNIVERSE TO BIT.",105,765,20,C['cyan'])}
<g stroke="{C['white']}" fill="none" opacity=".26"><path d="M820 180L1250 180L1450 450L1250 720L820 720L980 450Z"/><path d="M920 250L1190 250L1325 450L1190 650L920 650L1040 450Z"/></g>
<g stroke="{C['cyan']}" fill="none"><circle cx="1130" cy="450" r="210" opacity=".35"/><circle cx="1130" cy="450" r="125" opacity=".55"/><circle cx="1130" cy="450" r="44"/></g>
<path d="M900 520H1240" stroke="{C['cyan']}" opacity=".55"/>{flow_nodes}'''
write('assets/web/hero/ox-dx-hero.svg',svg(1600,900,hero_body,bg=C['void']))
write('assets/web/hero/ox-dx-hero-desktop.svg',(ROOT/'assets/web/hero/ox-dx-hero.svg').read_text())
hero_mobile=microgrid(900,1600,56,.06)+noise_lines(900,1600)+f'''<g transform="translate(300 150) scale(1.85)">{symbol(C['white'],C['cyan'],8,detail=True)}</g><g transform="translate(160 600)">{wordmark(C['white'],0,0,1.02)}</g>{textpath("Universal Ontology & Experience Engine",155,770,20,C['muted'])}{textpath("FROM STRUCTURE TO EXPERIENCE.",105,1000,24,C['white'],weight='bold')}{textpath("FROM UNIVERSE TO BIT.",180,1080,18,C['cyan'])}<g stroke="{C['cyan']}" fill="none" opacity=".38"><circle cx="450" cy="1320" r="200"/><circle cx="450" cy="1320" r="120"/><circle cx="450" cy="1320" r="45"/></g>'''
write('assets/web/hero/ox-dx-hero-mobile.svg',svg(900,1600,hero_mobile,bg=C['void']))

# Footer
footer_pattern=f'''<path d="M0 90H280L360 140L440 90H720L800 140L880 90H1160L1240 140L1320 90H1600" stroke="{C['muted']}" opacity=".28"/><g fill="{C['cyan']}">'''+''.join(f'<circle cx="{160+i*180}" cy="{90+(i%2)*50}" r="4" opacity=".7"/>' for i in range(8))+'</g>'
write('assets/web/footer/ox-dx-footer-pattern.svg',svg(1600,220,footer_pattern,bg=C['void']))
write('assets/web/footer/ox-dx-footer-background.svg',svg(1600,500,microgrid(1600,500,64,.05)+noise_lines(1600,500)+footer_pattern,bg=C['void']))
write('assets/web/footer/ox-dx-footer-mark.svg',svg(220,220,f'<g transform="translate(30 30)">{symbol(C["white"],C["cyan"],9,detail=False)}</g>',bg=C['void']))

# GitHub / README assets
social=microgrid(1280,640,64,.07)+noise_lines(1280,640)+f'''<g transform="translate(80 130)">{symbol(C['white'],C['cyan'],8,detail=True)}</g><g transform="translate(310 160)">{wordmark(C['white'],0,0,1.25)}</g>{textpath("Universal Ontology & Experience Engine",315,345,26,C['muted'])}{textpath("FROM STRUCTURE TO EXPERIENCE.",315,445,30,C['white'],weight='bold')}{textpath("FROM UNIVERSE TO BIT.",315,515,20,C['cyan'])}'''
write('assets/web/social/ox-dx-github-social-preview.svg',svg(1280,640,social,bg=C['void']))
write('assets/documentation/github/ox-dx-github-banner.svg',svg(1280,320,header_logo+f'<g transform="translate(600 90)">{wordmark(C["white"],0,0,.8)}</g>'+textpath("FROM STRUCTURE TO EXPERIENCE.",600,230,20,C['cyan']),bg=C['void']))
readme_banner=microgrid(1200,360,60,.05)+f'''<g transform="translate(60 95) scale(.85)">{symbol(C['white'],C['cyan'],8,detail=False)}</g><g transform="translate(260 90)">{wordmark(C['white'],0,0,1.15)}</g>{textpath("Universal Ontology & Experience Engine",265,250,24,C['muted'])}{textpath("FROM STRUCTURE TO EXPERIENCE.",265,315,20,C['cyan'])}'''
write('assets/documentation/README/ox-dx-readme-banner.svg',svg(1200,360,readme_banner,bg=C['void']))
write('assets/documentation/README/ox-dx-readme-header.svg',svg(1000,220,f'<g transform="translate(20 30) scale(.8)">{symbol(C["white"],C["cyan"],8,detail=False)}</g><g transform="translate(205 55)">{wordmark(C["white"],0,0,.88)}</g>{textpath("Universal Ontology & Experience Engine",210,185,20,C["muted"])}',bg=C['void']))
divider=f'<path d="M0 40H420L500 80L580 40H1000" stroke="{C["white"]}" opacity=".34"/><circle cx="500" cy="80" r="7" fill="{C["cyan"]}"/>'
write('assets/documentation/README/ox-dx-readme-section-divider.svg',svg(1000,120,divider,bg=C['void']))
# Ontology spine visual: 7x7 unlabeled cells to avoid semantic relabeling
spine=''.join(f'<rect x="{80+c*100}" y="{55+r*34}" width="{68}" height="8" fill="{C["cyan"] if c==r%7 else C["white"]}" opacity="{.18+.08*((r+c)%4)}"/>' for r in range(7) for c in range(7))
write('assets/documentation/README/ox-dx-readme-ontology-spine.svg',svg(880,340,microgrid(880,340,44,.05)+spine,bg=C['void']))
arch=f'''<g stroke="{C['white']}" stroke-width="2" fill="none" opacity=".7"><rect x="80" y="80" width="180" height="80"/><rect x="350" y="80" width="180" height="80"/><rect x="620" y="80" width="180" height="80"/><path d="M260 120H350M530 120H620"/><path d="M710 160V260H440V160"/></g><g fill="{C['cyan']}"><circle cx="305" cy="120" r="6"/><circle cx="575" cy="120" r="6"/><circle cx="440" cy="260" r="6"/></g>'''
write('assets/documentation/README/ox-dx-readme-architecture.svg',svg(880,340,microgrid(880,340,44,.05)+arch,bg=C['void']))

# Docs markdown
write('assets/documentation/technical/typography.md', '''# OX-DX Typography\n\nThe OX-DX wordmark is custom vector geometry and is not a font. Distribution SVGs outline the lettering, so they do not require installed fonts.\n\n- **Primary display:** custom OX-DX chamfered wordmark geometry; use only for the brand name.\n- **Secondary/body:** a neutral grotesk or system sans with restrained weights. Recommended web stack: `Inter, ui-sans-serif, system-ui, sans-serif`.\n- **Technical/mono:** `ui-monospace, SFMono-Regular, Menlo, Consolas, monospace`.\n- **Hierarchy:** wordmark > concise uppercase statement > descriptor > technical annotation.\n\nDo not replace the canonical vector wordmark with typed text. Do not commit proprietary font binaries.\n''')
write('assets/documentation/technical/design-tokens.md', f'''# OX-DX Design Tokens\n\n## Color\n\n| Token | Value | Use |\n|---|---|---|\n| void | `{C['void']}` | Primary dark canvas |\n| charcoal | `{C['charcoal']}` | Raised dark surface |\n| graphite | `{C['graphite']}` | Secondary dark surface |\n| white | `{C['white']}` | Primary foreground |\n| muted | `{C['muted']}` | Annotation / secondary text |\n| acid | `{C['acid']}` | Optional accent |\n| cyan | `{C['cyan']}` | Primary signal accent |\n| ultraviolet | `{C['ultraviolet']}` | Optional secondary accent |\n\nThe canonical logo must always remain valid in pure black or pure white. Neon accents are optional.\n\n## Geometry\n\n- Base spacing unit: **8 px**.\n- Core grid: multiples of **8 px**; larger editorial layouts may use 32 / 64 px modules.\n- Corner radius: **0 px** for brand geometry; product UI may define its own tokens.\n- Canonical symbol stroke at 160 px artboard: **10 px**.\n- Fine annotation line: **1–2 px**.\n- Clear space: at least **1/4 of the symbol width** around the canonical mark.\n- Minimum symbol size: **16 px**. At 16–24 px, use the simplified favicon/icon variant and omit annotation ticks.\n\n## Contrast and background\n\n- Dark-first usage: `{C['white']}` on `{C['void']}`.\n- Light usage: pure black / `{C['black']}` on `{C['white']}`.\n- Do not place the canonical logo over noisy imagery without a quiet field.\n- Grunge belongs to the surface layer, never inside the master geometry.\n''')
write('assets/documentation/brand-guidelines.md', '''# OX-DX Brand Guidelines\n\n## Brand\n\n**OX-DX**  \nUniversal Ontology & Experience Engine\n\n**FROM STRUCTURE TO EXPERIENCE.**  \n**FROM UNIVERSE TO BIT.**\n\nThe canonical visual direction is **Resolution Engine**: a dark-first, mathematically ordered system where a stable geometric frame resolves toward a central observation node and increasingly fine signal. The influence is Quranic in depth through proportion, symmetry, repetition, balance, and layered geometry—not through religious iconography.\n\n## Voice\n\n- Quranic depth\n- Gen-Z clarity\n- Grunge attitude\n- Intelligent precision\n\n## Correct usage\n\nUse the supplied canonical SVG. Preserve geometry, aspect ratio, clear space, and contrast. Prefer clean logo files for product surfaces. Use grunge only in banners, hero art, backgrounds, social previews, and editorial dividers.\n\n## Incorrect usage\n\n- Do not stretch the logo.\n- Do not rotate the logo.\n- Do not distort the geometry.\n- Do not place it over unreadable backgrounds.\n- Do not invent colors.\n- Do not modify or retype the wordmark.\n- Do not add arbitrary effects to the canonical logo.\n- Do not insert Arabic calligraphy, verses, crescents, mosque imagery, or other religious emblems.\n- Do not turn the supporting graphics into new ontology semantics.\n''')
write('assets/README.md', '''# OX-DX Assets\n\nThis directory contains the canonical OX-DX visual asset system.\n\n- Master logo: `brand/logo/ox-dx-logo-primary.svg`\n- Master icon: `brand/icon/ox-dx-icon.svg`\n- Wordmark: `brand/wordmark/ox-dx-wordmark.svg`\n- Lockups: `brand/lockups/`\n- Web hero/header/footer/social: `web/`\n- GitHub and README graphics: `documentation/github/` and `documentation/README/`\n- Supporting vector language: `vector/`\n- Editable/source references: `source/`\n\nFuture contributors should derive new assets from the canonical geometry and documented tokens. Do not redraw the symbol, modify the wordmark, invent palette values, or embed new technical ontology semantics in decorative graphics.\n''')
write('assets/source/design/ox-dx-master-geometry.svg',svg(160,160,symbol(C['white'],C['cyan'],10,detail=True),bg=C['void']))
write('assets/source/design/palette.json',json.dumps(C,indent=2)+"\n")
write('assets/source/references/README.md','''# References\n\nThe identity uses the approved OX-DX Resolution Engine direction as its internal visual reference: mathematical order, dark technical surfaces, resolution geometry, and restrained grunge. No external artwork, stock imagery, or religious iconography is embedded here.\n''')

# Rasterize selected SVGs
raster_jobs=[
 ('assets/brand/icon/ox-dx-icon.svg','assets/raster/png/ox-dx-icon-1024.png',1024,1024),
 ('assets/brand/logo/ox-dx-logo-primary.svg','assets/raster/png/ox-dx-logo-primary.png',1600,360),
 ('assets/web/hero/ox-dx-hero.svg','assets/raster/png/ox-dx-hero.png',1600,900),
 ('assets/web/hero/ox-dx-hero.svg','assets/raster/webp/ox-dx-hero.webp',1600,900),
 ('assets/web/hero/ox-dx-hero-desktop.svg','assets/raster/webp/ox-dx-hero-desktop.webp',1600,900),
 ('assets/web/hero/ox-dx-hero-mobile.svg','assets/raster/webp/ox-dx-hero-mobile.webp',900,1600),
 ('assets/web/social/ox-dx-github-social-preview.svg','assets/web/social/ox-dx-github-social-preview.png',1280,640),
 ('assets/web/social/ox-dx-github-social-preview.svg','assets/web/social/ox-dx-github-social-preview.webp',1280,640),
]

def render(src,dst,w,h):
    sp=ROOT/src; dp=ROOT/dst; dp.parent.mkdir(parents=True,exist_ok=True)
    png_bytes=cairosvg.svg2png(url=str(sp),output_width=w,output_height=h)
    if dp.suffix.lower()=='.png': dp.write_bytes(png_bytes)
    elif dp.suffix.lower()=='.webp':
        import io
        im=Image.open(io.BytesIO(png_bytes)).convert('RGBA')
        im.save(dp,'WEBP',lossless=True,method=6)

for j in raster_jobs: render(*j)
# Favicons / icons exact sizes
for sz in [16,24,32,48,64,128,192,256,512,1024]:
    name = f'assets/raster/favicon/ox-dx-favicon-{sz}.png' if sz in [16,32,48] else f'assets/raster/png/ox-dx-icon-{sz}.png'
    render('assets/brand/icon/ox-dx-icon.svg',name,sz,sz)
# Alias requested filenames
for src,dst in [
 ('assets/raster/favicon/ox-dx-favicon-16.png','assets/raster/favicon/favicon-16.png'),
 ('assets/raster/favicon/ox-dx-favicon-32.png','assets/raster/favicon/favicon-32.png'),
 ('assets/raster/favicon/ox-dx-favicon-48.png','assets/raster/favicon/favicon-48.png'),
 ('assets/raster/png/ox-dx-icon-192.png','assets/raster/png/icon-192.png'),
 ('assets/raster/png/ox-dx-icon-512.png','assets/raster/png/icon-512.png'),
]:
    (ROOT/dst).write_bytes((ROOT/src).read_bytes())
# requested generic icon / favicon names via SVG copies
write('assets/brand/icon/icon.svg',(ROOT/'assets/brand/icon/ox-dx-icon.svg').read_text())
write('assets/brand/icon/favicon.svg',(ROOT/'assets/brand/icon/ox-dx-favicon.svg').read_text())
# Top-level named requested web exports aliases
write('assets/web/header/header-logo.svg',(ROOT/'assets/web/header/ox-dx-header-logo.svg').read_text())
write('assets/web/header/header-logo-dark.svg',(ROOT/'assets/web/header/ox-dx-header-logo-dark.svg').read_text())
write('assets/web/header/header-logo-light.svg',(ROOT/'assets/web/header/ox-dx-header-logo-light.svg').read_text())
write('assets/web/header/header-pattern.svg',(ROOT/'assets/web/header/ox-dx-header-pattern.svg').read_text())
write('assets/web/footer/footer-pattern.svg',(ROOT/'assets/web/footer/ox-dx-footer-pattern.svg').read_text())
write('assets/web/footer/footer-background.svg',(ROOT/'assets/web/footer/ox-dx-footer-background.svg').read_text())
write('assets/web/footer/footer-mark.svg',(ROOT/'assets/web/footer/ox-dx-footer-mark.svg').read_text())
# Requested generic names in README/github
write('assets/documentation/github/github-banner.svg',(ROOT/'assets/documentation/github/ox-dx-github-banner.svg').read_text())
for a,b in [('readme-banner.svg','ox-dx-readme-banner.svg'),('readme-header.svg','ox-dx-readme-header.svg'),('readme-section-divider.svg','ox-dx-readme-section-divider.svg'),('readme-ontology-spine.svg','ox-dx-readme-ontology-spine.svg'),('readme-architecture.svg','ox-dx-readme-architecture.svg')]:
    write('assets/documentation/README/'+a,(ROOT/'assets/documentation/README'/b).read_text())
# Generic vector primitive aliases requested
for src,dst in [
 ('ox-dx-resolution-grid.svg','resolution-grid.svg'),('ox-dx-relation-node.svg','relation-node.svg'),('ox-dx-layer-pattern.svg','layer-pattern.svg'),('ox-dx-observation-frame.svg','observation-frame.svg')]:
    write('assets/vector/primary/'+dst,(ROOT/'assets/vector/primary'/src).read_text())
for src,dst in [('ox-dx-signal-fragment.svg','signal-fragment.svg'),('ox-dx-ontology-pattern.svg','ontology-pattern.svg')]:
    write('assets/vector/variants/'+dst,(ROOT/'assets/vector/variants'/src).read_text())

# Asset index after all files
rows=[]
for p in sorted((ROOT/'assets').rglob('*')):
    if p.is_file() and p.name!='ASSET_INDEX.md':
        rel=p.relative_to(ROOT/'assets').as_posix()
        ext=p.suffix.lower().lstrip('.')
        cat=rel.split('/')[0]
        variant='canonical'
        for key in ['monochrome','white','black','dark','light','inverse','mobile','desktop','favicon']:
            if key in rel: variant=key; break
        purpose='OX-DX brand asset'
        if 'hero' in rel: purpose='Website hero / resolution narrative'
        elif 'header' in rel: purpose='Website or README header'
        elif 'footer' in rel: purpose='Footer visual system'
        elif 'social' in rel or 'github' in rel: purpose='GitHub / social preview'
        elif 'icon' in rel or 'favicon' in rel: purpose='Icon / avatar / favicon'
        elif 'logo' in rel: purpose='Canonical logo'
        elif 'wordmark' in rel: purpose='OX-DX wordmark'
        elif 'lockup' in rel: purpose='Logo lockup'
        elif rel.endswith('.md'): purpose='Brand documentation'
        rows.append((rel,purpose,ext.upper(),variant,cat.title()))
idx=['# OX-DX Asset Index','','| Asset | Purpose | Format | Variant | Recommended Usage |','|---|---|---|---|---|']
for r in rows: idx.append('| `'+r[0]+'` | '+r[1]+' | '+r[2]+' | '+r[3]+' | '+r[4]+' |')
write('assets/ASSET_INDEX.md','\n'.join(idx)+'\n')

# Validation
errors=[]
svgs=list((ROOT/'assets').rglob('*.svg'))
for p in svgs:
    try:
        tree=ET.parse(p); root=tree.getroot();
        if 'viewBox' not in root.attrib: errors.append(f'NO_VIEWBOX {p}')
        txt=p.read_text(encoding='utf-8')
        if '<image' in txt: errors.append(f'EMBEDDED_RASTER {p}')
        if 'font-family' in txt or '<text' in txt: errors.append(f'FONT_DEP {p}')
    except Exception as e: errors.append(f'XML {p}: {e}')
rasters=list((ROOT/'assets').rglob('*.png'))+list((ROOT/'assets').rglob('*.webp'))
for p in rasters:
    try:
        im=Image.open(p); im.verify()
    except Exception as e: errors.append(f'RASTER {p}: {e}')
# exact favicon dimensions
for sz in [16,32,48]:
    p=ROOT/f'assets/raster/favicon/favicon-{sz}.png'
    im=Image.open(p)
    if im.size!=(sz,sz): errors.append(f'DIM {p} {im.size}')
print(json.dumps({'files':len([p for p in (ROOT/'assets').rglob('*') if p.is_file()]),'svgs':len(svgs),'rasters':len(rasters),'errors':errors},indent=2))


# Co-located canonical exports required by the repository asset contract.
import shutil
for src_rel,dst_rel in [
('assets/raster/png/ox-dx-hero.png','assets/web/hero/ox-dx-hero.png'),
('assets/raster/webp/ox-dx-hero.webp','assets/web/hero/ox-dx-hero.webp'),
('assets/raster/webp/ox-dx-hero-desktop.webp','assets/web/hero/ox-dx-hero-desktop.webp'),
('assets/raster/webp/ox-dx-hero-mobile.webp','assets/web/hero/ox-dx-hero-mobile.webp'),
('assets/web/social/ox-dx-github-social-preview.png','assets/web/social/github-social-preview.png'),
('assets/web/social/ox-dx-github-social-preview.webp','assets/web/social/github-social-preview.webp'),
('assets/web/social/ox-dx-github-social-preview.png','assets/documentation/github/github-social-preview.png'),
('assets/web/social/ox-dx-github-social-preview.webp','assets/documentation/github/github-social-preview.webp'),
]:
    dst=ROOT/dst_rel; dst.parent.mkdir(parents=True,exist_ok=True); shutil.copy2(ROOT/src_rel,dst)

# Rebuild inventory after all aliases and source files exist.
rows=[]
for p in sorted((ROOT/'assets').rglob('*')):
    if p.is_file() and p.name!='ASSET_INDEX.md':
        rel=p.relative_to(ROOT/'assets').as_posix(); ext=p.suffix.lower().lstrip('.')
        cat=rel.split('/')[0]; variant='canonical'
        for key in ['monochrome','white','black','dark','light','inverse','mobile','desktop','favicon']:
            if key in rel: variant=key; break
        purpose='OX-DX brand asset'
        if 'hero' in rel: purpose='Website hero / resolution narrative'
        elif 'header' in rel: purpose='Website or README header'
        elif 'footer' in rel: purpose='Footer visual system'
        elif 'social' in rel or 'github' in rel: purpose='GitHub / social preview'
        elif 'icon' in rel or 'favicon' in rel: purpose='Icon / avatar / favicon'
        elif 'logo' in rel: purpose='Canonical logo'
        elif 'wordmark' in rel: purpose='OX-DX wordmark'
        elif 'lockup' in rel: purpose='Logo lockup'
        elif rel.endswith('.md'): purpose='Brand documentation'
        rows.append((rel,purpose,ext.upper(),variant,cat.title()))
idx=['# OX-DX Asset Index','','| Asset | Purpose | Format | Variant | Recommended Usage |','|---|---|---|---|---|']
for r in rows: idx.append('| `'+r[0]+'` | '+r[1]+' | '+r[2]+' | '+r[3]+' | '+r[4]+' |')
(ROOT/'assets/ASSET_INDEX.md').write_text('\n'.join(idx)+'\n',encoding='utf-8')

# Final fail-closed validation.
final_errors=[]
for p in (ROOT/'assets').rglob('*.svg'):
    try:
        r=ET.parse(p).getroot()
        if 'viewBox' not in r.attrib: final_errors.append(f'NO_VIEWBOX {p}')
        t=p.read_text(encoding='utf-8')
        if '<image' in t: final_errors.append(f'EMBEDDED_RASTER {p}')
        if '<text' in t or 'font-family' in t: final_errors.append(f'FONT_DEP {p}')
    except Exception as e: final_errors.append(f'XML {p}: {e}')
for p in list((ROOT/'assets').rglob('*.png'))+list((ROOT/'assets').rglob('*.webp')):
    try:
        im=Image.open(p); im.verify()
    except Exception as e: final_errors.append(f'RASTER {p}: {e}')
for sz in [16,32,48]:
    p=ROOT/f'assets/raster/favicon/favicon-{sz}.png'; im=Image.open(p)
    if im.size!=(sz,sz): final_errors.append(f'DIM {p} {im.size}')
if final_errors:
    print(json.dumps({'validation':'fail','errors':final_errors},indent=2))
    raise SystemExit(1)
print(json.dumps({'validation':'pass','files':len([p for p in (ROOT/'assets').rglob('*') if p.is_file()])},indent=2))
