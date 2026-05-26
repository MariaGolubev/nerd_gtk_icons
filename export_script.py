import argparse
import os
import shutil
import re
import fontforge
import json

# ----------------------------
# CLI
# ----------------------------

parser = argparse.ArgumentParser(
    description="Export Nerd Fonts to GTK4 symbolic icons + GResource"
)

parser.add_argument("font", help="Path to the TTF/OTF font file")

parser.add_argument(
    "-o",
    "--output",
    default="resources",
    help="Output root directory (default: resources)"
)

# ----------------------------
# Helpers
# ----------------------------

def normalize_name(name: str) -> str:
    """
    Convert Nerd Font glyph names into GTK-friendly icon names.
    """
    name = name.lower()
    name = name.replace("_", "-")
    name = re.sub(r"[^a-z0-9\-]", "-", name)
    name = re.sub(r"-+", "-", name).strip("-")

    return f"nf-{name}-symbolic"


def clean_svg(path: str):
    """
    Make SVG compatible with GTK symbolic icon style.
    """
    with open(path, "r", encoding="utf-8") as f:
        svg = f.read()

    svg = re.sub(r'fill="[^"]+"', 'fill="currentColor"', svg)
    svg = re.sub(r'<\?xml.*?\?>', '', svg)
    svg = re.sub(r'<!DOCTYPE.*?>', '', svg)

    with open(path, "w", encoding="utf-8") as f:
        f.write(svg.strip())


# ----------------------------
# Main
# ----------------------------

if __name__ == "__main__":
    args = parser.parse_args()

    font = fontforge.open(args.font)

    # Output paths
    icons_dir = os.path.join(args.output, "icons")
    xml_path = os.path.join(args.output, "icons.gresource.xml")
    json_path = os.path.join(args.output, "metadata.json")

    # Clean output directory
    if os.path.exists(args.output):
        shutil.rmtree(args.output)

    os.makedirs(icons_dir, exist_ok=True)

    icons = []

    # ----------------------------
    # DEDUP STORAGE (IMPORTANT FIX)
    # ----------------------------
    seen_names = set()

    # ----------------------------
    # EXPORT GLYPHS
    # ----------------------------

    for glyph in font.glyphs():
        name = glyph.glyphname

        # Skip invalid glyphs
        if (
            not glyph.isWorthOutputting()
            or not name
            or name.startswith(".")
            or name.startswith("uni")
            or name.startswith("u")
        ):
            continue

        safe_name = normalize_name(name)

        if not safe_name:
            continue

        # ❗ prevent duplicate SVG exports (MAIN FIX)
        if safe_name in seen_names:
            continue

        seen_names.add(safe_name)

        codepoint = glyph.unicode
        codepoint_hex = f"{codepoint:04X}" if codepoint != -1 else None

        filename = os.path.join(icons_dir, f"{safe_name}.svg")

        try:
            glyph.export(filename)
            clean_svg(filename)
        except Exception:
            continue

        icons.append({
            "code": codepoint_hex,
            "name": safe_name,
            "file": f"resources/icons/{safe_name}.svg",
            "resource_path": f"/com/nerd/icons/{safe_name}"
        })

    # ----------------------------
    # GENERATE GRESOURCE XML
    # ----------------------------

    xml = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<gresources>',
        '  <gresource prefix="/com/nerd">'
    ]

    # stable ordering (optional but good practice)
    for icon in sorted(icons, key=lambda x: x["name"]):
        xml.append(f'    <file>icons/{icon["name"]}.svg</file>')

    xml += [
        '  </gresource>',
        '</gresources>'
    ]

    with open(xml_path, "w", encoding="utf-8") as f:
        f.write("\n".join(xml))

    # ----------------------------
    # EXPORT METADATA JSON
    # ----------------------------

    with open(json_path, "w", encoding="utf-8") as f:
        json.dump(icons, f, ensure_ascii=False, indent=4)

    print(f"✔ Exported {len(icons)} icons")
    print(f"✔ Icons directory: {icons_dir}")
    print(f"✔ GResource XML: {xml_path}")