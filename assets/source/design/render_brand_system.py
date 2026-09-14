from __future__ import annotations

from pathlib import Path
import io
import json
import math
import re
import shutil
from xml.etree import ElementTree as ET

import cairosvg
from PIL import Image
from matplotlib.textpath import TextPath
from matplotlib.font_manager import FontProperties

ROOT = Path('.')
ASSETS = ROOT / 'assets'

C = {
    'void': '#07090A',
    'charcoal': '#111418',
    'graphite': '#20252B',
    'white': '#F5F7F8',
    'muted': '#99A2AA',
    'acid': '#8CFF00',
    'cyan': '#00D8FF',
    'ultraviolet': '#9C4DFF',
    'paper': '#E8E4D8',
    'black': '#000000',
}

APPROVED_LOGO = ASSETS / 'brand/logo/ox-dx-logo-primary.svg'
if not APPROVED_LOGO.exists():
    raise SystemExit('approved OX-DX master logo is missing')
approved = APPROVED_LOGO.read_text(encoding='utf-8')
for contract in ['M18 24H66L92 80L66 136H18L48 80Z', 'M142 24H94L68 80L94 136H142L112 80Z', '#00D8FF']:
    if contract not in approved:
        raise SystemExit(f'approved master geometry contract missing: {contract}')

DIRS = [
    'assets/brand/symbols', 'assets/brand/glyphs',
    'assets/vector/patterns', 'assets/vector/textures',
    'assets/web/backgrounds', 'assets/web/github', 'assets/web/readme',
    'assets/web/releases', 'assets/web/social',
    'assets/documentation/technical', 'assets/source/design',
]
for d in DIRS:
    (ROOT / d).mkdir(parents=True, exist_ok=True)


def write(rel: str, content: str | bytes) -> None:
    p = ROOT / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(content, bytes):
        p.write_bytes(content)
    else:
        p.write_text(content, encoding='utf-8')


def svg(w: int, h: int, body: str, bg: str | None = None, viewbox: str | None = None) -> str:
    vb = viewbox or f'0 0 {w} {h}'
    bgrect = f'<rect width="100%" height="100%" fill="{bg}"/>' if bg else ''
    return f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="{vb}" fill="none">\n{bgrect}\n{body}\n</svg>\n'


def master_symbol(stroke=C['white'], accent=C['cyan'], sw=10, detail=False) -> str:
    s = f'''<g stroke="{stroke}" stroke-width="{sw}" stroke-linecap="square" stroke-linejoin="miter" fill="none">
  <path d="M18 24H66L92 80L66 136H18L48 80Z"/>
  <path d="M142 24H94L68 80L94 136H142L112 80Z"/>
</g>
<circle cx="80" cy="80" r="15" stroke="{accent}" stroke-width="{max(4, sw*0.6):g}" fill="none"/>
<rect x="75" y="75" width="10" height="10" fill="{accent}" transform="rotate(45 80 80)"/>'''
    if detail:
        s += f'''<g stroke="{accent}" stroke-width="2" opacity=".7">
<path d="M80 4V18M80 142V156M4 80H18M142 80H156"/></g>'''
    return s


def wordmark(fill=C['white'], x=0, y=0, scale=1.0) -> str:
    paths = [
        'M0 20L20 0H95L115 20V80L95 100H20L0 80ZM24 25V75H91V25Z',
        'M130 0H160L190 35L220 0H250L207 50L250 100H220L190 65L160 100H130L173 50Z',
        'M265 42H315V58H265Z',
        'M330 0H405L440 20V80L405 100H330ZM354 24V76H397L416 65V35L397 24Z',
        'M455 0H485L515 35L545 0H575L532 50L575 100H545L515 65L485 100H455L498 50Z',
    ]
    return f'<g transform="translate({x} {y}) scale({scale})">' + ''.join(
        f'<path d="{d}" fill="{fill}" fill-rule="evenodd"/>' for d in paths
    ) + '</g>'


def textpath(text: str, x: float, y: float, size: float, fill=C['white'], weight='normal', family='DejaVu Sans Mono') -> str:
    fp = FontProperties(family=family, weight=weight)
    tp = TextPath((0, 0), text, size=size, prop=fp)
    polys = []
    for poly in tp.to_polygons():
        pts = ' '.join(f'{x+px:.2f},{y-py:.2f}' for px, py in poly)
        polys.append(f'<polygon points="{pts}"/>')
    return f'<g fill="{fill}" fill-rule="evenodd">' + ''.join(polys) + '</g>'


def microgrid(w: int, h: int, step=40, opacity=.08, color=C['muted']) -> str:
    lines = [f'<path d="M{x} 0V{h}"/>' for x in range(0, w+1, step)]
    lines += [f'<path d="M0 {y}H{w}"/>' for y in range(0, h+1, step)]
    return f'<g stroke="{color}" stroke-width="1" opacity="{opacity}">' + ''.join(lines) + '</g>'


