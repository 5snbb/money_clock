"""Generate the money_clock source application icon (1024x1024 PNG).

Run from the repository root with Pillow installed:

    python scripts/generate-app-icon.py
"""

import math
from pathlib import Path

from PIL import Image, ImageDraw

SIZE = 1024
SUPERSAMPLE = 4
CANVAS = SIZE * SUPERSAMPLE

BACKGROUND_TOP = (15, 23, 42)
BACKGROUND_BOTTOM = (5, 9, 16)
BRAND = (52, 211, 153)
BRAND_LIGHT = (110, 231, 183)
FOREGROUND = (236, 253, 245)


def mix(start: tuple[int, int, int], end: tuple[int, int, int], ratio: float) -> tuple[int, int, int]:
    return tuple(round(a + (b - a) * ratio) for a, b in zip(start, end))


def build_gradient() -> Image.Image:
    gradient = Image.new("RGB", (CANVAS, CANVAS))
    draw = ImageDraw.Draw(gradient)
    for y in range(CANVAS):
        draw.line([(0, y), (CANVAS, y)], fill=mix(BACKGROUND_TOP, BACKGROUND_BOTTOM, y / CANVAS))
    return gradient


def rounded_mask(radius: int) -> Image.Image:
    mask = Image.new("L", (CANVAS, CANVAS), 0)
    ImageDraw.Draw(mask).rounded_rectangle(
        (0, 0, CANVAS - 1, CANVAS - 1), radius=radius, fill=255
    )
    return mask


def main() -> None:
    canvas = Image.new("RGBA", (CANVAS, CANVAS), (0, 0, 0, 0))
    canvas.paste(build_gradient(), (0, 0), rounded_mask(radius=220 * SUPERSAMPLE))

    draw = ImageDraw.Draw(canvas)
    center = CANVAS // 2
    ring_radius = 300 * SUPERSAMPLE
    ring_width = 56 * SUPERSAMPLE

    # Clock ring with an intentional gap so the mark reads as motion.
    ring_box = (
        center - ring_radius,
        center - ring_radius,
        center + ring_radius,
        center + ring_radius,
    )
    draw.arc(ring_box, start=-90, end=232, fill=BRAND, width=ring_width)

    # Round the arc caps.
    cap_radius = ring_width // 2
    for angle in (-90, 232):
        radians = math.radians(angle)
        cap_x = center + ring_radius * math.cos(radians)
        cap_y = center + ring_radius * math.sin(radians)
        draw.ellipse(
            (
                cap_x - cap_radius,
                cap_y - cap_radius,
                cap_x + cap_radius,
                cap_y + cap_radius,
            ),
            fill=BRAND,
        )

    # Clock hands with rounded ends.
    hand_width = 32 * SUPERSAMPLE
    hands = (
        ((center, center), (center, center - 178 * SUPERSAMPLE)),
        ((center, center), (center + 118 * SUPERSAMPLE, center + 96 * SUPERSAMPLE)),
    )
    for start, end in hands:
        draw.line([start, end], fill=FOREGROUND, width=hand_width)
        radius = hand_width // 2
        for point in (start, end):
            draw.ellipse(
                (point[0] - radius, point[1] - radius, point[0] + radius, point[1] + radius),
                fill=FOREGROUND,
            )

    # Brand-colored hub.
    hub_radius = 22 * SUPERSAMPLE
    draw.ellipse(
        (
            center - hub_radius,
            center - hub_radius,
            center + hub_radius,
            center + hub_radius,
        ),
        fill=BRAND_LIGHT,
    )

    icon = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)

    output = Path(__file__).resolve().parent.parent / "assets" / "branding" / "app-icon.png"
    output.parent.mkdir(parents=True, exist_ok=True)
    icon.save(output)
    print(f"Wrote {output}")


if __name__ == "__main__":
    main()
