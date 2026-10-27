#!/usr/bin/env python3
"""Import the UI contract from a local PCL2-CE checkout, without modifying it."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
XAML = '{http://schemas.microsoft.com/winfx/2006/xaml}'

def local(value):
    return value.rsplit('}', 1)[-1]

def component_name(value):
    return 'P' + value[2:] if value.startswith('My') else value

def convert(element, inherited=None):
    inherited = inherited or {}
    styles = {key: dict(value) for key, value in inherited.items()}
    for resource in element:
        if local(resource.tag).endswith(".Resources"):
            for style in resource:
                if local(style.tag) == "Style":
                    kind = style.get("TargetType", "").replace("{x:Type ", "").rstrip("}").split(":")[-1]
                    styles.setdefault(kind, {}).update({setter.get("Property"): setter.get("Value") for setter in style if local(setter.tag) == "Setter" and setter.get("Value")})
    attrs = {local(k): v for k, v in element.attrib.items()
             if not k.startswith(('{http://schemas.microsoft.com/expression/',
                                  '{http://schemas.openxmlformats.org/'))}
    attrs = {**styles.get(local(element.tag), {}), **attrs}
    node = {'type': component_name(local(element.tag)), 'attrs': attrs}
    if element.text and element.text.strip():
        node['text'] = element.text.strip()
    children = [convert(child, styles) for child in element]
    if children:
        node['children'] = children
    return node

def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    content = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    if not path.exists() or path.read_text() != content:
        path.write_text(content)

def copy_changed(source, target):
    source, target = Path(source), Path(target)
    if not target.exists() or source.read_bytes() != target.read_bytes():
        shutil.copyfile(source, target)
    return str(target)

def path_style(element, key):
    if key != 'Data':
        return {'fill': 'currentColor'}
    stroke = element.get('Stroke')
    style = {'fill': 'currentColor' if element.get('Fill') else 'none'}
    if stroke:
        style['stroke'] = 'currentColor'
        style['stroke-width'] = element.get('StrokeThickness', '1')
        style['stroke-linecap'] = {'Flat': 'butt', 'Square': 'square', 'Round': 'round'}.get(element.get('StrokeStartLineCap'), 'butt')
        style['stroke-linejoin'] = element.get('StrokeLineJoin', 'Miter').lower()
    elif not element.get('Fill'):
        style['fill'] = 'currentColor'
    return style

def sync(source):
    app = source / 'Plain Craft Launcher 2'
    target = ROOT / 'src/assets'
    pages = sorted((app / 'Pages').rglob('*.xaml'))
    assert pages, 'No PCL2-CE pages found'
    sources = {}
    setting_values = {}
    config = (source / 'PCL.Core/App/Config.cs').read_text()
    for match in re.finditer(r'ConfigItem<[^>]+>\("([^"]+)",\s*("(?:\\.|[^"\\])*"|true|false|-?\d+(?:\.\d+)?[dfm]?)\s*[,)]', config):
        raw = match[2]
        try:
            setting_values[match[1]] = json.loads(re.sub(r'(?<=\d)[dfm]$', '', raw))
        except json.JSONDecodeError:
            pass
    page_defaults = {}
    nav_actions = {}
    for page in pages + [app / 'FormMain.xaml']:
        tree = ET.parse(page).getroot()
        name = component_name(page.stem)
        assert name not in sources, f'Duplicate page: {name}'
        sources[name] = {'path': str(page.relative_to(source)),
                         'sha256': hashlib.sha256(page.read_bytes()).hexdigest()}
        write_json(target / 'layouts' / (name + '.json'), convert(tree))
        if page.stem in ('PageSetupLeft', 'PageInstanceLeft', 'PageDownloadLeft', 'PageToolsLeft'):
            actions = {}
            for item in tree.iter():
                if local(item.tag) != 'MyListItem':
                    continue
                button = next((child for child in item.iter() if local(child.tag) == 'MyIconButton' and child.get('ToolTip')), None)
                if button is not None:
                    title = button.get('ToolTip')
                    actions[item.get(XAML + 'Name')] = {'title': title, 'icon': button.get('SvgIcon', 'lucide/refresh-cw'), 'kind': 'initialize' if 'Initialize' in title else 'refresh'}
            nav_actions[name] = actions
        defaults = {}
        for element in tree.iter():
            kind = local(element.tag)
            tag = element.get('Tag', '').split('/')[0]
            control = element.get(XAML + 'Name') or element.get('Name')
            if tag in setting_values and control:
                if kind == 'MyRadioBox':
                    defaults[tag] = str(setting_values[tag])
                elif kind in ('MyCheckBox', 'MySlider', 'MyComboBox', 'MyTextBox'):
                    value = setting_values[tag]
                    defaults[control] = ('0' if value else '1') if kind == 'MyComboBox' and isinstance(value, bool) else str(value) if kind in ('MyComboBox', 'MyTextBox') else value
        if defaults:
            page_defaults[name] = defaults
    write_json(target / 'setting-defaults.json', page_defaults)
    write_json(target / 'nav-actions.json', nav_actions)
    locale_names = []
    for path in sorted((source / 'PCL.Core/App/Localization/Languages').glob('*.xaml')):
        locale = path.stem
        locale_names.append(locale)
        strings = {e.get(XAML + 'Key'): ''.join(e.itertext()) for e in ET.parse(path).getroot()}
        assert strings, f'Empty language resource: {locale}'
        if locale in ('zh-CN', 'en-US'):
            assert strings.get('Main.Tab.Launch'), f'Missing launcher strings: {locale}'
        write_json(target / 'languages' / (locale + '.json'), strings)
    write_json(target / 'language-names.json', locale_names)
    write_json(target / 'pcl-metadata.json', json.loads((app / 'metadata.json').read_text()))
    export_options = []
    def export_form(element, parent=None, indent=False):
        visibility = element.get('Visibility', '')
        match = re.search(r'ElementName=(\w+)', visibility)
        parent = match[1] if match else parent
        indent = indent or element.get('Margin', '').startswith('30,')
        if local(element.tag) == 'MyCheckBox':
            option = next((child for child in element.iter() if local(child.tag) == 'ExportOption'), None)
            if option is not None:
                title = option.get('Title')
                name = element.get(XAML + 'Name') or 'CheckOptions' + title.rsplit('.', 1)[-1].rstrip('}')
                export_options.append({'id': name, 'title': title, 'description': option.get('Description', ''),
                                       'checked': option.get('DefaultChecked') == 'True',
                                       'disabled': element.get('IsEnabled') == 'False', 'indent': indent,
                                       'parent': parent, 'require': 'optifine' if option.get('RequireOptiFine') == 'True' else 'mod' if option.get('RequireModLoader') == 'True' else ''})
        for child in element:
            export_form(child, parent, indent)
    export_form(ET.parse(app / 'Pages/PageInstance/PageInstanceExport.xaml').getroot())
    write_json(target / 'export-options.json', export_options)
    assets = ROOT / 'public/pcl'
    icons = source / 'PCL.Core/UI/Assets/IconPacks'
    icon_names = sorted({p.stem for p in icons.rglob('*.svg')} | {'check', 'chevron-down', 'chevron-up', 'ellipsis', 'clock'})
    def component(name):
        return {'heart-filled': 'Heart', 'grid-2x2': 'Grid2X2'}.get(name, ''.join(part.capitalize() for part in name.split('-')))
    imports = sorted({component(name) for name in icon_names})
    module = '// Generated by scripts/sync-pcl-ui.py. Import named components to keep tree shaking.\n'
    module += 'import {\n  ' + ',\n  '.join(imports) + "\n} from '@lucide/vue'\n\n"
    module += "import type { Component } from 'vue'\n\nexport const pclIcons: Record<string, Component> = {\n"
    module += ''.join(f"  'lucide/{name}': {component(name)},\n" for name in icon_names)
    module += '}\n'
    if not (ROOT / 'src/components/icons/pclIcons.ts').exists() or (ROOT / 'src/components/icons/pclIcons.ts').read_text() != module:
        (ROOT / 'src/components/icons/pclIcons.ts').write_text(module)
    write_json(target / 'icon-names.json', icon_names)
    shutil.copytree(app / 'Images', assets / 'images', dirs_exist_ok=True, copy_function=copy_changed)
    # WPF paths are retained separately as SVGs as well as in their original layout nodes.
    paths = {}
    control_paths = {}
    for page in sorted(app.rglob('*.xaml')):
        for e in ET.parse(page).getroot().iter():
            for key in ('Data', 'Logo', 'Icon'):
                value = e.get(key, '')
                if re.match(r'^(?:F[01]\s*)?[Mm]\s*[-\d.]', value):
                    digest = hashlib.sha256(value.encode()).hexdigest()[:16]
                    paths[digest] = {'data': value, 'source': str(page.relative_to(source)), 'style': path_style(e, key)}
                    name = e.get(XAML + 'Name') or e.get('Name')
                    if name:
                        control_paths[f'{page.stem}.{name}'] = value
    for code in sorted(app.rglob('*.cs')):
        for match in re.finditer(r'"((?:F[01]\s*)?[Mm][0-9.,\s+\-][MmLlHhVvCcSsQqTtAaZz0-9.,\s+\-Ee]+)"', code.read_text(encoding='utf-8-sig')):
            value = match[1]
            digest = hashlib.sha256(value.encode()).hexdigest()[:16]
            paths.setdefault(digest, {'data': value, 'source': str(code.relative_to(source))})
    write_json(target / 'control-geometry.json', control_paths)
    from fontTools.pens.boundsPen import BoundsPen
    from fontTools.svgLib.path import parse_path
    geometry_dir = assets / 'geometry'
    geometry_dir.mkdir(parents=True, exist_ok=True)
    for digest, item in paths.items():
        data = re.sub(r'^F[01]\s*', '', item['data'])
        pen = BoundsPen(None)
        parse_path(data, pen)
        x0, y0, x1, y1 = pen.bounds
        style = item.setdefault('style', {'fill': 'currentColor'})
        padding = float(style.get('stroke-width', '0')) / 2
        x0, y0, x1, y1 = x0 - padding, y0 - padding, x1 + padding, y1 + padding
        viewbox = f'{x0:g} {y0:g} {x1-x0:g} {y1-y0:g}'
        svg = ET.Element('svg', {'xmlns': 'http://www.w3.org/2000/svg', 'viewBox': viewbox})
        ET.SubElement(svg, 'path', {'d': data, **style, 'fill-rule': 'nonzero' if item['data'].startswith('F1') else 'evenodd'})
        svg_path = geometry_dir / f'{digest}.svg'
        content = ET.tostring(svg, encoding='unicode') + '\n'
        if not svg_path.exists() or svg_path.read_text() != content:
            svg_path.write_text(content)
        if item['data'] == control_paths.get('FormMain.ShapeTitleLogo'):
            svg.set('width', '39')
            svg.set('height', '20')
            (target / 'icons/TitleLogo.svg').write_text(ET.tostring(svg, encoding='unicode') + '\n')
        item['viewBox'] = viewbox
    write_json(target / 'vector-paths.json', paths)
    revision = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    write_json(target / 'pcl-manifest.json', {'repository': 'PCL-Community/PCL2-CE', 'revision': revision,
                                        'pages': sources, 'svgCount': len(list(icons.rglob('*.svg'))),
                                        'geometryCount': len(paths)})
    copy_changed(app / 'LICENCE', assets / 'LICENCE')
    print(f'Imported {len(sources)} layouts and {len(paths)} geometries; registered {len(icon_names)} Lucide names from {revision}')

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    args = parser.parse_args()
    sync(args.source.resolve())