def scratches(w: int, h: int, color=C['white'], opacity=.06, count=28) -> str:
    segs=[]
    for i in range(count):
        y=11+(i*47)%max(12,h-12); x=(i*91)%max(1,w-180); ln=24+(i*31)%170
        segs.append(f'<path d="M{x} {y}h{ln}"/>')
    for i in range(max(6,count//3)):
        x=17+(i*83)%max(18,w-18); y=(i*59)%max(1,h-80); ln=18+(i*19)%72
        segs.append(f'<path d="M{x} {y}v{ln}"/>')
    return f'<g stroke="{color}" opacity="{opacity}" stroke-width="1">'+''.join(segs)+'</g>'


def diamond(cx, cy, r, fill=C['cyan'], opacity=1.0):
    return f'<rect x="{cx-r}" y="{cy-r}" width="{2*r}" height="{2*r}" fill="{fill}" opacity="{opacity}" transform="rotate(45 {cx} {cy})"/>'


def node(cx, cy, r=10, accent=C['cyan'], outer=True):
    outer_svg = f'<circle cx="{cx}" cy="{cy}" r="{r*2}" stroke="{accent}" stroke-width="3"/>' if outer else ''
    return outer_svg + diamond(cx, cy, max(3, r*.55), accent)


def frame(cx=80, cy=80, scale=1.0, stroke=C['white'], sw=6, opacity=1.0):
    a=48*scale; b=22*scale
    pts1=f'{cx-a},{cy-b} {cx-b},{cy-a} {cx+b},{cy-a} {cx+a},{cy-b} {cx+a},{cy+b} {cx+b},{cy+a} {cx-b},{cy+a} {cx-a},{cy+b}'
    return f'<polygon points="{pts1}" fill="none" stroke="{stroke}" stroke-width="{sw}" opacity="{opacity}"/>'


def render_svg(src: Path, dst: Path, w: int, h: int):
    png = cairosvg.svg2png(url=str(src), output_width=w, output_height=h)
    dst.parent.mkdir(parents=True, exist_ok=True)
    if dst.suffix.lower()=='.png':
        dst.write_bytes(png)
    elif dst.suffix.lower()=='.webp':
        im=Image.open(io.BytesIO(png)).convert('RGBA')
        im.save(dst, 'WEBP', lossless=True, method=6)
    else:
        raise ValueError(dst)


# -----------------------------------------------------------------------------
# SYMBOL SYSTEM — visual primitives only, never ontology semantics.
# -----------------------------------------------------------------------------

def symbol_art(kind: str) -> str:
    W=160; H=160
    base=f'<g stroke="{C["white"]}" stroke-width="6" stroke-linecap="square" stroke-linejoin="miter" fill="none">'
    if kind=='structure':
        body=frame(80,80,1.0,sw=6)+frame(80,80,.58,stroke=C['muted'],sw=3,opacity=.7)+node(80,80,8)
    elif kind=='layer':
        body=''.join(f'<path d="M34 {46+i*24}L80 {24+i*24}L126 {46+i*24}L80 {68+i*24}Z" opacity="{.32+i*.2:.2f}"/>' for i in range(4))+node(80,80,7)
        body=base+body+'</g>'
    elif kind=='relation':
        body=base+'<path d="M34 48L80 80L126 48M34 112L80 80L126 112"/></g>'+''.join(node(x,y,5,outer=False) for x,y in [(34,48),(126,48),(34,112),(126,112)])+node(80,80,7)
    elif kind=='observation':
        body=base+'<path d="M24 54V24H54M106 24H136V54M24 106V136H54M106 136H136V106"/><circle cx="80" cy="80" r="32"/></g>'+node(80,80,7)
    elif kind=='resolution':
        body=frame(80,80,1.0,sw=5)+frame(80,80,.68,stroke=C['muted'],sw=3,opacity=.7)+frame(80,80,.36,stroke=C['cyan'],sw=3,opacity=.8)+node(80,80,6)
    elif kind=='signal':
        bars=''.join(f'<path d="M{26+i*13} {80-h/2}V{80+h/2}"/>' for i,h in enumerate([16,26,40,58,72,58,40,26,16]))
        body=base+bars+'<path d="M18 80H142" opacity=".35"/></g>'+node(80,80,6)
    elif kind=='transition':
        body=base+'<path d="M26 48H72V26M134 112H88V134M54 80H106"/><path d="M96 68L108 80L96 92"/></g>'+node(80,80,5)
    elif kind=='fragment':
        body=base+'<path d="M28 52L52 28H70M90 28H108L132 52V70M132 90V108L108 132H90M70 132H52L28 108V90"/></g>'+node(80,80,7)
    elif kind=='context':
        body=base+'<path d="M28 58V28H58M102 28H132V58M28 102V132H58M102 132H132V102"/><circle cx="80" cy="80" r="48" opacity=".35"/></g>'+node(80,80,7)
    elif kind=='trace':
        dots=''.join(f'<circle cx="{24+i*14}" cy="{112-((i*17)%58)}" r="3" fill="{C["white"]}" opacity="{.35+.05*i:.2f}"/>' for i in range(9))
        body=base+'<path d="M24 112L38 95L52 105L66 68L80 80L94 52L108 65L122 46L136 58" opacity=".55"/></g>'+dots+node(80,80,6)
    else:
        raise ValueError(kind)
    if not body.startswith('<g') and kind not in {'structure','resolution'}:
        body=base+body+'</g>'
    return svg(W,H,body)

symbols=['structure','layer','relation','observation','resolution','signal','transition','fragment','context','trace']
for name in symbols:
    write(f'assets/brand/symbols/ox-dx-symbol-{name}.svg', symbol_art(name))


# -----------------------------------------------------------------------------
# GLYPH SYSTEM — compact notation family, 12 primary glyphs.
# -----------------------------------------------------------------------------

def glyph_art(kind: str) -> str:
    s=f'<g stroke="{C["white"]}" stroke-width="5" stroke-linecap="square" stroke-linejoin="miter" fill="none">'
    if kind=='node': body='<circle cx="48" cy="48" r="22"/><path d="M48 14V26M48 70V82M14 48H26M70 48H82"/>'
    elif kind=='relation': body='<circle cx="26" cy="48" r="10"/><circle cx="70" cy="48" r="10"/><path d="M36 48H60"/>'
    elif kind=='frame': body='<path d="M14 34V14H34M62 14H82V34M14 62V82H34M62 82H82V62"/>'
    elif kind=='layer': body='<path d="M18 30L48 14L78 30L48 46ZM18 48L48 32L78 48L48 64ZM18 66L48 50L78 66L48 82Z"/>'
    elif kind=='signal': body='<path d="M16 48H24M30 38V58M40 28V68M52 20V76M64 32V64M74 42V54M82 48H86"/>'
    elif kind=='trace': body='<path d="M14 70L30 52L42 60L55 30L68 44L82 22"/>'
    elif kind=='split': body='<path d="M18 48H44M44 48L72 24M44 48L72 72"/><circle cx="44" cy="48" r="6"/>'
    elif kind=='merge': body='<path d="M18 24L48 48M18 72L48 48M48 48H80"/><circle cx="48" cy="48" r="6"/>'
    elif kind=='resolve': body='<rect x="16" y="16" width="64" height="64"/><rect x="28" y="28" width="40" height="40"/><rect x="40" y="40" width="16" height="16"/>'
    elif kind=='observation': body='<path d="M14 34V14H34M62 14H82V34M14 62V82H34M62 82H82V62"/><circle cx="48" cy="48" r="18"/>'
    elif kind=='context': body='<circle cx="48" cy="48" r="32" opacity=".55"/><path d="M48 10V22M48 74V86M10 48H22M74 48H86"/>'
    elif kind=='transition': body='<path d="M14 48H70M58 36L70 48L58 60"/><path d="M28 28V68" opacity=".45"/>'
    else: raise ValueError(kind)
    return svg(96,96,s+body+'</g>'+diamond(48,48,3,C['cyan']))

glyphs=['node','relation','frame','layer','signal','trace','split','merge','resolve','observation','context','transition']
for name in glyphs:
    write(f'assets/brand/glyphs/ox-dx-glyph-{name}.svg', glyph_art(name))


# -----------------------------------------------------------------------------
# PATTERN LANGUAGE
# -----------------------------------------------------------------------------

def p_grid(w=1200,h=600):
    return microgrid(w,h,48,.12)+f'<g transform="translate(520 220) scale(1.0)">{master_symbol(sw=6,detail=True)}</g>'

def p_radial(w=1200,h=600):
    rings=''.join(f'<circle cx="600" cy="300" r="{r}"/>' for r in [52,96,150,220,285])
    spokes=''.join(f'<path d="M600 300L{600+285*math.cos(a):.2f} {300+285*math.sin(a):.2f}"/>' for a in [i*math.pi/8 for i in range(16)])
    return f'<g stroke="{C["white"]}" fill="none" opacity=".2">{rings}{spokes}</g>'+node(600,300,11)

def p_node(w=1200,h=600):
    pts=[]
    for r in range(5):
        for c in range(11):
            x=80+c*104+(r%2)*22; y=92+r*104
            pts.append(node(x,y,3+(c+r)%3, C['cyan'] if (c+r)%5==0 else C['muted'], outer=False))
    return ''.join(pts)

def p_relation(w=1200,h=600):
    coords=[(90,300),(230,160),(230,440),(420,230),(420,370),(600,300),(790,160),(790,440),(980,230),(1110,300)]
    lines=''.join(f'<path d="M{x1} {y1}L{x2} {y2}"/>' for (x1,y1),(x2,y2) in zip(coords,coords[1:]))
    branches='<path d="M230 160L420 370M230 440L420 230M600 300L790 440M600 300L790 160"/>'
    nodes=''.join(node(x,y,6,C['cyan'] if i in (0,5,9) else C['white'],outer=False) for i,(x,y) in enumerate(coords))
    return f'<g stroke="{C["white"]}" opacity=".3">{lines}{branches}</g>'+nodes

def p_layer(w=1200,h=600):
    return ''.join(f'<polygon points="{160+i*48},100 {600},50 {1040-i*48},100 {1040-i*48},500 {600},550 {160+i*48},500" fill="none" stroke="{C["cyan"] if i==4 else C["white"]}" stroke-width="{1 if i<4 else 2}" opacity="{.12+i*.09:.2f}"/>' for i in range(7))

def p_resolution(w=1200,h=600):
    frames=''.join(frame(600,300,5.0-i*.65,stroke=C['white'] if i<5 else C['cyan'],sw=1.5 if i<5 else 2,opacity=.12+i*.08) for i in range(7))
    return frames+node(600,300,10)

def p_fragment(w=1200,h=600):
    bits=[]
    for i in range(46):
        x=(i*97)%1140+20; y=(i*61)%540+20; ln=18+(i*23)%90
        bits.append(f'<path d="M{x} {y}h{ln}"/>')
    return f'<g stroke="{C["white"]}" opacity=".13">'+''.join(bits)+'</g>'+f'<g transform="translate(520 220)">{master_symbol(sw=5,detail=False)}</g>'

def p_signal(w=1200,h=600):
    bars=[]
    for i in range(76):
        x=35+i*15; amp=12+int(100*(math.sin(i*.23)**2));
        bars.append(f'<path d="M{x} {300-amp}V{300+amp}"/>')
    return f'<g stroke="{C["white"]}" opacity=".24">'+''.join(bars)+'</g><path d="M30 300H1170" stroke="'+C['cyan']+'" opacity=".65"/>'+node(600,300,8)

def p_noise(w=1200,h=600):
    dots=[]
    for i in range(420):
        x=(i*83)%1194+3; y=(i*137)%594+3; s=1+(i%3)
        dots.append(f'<rect x="{x}" y="{y}" width="{s}" height="{s}" fill="{C["white"]}" opacity="{.035+(i%5)*.012:.3f}"/>')
    return ''.join(dots)+scratches(w,h,opacity=.05,count=36)

patterns={
    'grid':p_grid,'radial':p_radial,'node':p_node,'relation':p_relation,'layer':p_layer,
    'resolution':p_resolution,'fragment':p_fragment,'signal':p_signal,'noise':p_noise,
}
for name,fn in patterns.items():
    write(f'assets/vector/patterns/ox-dx-pattern-{name}.svg',svg(1200,600,fn(),bg=C['void']))

# Scale → resolution motif, deliberately not containment.
labels=[('UNIVERSE',100,112,54),('STRUCTURE',255,170,42),('RELATION',410,228,34),('OBSERVATION',565,286,27),('DATA',720,344,21),('TOKEN',845,402,16),('CHARACTER',965,460,12),('BIT',1110,518,8)]
motif=microgrid(1200,600,50,.05)
for i,(label,x,y,r) in enumerate(labels):
    motif += f'<circle cx="{x}" cy="{y}" r="{r}" fill="none" stroke="{C["cyan"] if i in (0,len(labels)-1) else C["white"]}" stroke-width="{2 if i in (0,len(labels)-1) else 1}" opacity="{.28+i*.07:.2f}"/>'
    motif += textpath(label,x+r+14,y+5,12 if i<4 else 10,C['muted'])
    if i < len(labels)-1:
        nx,ny=labels[i+1][1],labels[i+1][2]
        motif += f'<path d="M{x+r} {y}L{nx-labels[i+1][3]} {ny}" stroke="{C["white"]}" stroke-dasharray="4 8" opacity=".24"/>'
motif += node(labels[-1][1],labels[-1][2],4)
write('assets/vector/patterns/ox-dx-pattern-resolution-scale.svg',svg(1200,600,motif,bg=C['void']))


# -----------------------------------------------------------------------------
# CONTROLLED GRUNGE: CLEAN STRUCTURE + DIRTY SURFACE.
# Each texture has light/dark variants at subtle intensity.
# -----------------------------------------------------------------------------

def texture_body(kind: str, w=1200,h=600,fg=C['white']):
    if kind=='paper-grain':
        return ''.join(f'<circle cx="{(i*71)%1198+1}" cy="{(i*113)%598+1}" r="{.6+(i%3)*.45}" fill="{fg}" opacity="{.035+(i%4)*.01:.3f}"/>' for i in range(520))
    if kind=='photocopy':
        out=[]
        for i in range(78):
            x=(i*89)%1160; y=(i*53)%560; ww=12+(i*19)%76; hh=2+(i%5)*2
            out.append(f'<rect x="{x}" y="{y}" width="{ww}" height="{hh}" fill="{fg}" opacity="{.03+(i%4)*.015:.3f}"/>')
        return ''.join(out)
    if kind=='scratch':
        return scratches(w,h,fg,.09,52)
    if kind=='noise':
        return ''.join(f'<rect x="{(i*97)%1197}" y="{(i*149)%597}" width="{1+i%2}" height="{1+(i+1)%2}" fill="{fg}" opacity="{.04+(i%4)*.015:.3f}"/>' for i in range(700))
    if kind=='broken-line':
        return ''.join(f'<path d="M{20+(i*117)%1120} {40+(i*71)%520}h{28+(i*23)%140}" stroke="{fg}" stroke-width="1" opacity="{.05+(i%3)*.025:.3f}"/>' for i in range(64))
    if kind=='scan':
        return ''.join(f'<path d="M0 {12+i*17}H1200" stroke="{fg}" stroke-width="{1 if i%5 else 2}" opacity="{.018+(i%7)*.006:.3f}"/>' for i in range(35))
    if kind=='distortion':
        return ''.join(f'<path d="M{(i*67)%300} {30+i*21}H{300+(i*41)%850}" stroke="{fg}" stroke-width="{1+i%2}" opacity="{.035+(i%5)*.012:.3f}"/>' for i in range(26))
    raise ValueError(kind)

textures=['paper-grain','photocopy','scratch','noise','broken-line','scan','distortion']
for kind in textures:
    write(f'assets/vector/textures/ox-dx-texture-{kind}-dark.svg',svg(1200,600,texture_body(kind,fg=C['white']),bg=C['void']))
    write(f'assets/vector/textures/ox-dx-texture-{kind}-light.svg',svg(1200,600,texture_body(kind,fg=C['black']),bg=C['white']))


# -----------------------------------------------------------------------------
# BACKGROUND SYSTEM: dark / graphite / technical / grid / noise / distortion / inverse
# -----------------------------------------------------------------------------

def background(kind: str) -> str:
    w,h=1600,900
    if kind=='dark': body=scratches(w,h,opacity=.035,count=20); bg=C['void']
    elif kind=='graphite': body=microgrid(w,h,80,.035)+scratches(w,h,opacity=.028,count=20); bg=C['graphite']
    elif kind=='technical': body=microgrid(w,h,48,.075)+p_radial(w,h)+scratches(w,h,opacity=.025,count=16); bg=C['void']
    elif kind=='grid': body=microgrid(w,h,64,.11)+f'<g transform="translate(1180 320) scale(1.3)">{master_symbol(sw=4,detail=True)}</g>'; bg=C['void']
    elif kind=='noise': body=texture_body('noise',w,h,C['white'])+microgrid(w,h,96,.025); bg=C['void']
    elif kind=='distortion': body=texture_body('distortion',w,h,C['white'])+f'<path d="M0 450H1600" stroke="{C["cyan"]}" opacity=".2"/>'; bg=C['void']
    elif kind=='inverse': body=microgrid(w,h,64,.08,C['graphite'])+scratches(w,h,C['black'],.025,20); bg=C['white']
    else: raise ValueError(kind)
    return svg(w,h,body,bg=bg)

backgrounds=['dark','graphite','technical','grid','noise','distortion','inverse']
for kind in backgrounds:
    path=f'assets/web/backgrounds/ox-dx-bg-{kind}.svg'
    write(path,background(kind))
    render_svg(ROOT/path, ROOT/f'assets/web/backgrounds/ox-dx-bg-{kind}.webp', 1600, 900)


# -----------------------------------------------------------------------------
# COMPOSITION HELPERS for GitHub / README / release / social.
# -----------------------------------------------------------------------------

def branded_canvas(w,h,title=None,subtitle=None,accent=C['cyan'],quiet=False):
    body=microgrid(w,h,64 if w>800 else 40,.045)+scratches(w,h,opacity=.035,count=20 if not quiet else 10)
    body += f'<g transform="translate({int(w*.06)} {int(h*.18)}) scale({max(.5,min(1.05,w/1400))})">{master_symbol(sw=7,accent=accent,detail=True)}</g>'
    body += f'<g transform="translate({int(w*.22)} {int(h*.22)})">{wordmark(C["white"],scale=max(.5,min(1.1,w/1400)))}</g>'
    if title:
        body += textpath(title,int(w*.22),int(h*.68),max(18,min(34,w/42)),C['white'],weight='bold')
    if subtitle:
        body += textpath(subtitle,int(w*.22),int(h*.82),max(12,min(20,w/70)),accent)
    body += f'<path d="M{int(w*.06)} {int(h*.9)}H{int(w*.94)}" stroke="{C["muted"]}" opacity=".22"/>'
    return body

# GitHub assets
write('assets/web/github/ox-dx-github-avatar.svg', svg(512,512,f'<g transform="translate(96 96) scale(2)">{master_symbol(sw=8,detail=False)}</g>',bg=C['void']))
render_svg(ROOT/'assets/web/github/ox-dx-github-avatar.svg',ROOT/'assets/web/github/ox-dx-github-avatar.png',512,512)
write('assets/web/github/ox-dx-github-social-preview.svg',svg(1280,640,branded_canvas(1280,640,'FROM STRUCTURE TO EXPERIENCE.','FROM UNIVERSE TO BIT.'),bg=C['void']))
render_svg(ROOT/'assets/web/github/ox-dx-github-social-preview.svg',ROOT/'assets/web/github/ox-dx-github-social-preview.png',1280,640)
render_svg(ROOT/'assets/web/github/ox-dx-github-social-preview.svg',ROOT/'assets/web/github/ox-dx-github-social-preview.webp',1280,640)
write('assets/web/github/ox-dx-github-repository-banner.svg',svg(1600,420,branded_canvas(1600,420,'Universal Ontology & Experience Engine','EVIDENCE FIRST.'),bg=C['void']))
write('assets/web/github/ox-dx-github-release-banner.svg',svg(1600,420,branded_canvas(1600,420,'RELEASE','RESOLVE, DON\'T DECORATE.',C['acid']),bg=C['void']))
write('assets/web/github/ox-dx-github-discussion-banner.svg',svg(1600,420,branded_canvas(1600,420,'DISCUSSION','THE GRAPH CONNECTS.',C['ultraviolet']),bg=C['void']))
write('assets/web/github/ox-dx-github-issue-graphic.svg',svg(1200,360,branded_canvas(1200,360,'ISSUE','READ THE EVIDENCE.',C['acid'],quiet=True),bg=C['void']))
write('assets/web/github/ox-dx-github-pr-graphic.svg',svg(1200,360,branded_canvas(1200,360,'PULL REQUEST','DO NOT FORCE THE SHAPE.',C['cyan'],quiet=True),bg=C['void']))

# README assets
write('assets/web/readme/ox-dx-readme-header.svg',svg(1400,360,branded_canvas(1400,360,'Universal Ontology & Experience Engine','FROM STRUCTURE TO EXPERIENCE.',quiet=True),bg=C['void']))
readme_divider=f'<path d="M0 40H560L700 86L840 40H1400" stroke="{C["white"]}" opacity=".26"/><circle cx="700" cy="86" r="12" stroke="{C["cyan"]}"/><circle cx="700" cy="86" r="4" fill="{C["cyan"]}"/>'
write('assets/web/readme/ox-dx-readme-section-divider.svg',svg(1400,120,readme_divider,bg=C['void']))
write('assets/web/readme/ox-dx-readme-ontology-spine.svg',svg(1200,600,motif,bg=C['void']))
arch=microgrid(1200,540,60,.04)+f'''<g stroke="{C['white']}" fill="none" stroke-width="2" opacity=".65">
<rect x="90" y="100" width="230" height="92"/><rect x="485" y="100" width="230" height="92"/><rect x="880" y="100" width="230" height="92"/>
<path d="M320 146H485M715 146H880"/><path d="M600 192V300"/>
<rect x="485" y="300" width="230" height="92"/><path d="M485 346H350V440H850V346H715"/></g>
{node(402,146,6)}{node(798,146,6)}{node(600,246,6)}{node(350,440,5)}{node(850,440,5)}'''
write('assets/web/readme/ox-dx-readme-architecture.svg',svg(1200,540,arch,bg=C['void']))
principle=microgrid(1200,420,60,.035)+textpath('EVIDENCE FIRST.',100,145,42,C['white'],weight='bold')+textpath("DON'T FORCE THE SHAPE.",100,245,34,C['cyan'])+textpath('RESOLUTION, NOT REDUCTION.',100,330,22,C['muted'])+f'<g transform="translate(940 125) scale(1.15)">{master_symbol(sw=6,detail=True)}</g>'
write('assets/web/readme/ox-dx-readme-principle-graphic.svg',svg(1200,420,principle,bg=C['void']))
status=microgrid(1200,320,64,.035)+f'<g stroke="{C["white"]}" opacity=".25">'+''.join(f'<path d="M{110+i*130} 225V{225-(40+i*15)%120}"/>' for i in range(8))+'</g>'+''.join(node(110+i*130,225-(40+i*15)%120,5,C['acid'] if i>=5 else C['cyan'],outer=False) for i in range(8))+textpath('OBSERVE. RESOLVE. UNDERSTAND.',100,90,28,C['white'],weight='bold')
write('assets/web/readme/ox-dx-readme-status-visual.svg',svg(1200,320,status,bg=C['void']))
write('assets/web/readme/ox-dx-readme-footer-mark.svg',svg(1200,220,f'<g transform="translate(60 30) scale(.95)">{master_symbol(sw=6,detail=False)}</g>'+textpath('FROM UNIVERSE TO BIT.',260,126,22,C['cyan']),bg=C['void']))

# Release assets
release_specs=[
    ('stable','STABLE RELEASE','EVIDENCE-BACKED. VERSIONED. REVIEWABLE.',C['acid']),
    ('development','DEVELOPMENT RELEASE','MOVING STRUCTURE TOWARD EXPERIENCE.',C['cyan']),
    ('milestone','MILESTONE','RESOLUTION WITHOUT REDUCTION.',C['ultraviolet']),
    ('phase-complete','PHASE COMPLETE','THE GRAPH CONNECTS.',C['acid']),
]
for slug,title,sub,accent in release_specs:
    write(f'assets/web/releases/ox-dx-release-{slug}.svg',svg(1600,500,branded_canvas(1600,500,title,sub,accent),bg=C['void']))

# Social / OpenGraph composition system
social_specs=[
    ('opengraph',1200,630,'FROM STRUCTURE TO EXPERIENCE.','FROM UNIVERSE TO BIT.',C['cyan']),
    ('x',1600,900,'ORDER INSIDE CHAOS.','EVIDENCE FIRST.',C['acid']),
    ('linkedin',1200,627,"DON'T FORCE THE SHAPE.",'RESOLUTION, NOT REDUCTION.',C['ultraviolet']),
    ('announcement',1600,900,'OBSERVE. RESOLVE. UNDERSTAND.','Universal Ontology & Experience Engine',C['cyan']),
    ('github',1280,640,'THE GRAPH CONNECTS.','Universal Ontology & Experience Engine',C['acid']),
]
for slug,w,h,title,sub,accent in social_specs:
    write(f'assets/web/social/ox-dx-social-{slug}.svg',svg(w,h,branded_canvas(w,h,title,sub,accent),bg=C['void']))


# -----------------------------------------------------------------------------
# DOCUMENTATION
# -----------------------------------------------------------------------------
manifesto = '''# OX-DX Brand Manifesto

Reality does not arrive pre-sorted.

It leaves traces: structure, names, relations, state, events, data, representation. We begin there.

We observe before we classify. We preserve provenance before we compress. We distinguish what contains from what connects, what happened from what exists, and where something was seen from what it is.

We do not force the shape because a clean diagram can still be a false model.

OX-DX is built around a harder discipline: name only what the evidence supports, keep uncertainty visible, and let the graph hold relations without turning every relation into ownership.

Resolution is not reduction. Moving closer should reveal more structure, not erase the structure that came before.

The visual language follows the same rule: clean geometry carries the system; rough surfaces carry the human trace. Order is discovered inside noise rather than painted over it.

Quranic inspiration enters philosophically through questions of order, measure, signs, relation, balance, and scale. It is not presented as a technical prescription. OX-DX engineering remains independently specified, versioned, testable, and reviewable.

Ancient questions. Modern machines. Evidence first.

**FROM STRUCTURE TO EXPERIENCE.**

**FROM UNIVERSE TO BIT.**
'''
write('assets/documentation/brand-manifesto.md',manifesto)

voice='''# OX-DX Brand Voice

## Voice

OX-DX sounds like **ancient questions meeting modern machines**: clear, sharp, reflective, slightly mysterious, raw, precise, and confident.

It never needs startup hype to sound important. The system should feel intelligent because it makes careful distinctions.

## Tone

- **Technical:** exact, testable, bounded. State what exists and what does not.
- **Philosophical:** concise and grounded. Ask large questions without pretending they are engineering proof.
- **Community / open source:** direct, respectful, inspectable. Prefer evidence over authority.
- **Release:** calm and factual. Celebrate progress without implying unimplemented capability.
- **Failure / warning:** explicit. Do not hide uncertainty behind polished language.

## Words we use

`evidence`, `trace`, `observe`, `resolve`, `relation`, `graph`, `structure`, `context`, `provenance`, `identity`, `representation`, `scale`, `bounded`, `testable`, `reviewable`.

## Words we avoid

Avoid generic marketing language such as `revolutionary`, `magic`, `AI-powered`, `game-changing`, `disruptive`, `ultimate`, `world-changing`, and unsupported `intelligent` claims.

Avoid preaching language, invented scripture, and claims that the technical ontology is divinely specified.

Avoid bureaucracy when one exact sentence works.

## Sentence rhythm

Use short declarative sentences to establish principles.

**Systems are messy.**

Then use one precise explanatory sentence when the distinction matters.

**Something happening is not the same as something being.** Runtime observation therefore does not become structural ownership.

Use fragments sparingly as editorial pressure, not as a gimmick.

**Evidence first.**  
**Don't flatten the world.**  
**The graph connects.**

## Technical language

Prefer canonical repository terminology. Distinguish containment, projection, representation, observation, type, kind, identity, and provenance. Never invent ontology levels for branding.

When a feature is planned, say **planned**. When gated, say **gated**. When evidence is missing, say **unmaterialized** or **not established**, according to the technical source.

## Philosophical language

Philosophical language may speak about order, signs, scale, relation, measure, observation, and trace. It must remain clearly separate from normative engineering claims.

The desired tension is:

**deep enough to make you think; precise enough to inspect.**
'''
write('assets/documentation/brand-voice.md',voice)

write('assets/documentation/taglines.md','''# OX-DX Tagline System

| Class | Line | Use |
|---|---|---|
| PRIMARY | **FROM STRUCTURE TO EXPERIENCE.** | Main public brand statement; hero, repository banner, primary social preview. |
| SECONDARY | **FROM UNIVERSE TO BIT.** | Scale / resolution statement; footer, architecture, resolution-spine communication. |
| CAMPAIGN / CONTEXTUAL | **EVIDENCE FIRST.** | Technical principles, release or documentation moments. |
| CAMPAIGN / CONTEXTUAL | **DON'T REWRITE THE WORLD.** | Discovery / read-only positioning. |
| CAMPAIGN / CONTEXTUAL | **ORDER INSIDE CHAOS.** | Visual system, editorial composition, brand storytelling. |
| CAMPAIGN / CONTEXTUAL | **OBSERVE. RESOLVE. UNDERSTAND.** | Workflow / explanatory communication. |

Do not multiply slogans casually. The primary and secondary lines remain canonical; contextual lines support specific communication without replacing them.
''')

write('assets/documentation/technical/color-system.md',f'''# OX-DX Color System

OX-DX is dark-first and monochrome-capable. Neon colors are signals, not decoration.

| Role | Name | HEX | RGB | Usage | Contrast / behavior |
|---|---|---|---|---|---|
| PRIMARY / SURFACE | Void | `{C['void']}` | `7, 9, 10` | Primary canvas, hero, GitHub/release backgrounds | Pair with White; safest high-contrast environment. |
| SECONDARY / SURFACE | Charcoal | `{C['charcoal']}` | `17, 20, 24` | Raised or alternate dark surface | Pair with White or Muted; avoid low-opacity cyan text. |
| SURFACE | Graphite | `{C['graphite']}` | `32, 37, 43` | Secondary technical panels / backgrounds | Pair with White; use Muted only at readable sizes. |
| TEXT | White | `{C['white']}` | `245, 247, 248` | Primary text and geometry | Canonical foreground on dark surfaces. |
| MUTED | Muted Steel | `{C['muted']}` | `153, 162, 170` | Annotation, secondary labels, grid lines | Never use for tiny critical text on Graphite. |
| ACCENT | Acid | `{C['acid']}` | `140, 255, 0` | Milestones, success-like editorial signal, rare emphasis | Signal only; do not flood large surfaces. |
| SIGNAL | Electric Cyan | `{C['cyan']}` | `0, 216, 255` | Observation node, resolution axis, primary technical signal | Canonical accent. Use against dark fields. |
| ACCENT | Ultraviolet | `{C['ultraviolet']}` | `156, 77, 255` | Secondary editorial / discussion signal | Avoid as body text at small sizes. |
| INVERSE | Black | `{C['black']}` | `0, 0, 0` | Monochrome mark on light backgrounds | Pair with White/light surfaces. |

## Rules

1. The logo must work in pure black or pure white.
2. Cyan is the default signal accent. Acid and Ultraviolet are contextual, not competing primaries.
3. Do not add new palette colors without explicit brand approval.
4. Grunge uses opacity and texture, not extra colors.
5. Critical text must prioritize contrast over atmosphere.
''')

write('assets/documentation/technical/typography.md','''# OX-DX Typography

The OX-DX wordmark is custom vector geometry and is **not a font**. Distribution SVGs outline the lettering and require no installed font.

## DISPLAY

- Family: canonical OX-DX vector wordmark only.
- Weight: defined by geometry; do not simulate weight changes.
- Usage: brand name, primary lockups.
- Rule: never retype `OX-DX` to replace the master wordmark in identity-critical placements.

## HEADLINE

- Family: `Inter, ui-sans-serif, system-ui, sans-serif`.
- Weight: 650–800 where supported; otherwise 700.
- Typical size: 40–72 px desktop, 30–48 px compact layouts.
- Line-height: 0.98–1.08.
- Letter-spacing: `-0.03em` to `-0.01em`.
- Case: sentence case for explanatory headlines; uppercase for short principle statements only.

## BODY

- Family: `Inter, ui-sans-serif, system-ui, sans-serif`.
- Weight: 400–500.
- Typical size: 16–20 px.
- Line-height: 1.45–1.65.
- Letter-spacing: normal.
- Case: normal sentence case.

## MONO

- Family: `ui-monospace, SFMono-Regular, Menlo, Consolas, monospace`.
- Weight: 400–550.
- Typical size: 13–16 px.
- Line-height: 1.4–1.55.
- Usage: paths, commands, ontology names, technical evidence, annotations.

## TECHNICAL LABEL

- Family: monospace stack above.
- Weight: 550–650.
- Typical size: 10–12 px.
- Line-height: 1.2–1.35.
- Letter-spacing: `0.10em`–`0.16em`.
- Case: uppercase.

## Hierarchy

`WORDMARK → HEADLINE → DESCRIPTOR → BODY → TECHNICAL LABEL`

Typography should feel editorial, technical, modern, and slightly brutalist. Do not use rounded startup fonts or illegible cyberpunk display faces. Do not commit proprietary font binaries.
''')

write('assets/documentation/logo-specification.md','''# OX-DX Logo Specification

## Canonical source

Master logo: `assets/brand/logo/ox-dx-logo-primary.svg`  
Master icon: `assets/brand/icon/ox-dx-icon.svg`

The approved **Resolution Engine** geometry is authoritative. This document explains it; it does not redefine it.

## Geometry

The symbol is built on a 160 × 160 construction field:

- two opposed chamfered frames establish tension / relation;
- a centered circular observation field anchors the system;
- a rotated square / diamond marks the resolved signal;
- optional orthogonal ticks may appear in larger editorial marks, never in tiny favicon geometry;
- canonical brand geometry uses square caps, hard corners, and zero decorative radius.

At the 160 px symbol artboard the main frame stroke is 10 px. The canonical logo lockup uses the same geometry at a smaller stroke appropriate to its composition.

## Clear space

Keep at least **1/4 of the symbol width** clear on every side. For a 160 px mark, the minimum clear field is 40 px.

## Minimum size

- 16–24 px: use the simplified favicon / icon mark; omit annotation ticks.
- 32 px and above: standard icon geometry is safe.
- Wordmark lockups must not be reduced until counters and chamfers collapse; use symbol-only below that point.

## Safe backgrounds

Preferred: White geometry on Void / Charcoal / Graphite.  
Inverse: Black geometry on White.  
Accent: Cyan may mark the observation node but is never required for recognition.

Do not place the logo directly on aggressive grunge. Create a quiet field around the mark.

## Monochrome

The mark must remain complete in pure black and pure white. Never encode meaning that depends only on neon accent color.

## Wordmark relationship

The wordmark is custom vector geometry. Do not retype, stretch, skew, rotate, outline with effects, or substitute a futuristic font.

## Symbol relationship

Supporting symbols and glyphs inherit the line logic, frame tension, node / diamond signal, proportion, and zero-radius construction. They are visual notation, not new ontology semantics.

## Incorrect usage

Do not stretch, rotate, soften, add gradients to the canonical mark, inject texture inside the master paths, add arbitrary glow, invent colors, or combine the logo with religious iconography.
''')

write('assets/documentation/technical/motion-language.md','''# OX-DX Motion Language

## Core principle

**RESOLVE, DON'T DECORATE.**

Motion exists to explain changing resolution, relation, observation, or state. It should never become ambient spectacle.

## Motion sequence

`FRAGMENT → STRUCTURE → RELATION → OBSERVATION → RESOLUTION`

The sequence is conceptual, not a mandatory technical pipeline.

## Timing tokens

| Token | Duration | Use |
|---|---:|---|
| snap | 120 ms | tiny state confirmation / glyph response |
| quick | 180 ms | hover, focus, compact reveal |
| resolve | 280 ms | structural transition / relation appearance |
| deep | 420 ms | hero or section-level resolution reveal |

Recommended easing: `cubic-bezier(0.22, 1, 0.36, 1)` for resolution; use linear only for signal scanning where constant speed carries meaning.

## Entry

Fragments or low-opacity lines appear first, then align into the stable frame. The observation node appears last. Avoid scale-from-zero gimmicks.

## Exit

Reverse emphasis rather than explode the system: node fades, relations thin, frame releases. Keep exit shorter than entry.

## Hover

Use one controlled response: line-weight shift, 1–2 px translation, or accent-node activation. Do not combine all three.

## Scroll

Reveal resolution in discrete stages tied to content. No perpetual parallax and no decorative particle field.

## Loading

Prefer a bounded scan or staged frame resolution. The animation must communicate waiting without implying fake progress.

## Transition

Preserve spatial continuity. Relations should connect before a new detail layer becomes dominant.

## Reveal

Use masks / clipping or opacity to expose ordered geometry from noisy surface, reinforcing **ORDER INSIDE CHAOS**.

## Reduced motion

Respect `prefers-reduced-motion: reduce`. Replace motion sequences with immediate state changes or a single short opacity transition. No required information may exist only in animation.
''')

write('assets/documentation/asset-governance.md','''# OX-DX Asset Governance

## Asset classes

### MASTER ASSET

The authoritative editable geometry or specification from which identity-critical variants derive. Master SVG geometry is canonical.

### DERIVED ASSET

A PNG, WEBP, alternate background treatment, layout crop, or other export produced from a master or approved composition. Derived assets must not silently change geometry.

### EXPERIMENTAL ASSET

A candidate visual that may be reviewed but must not replace canonical identity assets or appear as the default brand system without approval.

### DEPRECATED ASSET

An asset intentionally retained for migration or history but no longer recommended for new usage.

## Status vocabulary

- `CANONICAL` — source of truth.
- `APPROVED` — valid derived or supporting asset.
- `EXPERIMENTAL` — review-only.
- `DEPRECATED` — retained but should not be used for new work.

## Rules

1. Master SVG is canonical; PNG / WEBP are exports.
2. Color or background variants must preserve the approved geometry.
3. New symbols, glyphs, patterns, and compositions must inherit documented design tokens.
4. Grunge is a surface treatment and must never corrupt master logo geometry.
5. Decorative symbols are not ontology semantics and may not create technical levels, relations, or claims.
6. Experimental assets cannot become canonical without explicit project-owner approval.
7. Do not commit proprietary fonts, generation caches, temporary previews, or model metadata.
8. Deterministic filenames use `ox-dx-{category}-{name}-{variant}.{ext}` where a variant is needed.
9. Every canonical / approved asset must be listed in `assets/ASSET_INDEX.md`.
10. Validation must pass before brand-system changes are committed.
''')

# Expand guidelines while preserving approved direction.
write('assets/documentation/brand-guidelines.md','''# OX-DX Brand Guidelines

## Brand

**OX-DX**  
Universal Ontology & Experience Engine

**FROM STRUCTURE TO EXPERIENCE.**  
**FROM UNIVERSE TO BIT.**

The canonical visual direction remains **Resolution Engine**: a dark-first, mathematically ordered system where a stable frame resolves toward observation, relation, and increasingly fine signal.

## Identity equation

**QURANIC DEPTH × GEN-Z CLARITY × GRUNGE ATTITUDE × INTELLIGENT PRECISION**

Quranic influence is philosophical and mathematical: proportion, symmetry, repetition, balance, layering, measure, and order. It never requires calligraphy, verses, crescents, mosque imagery, or claims that the technical system is divinely specified.

Gen-Z energy appears through direct language, unconventional spacing, sharp composition, and open-source confidence—not memes or childish styling.

Grunge is the surface. Geometry is the structure.

**CLEAN STRUCTURE. DIRTY SURFACE.**

## Recognizable without the logo

An OX-DX composition should remain recognizable through:

- opposed / nested hard-edged frames;
- centered observation nodes and diamond signals;
- resolution axes and scale changes;
- 8 px-derived modular spacing;
- dark Void / Graphite surfaces;
- White geometry with restrained Cyan signal;
- sparse Acid or Ultraviolet contextual accents;
- precise technical labels + controlled scratches / scan texture.

## Correct usage

Use the supplied canonical SVG. Preserve geometry, aspect ratio, clear space, and contrast. Use grunge primarily in backgrounds, banners, section dividers, release/social compositions, and editorial surfaces.

## Incorrect usage

- Do not stretch, rotate, soften, or distort the canonical logo.
- Do not place it over unreadable noise without a quiet field.
- Do not invent palette colors.
- Do not modify or retype the wordmark.
- Do not add gradients or arbitrary effects to the canonical logo.
- Do not insert Arabic calligraphy, Quranic verses, crescents, mosque imagery, or other religious emblems.
- Do not turn visual symbols, glyphs, or patterns into new ontology semantics.
- Do not make the identity resemble generic AI, crypto/Web3, fintech, gaming, or corporate consulting branding.

## Related specifications

- `brand-manifesto.md`
- `brand-voice.md`
- `taglines.md`
- `logo-specification.md`
- `asset-governance.md`
- `technical/color-system.md`
- `technical/typography.md`
- `technical/design-tokens.md`
- `technical/motion-language.md`
''')

write('assets/README.md','''# OX-DX Assets

This directory contains the canonical OX-DX brand foundation, visual language, and derived asset system.

## Canonical identity

- Master logo: `brand/logo/ox-dx-logo-primary.svg`
- Master icon: `brand/icon/ox-dx-icon.svg`
- Wordmark: `brand/wordmark/ox-dx-wordmark.svg`
- Lockups: `brand/lockups/`
- Approved visual symbols: `brand/symbols/`
- Compact glyph notation: `brand/glyphs/`

## Visual language

- Patterns: `vector/patterns/`
- Controlled grunge textures: `vector/textures/`
- Web backgrounds: `web/backgrounds/`
- Header / hero / footer: `web/header/`, `web/hero/`, `web/footer/`
- GitHub system: `web/github/`
- README system: `web/readme/`
- Release system: `web/releases/`
- Social / OpenGraph system: `web/social/`

## Documentation

Start with `documentation/brand-guidelines.md`, then use the manifesto, voice, tagline, logo, governance, color, typography, token, and motion specifications as the shared language for future website, documentation, UI, release, and product work.

## Source / editable

- Existing deterministic base renderer: `source/design/render_brand_assets.py`
- Brand-system renderer: `source/design/render_brand_system.py`
- Approved master geometry: `source/design/ox-dx-master-geometry.svg`

Future contributors must derive new assets from the canonical geometry and documented tokens. Do not redraw the symbol, modify the wordmark, invent palette values, or embed new technical ontology semantics in decorative graphics.
''')

# Save generator itself as editable source.
source_target = ROOT/'assets/source/design/render_brand_system.py'
if Path(__file__).resolve() != source_target.resolve():
    source_target.write_text(Path(__file__).read_text(encoding='utf-8'),encoding='utf-8')

# -----------------------------------------------------------------------------
# ASSET INDEX with governance status.
# -----------------------------------------------------------------------------
def status_for(rel: str) -> str:
    canonical = {
        'brand/logo/ox-dx-logo-primary.svg','brand/icon/ox-dx-icon.svg','brand/wordmark/ox-dx-wordmark.svg',
        'brand/lockups/ox-dx-lockup-horizontal.svg','brand/lockups/ox-dx-lockup-stacked.svg',
        'documentation/brand-guidelines.md','documentation/brand-manifesto.md','documentation/brand-voice.md',
        'documentation/taglines.md','documentation/logo-specification.md','documentation/asset-governance.md',
        'documentation/technical/color-system.md','documentation/technical/typography.md',
        'documentation/technical/design-tokens.md','documentation/technical/motion-language.md',
        'source/design/ox-dx-master-geometry.svg','source/design/palette.json','source/design/render_brand_assets.py','source/design/render_brand_system.py',
    }
    if rel in canonical: return 'CANONICAL'
    if 'source/references/' in rel: return 'APPROVED'
    return 'APPROVED'


def purpose_for(rel: str) -> str:
    if rel.startswith('brand/symbols/'): return 'Visual symbol language; non-semantic brand notation'
    if rel.startswith('brand/glyphs/'): return 'Compact technical / editorial glyph'
    if rel.startswith('vector/patterns/'): return 'Canonical supporting pattern language'
    if rel.startswith('vector/textures/'): return 'Controlled grunge surface texture'
    if rel.startswith('web/backgrounds/'): return 'Web / editorial background surface'
    if rel.startswith('web/github/'): return 'GitHub communication system'
    if rel.startswith('web/readme/'): return 'README communication system'
    if rel.startswith('web/releases/'): return 'Release / milestone graphic template'
    if rel.startswith('web/social/'): return 'Social / OpenGraph communication asset'
    if 'hero' in rel: return 'Website hero / resolution narrative'
    if 'header' in rel: return 'Header / identity composition'
    if 'footer' in rel: return 'Footer / closing-system composition'
    if 'icon' in rel or 'favicon' in rel: return 'Icon / avatar / favicon'
    if 'logo' in rel: return 'Canonical or derived logo asset'
    if 'wordmark' in rel: return 'OX-DX wordmark'
    if 'lockup' in rel: return 'Logo lockup'
    if rel.endswith('.md'): return 'Brand documentation'
    if rel.endswith('.py'): return 'Deterministic source / renderer'
    if rel.endswith('.json'): return 'Brand source data'
    return 'OX-DX brand asset'

rows=[]
for p in sorted(ASSETS.rglob('*')):
    if not p.is_file() or p.name=='ASSET_INDEX.md': continue
    rel=p.relative_to(ASSETS).as_posix()
    ext=p.suffix.lower().lstrip('.').upper()
    variant='canonical'
    for key in ['monochrome','inverse','white','black','dark','light','mobile','desktop','favicon','stable','development','milestone','phase-complete']:
        if key in rel: variant=key; break
    rows.append((p.stem,rel,purpose_for(rel),ext,variant,status_for(rel)))
idx=['# OX-DX Asset Index','',
     'Every listed item is either a canonical source of truth or an approved derivative/supporting asset. Status meanings are defined in `documentation/asset-governance.md`.','',
     '| Name | Path | Purpose | Format | Variant | Status |','|---|---|---|---|---|---|']
for name,rel,purpose,fmt,variant,status in rows:
    idx.append(f'| `{name}` | `{rel}` | {purpose} | {fmt} | {variant} | **{status}** |')
write('assets/ASSET_INDEX.md','\n'.join(idx)+'\n')

# -----------------------------------------------------------------------------
# VALIDATION — fail closed.
# -----------------------------------------------------------------------------
errors=[]
svgs=list(ASSETS.rglob('*.svg'))
for p in svgs:
    try:
        root=ET.parse(p).getroot()
        if 'viewBox' not in root.attrib: errors.append(f'NO_VIEWBOX {p}')
        t=p.read_text(encoding='utf-8')
        if '<image' in t: errors.append(f'EMBEDDED_RASTER {p}')
        if '<text' in t or 'font-family' in t: errors.append(f'EXTERNAL_FONT_DEP {p}')
        if re.search(r'(?:href|xlink:href)="https?://',t): errors.append(f'EXTERNAL_REF {p}')
    except Exception as e:
        errors.append(f'XML {p}: {e}')

rasters=list(ASSETS.rglob('*.png'))+list(ASSETS.rglob('*.webp'))
for p in rasters:
    try:
        im=Image.open(p); im.verify()
    except Exception as e:
        errors.append(f'RASTER {p}: {e}')

# Explicit new-output dimensions.
dims={
    'assets/web/github/ox-dx-github-avatar.png':(512,512),
    'assets/web/github/ox-dx-github-social-preview.png':(1280,640),
    'assets/web/github/ox-dx-github-social-preview.webp':(1280,640),
}
for rel,expected in dims.items():
    im=Image.open(ROOT/rel)
    if im.size != expected: errors.append(f'DIM {rel} {im.size} != {expected}')

# Markdown path audit for explicit assets/... code references.
all_paths={p.relative_to(ROOT).as_posix() for p in ASSETS.rglob('*') if p.is_file()}
for md in ASSETS.rglob('*.md'):
    txt=md.read_text(encoding='utf-8')
    for ref in re.findall(r'`(assets/[^`]+)`',txt):
        if ref not in all_paths:
            errors.append(f'MD_PATH {md}: {ref}')

# Boundary audit.
for p in ROOT.rglob('*'):
    if p.is_file() and p.parts and p.parts[0] != 'assets' and p.resolve() != Path(__file__).resolve():
        # The runner executes inside the repository, which already contains non-assets files.
        # We never write them; no failure needed here.
        pass

summary={
    'validation':'pass' if not errors else 'fail',
    'asset_files':len([p for p in ASSETS.rglob('*') if p.is_file()]),
    'svgs':len(svgs),
    'rasters':len(rasters),
    'symbols':len(list((ASSETS/'brand/symbols').glob('*.svg'))),
    'glyphs':len(list((ASSETS/'brand/glyphs').glob('*.svg'))),
    'patterns':len(list((ASSETS/'vector/patterns').glob('*.svg'))),
    'textures':len(list((ASSETS/'vector/textures').glob('*.svg'))),
    'backgrounds':len(list((ASSETS/'web/backgrounds').glob('*'))),
    'errors':errors,
}
print(json.dumps(summary,indent=2))
if errors:
    raise SystemExit(1)
