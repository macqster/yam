# Weather Sprite Prototypes V1

This file preserves the first imported batch of plain-text monochrome weather-sprite proposals for future manual selection and refinement.

It is intentionally wider than the current runtime atlas. The goal here is to preserve alternatives, not to force early convergence.

## Sunny

For **sunny**, stay very close to the current wttr-like YAM anchor: open rays, centered disk, no extra texture.

### Variant A — canonical cleaned

```text
    \   /
     .-.
  - (   ) -
     `-'
    /   \
```

### Variant B — current shape, stronger horizontal ray

```text
    \   /
     .-.
  ― (   ) ―
     `-'
    /   \
```

### Variant C — compact 12-wide

```text
   \ | /
    .-.
 - (   ) -
    `-'
   / | \
```

### Variant D — softer / less noisy

```text
    \ /
     .
  .-( )-.
     '
    / \
```

Pick for now:

- `Variant A` if maximum portability matters most
- `Variant B` if `―` renders cleanly and the stronger current theatrical weight is preferred

## Clear Night

For **clear night**, keep it visually related to `sunny`, but remove rays and make the crescent calm and dim.

### Variant A — compact crescent

```text
     .--.
   .'  /
  /   (
  \    `.
   `---'
```

### Variant B — smaller / airier

```text
     .-.
   .' /
  (  (
   `-.`
```

### Variant C — sunny-derived moon disk

```text
     .-.
   (  /
    \(
     `-'
```

### Variant D — crescent + tiny star

```text
  .
     .--.
   .'  /
  /   (
   `--'
```

Pick for now:

- `Variant A`

## Partly Cloudy

For **partly cloudy**, keep the current YAM shape as the anchor: sun in the upper-left, cloud occluding it from the lower-right, and no precipitation rows.

### Variant A — current anchor / keep

```text
 \  /
_ /"".-.
  \_(   ).
  /(___(__)
```

### Variant B — slightly cleaner sun occlusion

```text
 \  /
_ /"".-.
  \_(   ).
   (___(__)
```

### Variant C — more compact 12-wide

```text
 \  /
_ /"".-.
  \(   ).
  (___(__)
```

### Variant D — softer / less diagonal noise

```text
  \ /
 _/"".-.
  _(   ).
 (___(__)
```

Pick for now:

- `Variant B`

## Cloudy

For **cloudy**, keep the current lumpy cloud as the canonical parent.

### Variant A — current anchor / keep

```text
    .--.
 .-(    ).
(___.__)__)
```

### Variant B — slightly cleaner underside

```text
    .--.
 .-(    ).
 (___(___))
```

### Variant C — wider, softer cloud

```text
    .---.
 .-(     ).
(___.___(__)
```

### Variant D — compact 12-wide

```text
   .--.
.-(    ).
(___.__)_)
```

Pick for now:

- `Variant A`

## Very Cloudy

For **very cloudy**, derive from `cloudy` but add a second rounded cloud mass.

### Variant A — double cloud, closest to current

```text
   .--. .-.
.-(    (   ).
(___.____(__)
```

### Variant B — fuller, heavier body

```text
   .--. .--.
.-(    (    ).
(___.____.__)
```

### Variant C — compact overlap

```text
    .--..-.
 .-(   (   ).
(___.___(__)
```

### Variant D — lumpy but still airy

```text
    .-. .--.
 .-(   (    ).
(___(___.__)
```

Pick for now:

- `Variant A`

## Overcast

For **overcast**, derive from `cloudy`, but flatten the silhouette into a low ceiling/slab.

### Variant A — flattened from current cloud

```text
   .------.
 .(        ).
(___________)
```

### Variant B — heavier slab / best overcast read

```text
 .----------.
(            )
(___ ____ ___)
```

### Variant C — low ceiling with shadow line

```text
  .--------.
 (__________)
  ----------
```

### Variant D — rounded but oppressive

```text
    .----.
 .-(      ).
(___________)
  ---------
```

Pick for now:

- `Variant B`

## Mist

For **mist**, derive from the current atmospheric banding, but make it lighter than fog: fewer marks, more air, broken rows.

### Variant A — light broken veil

```text
 _   -   _
   -   -
 _     - _
```

### Variant B — closest to current, reduced density

```text
 _ -   - _
    _ -
 _   -  _
```

### Variant C — flatter / calmer

```text
  _  _  _
    -  -
  _    _
```

### Variant D — almost haze only

```text
    -   -
 _     _
    -
```

Pick for now:

- `Variant B`

## Fog

For **fog**, use the current dense `_ - _ -` banding as the parent.

### Variant A — current dense fog / keep

```text
_ - _ - _ -
  _ - _ - _
 _ - _ - _ -
  _ - _ - _
 _ - _ - _ -
```

### Variant B — slightly cleaner, less noisy

```text
_ - _ - _ -
  _ - _ - _
_ - _ - _ -
  _ - _ - _
```

### Variant C — heavier horizontal veil

```text
_ _ _ _ _ _
  - - - -
_ _ _ _ _ _
  - - - -
_ _ _ _ _ _
```

### Variant D — layered fog, more atmospheric

```text
_ - _   _ -
   _ - _
_ - _ - _ -
   _ - _
_ - _   _ -
```

Pick for now:

- `Variant B`

## Light Showers

For **light showers**, keep the `cloudy` parent but make precipitation localized, not full-width.

### Variant A — sparse right-side shower

```text
    .--.
 .-(    ).
(___.__)__)
      ‘ ‘
     ‘
```

### Variant B — centered light shower

```text
    .--.
 .-(    ).
(___.__)__)
    ‘ ‘
      ‘
```

### Variant C — diagonal shower trace

```text
    .--.
 .-(    ).
(___.__)__)
      ‘
    ‘   ‘
```

### Variant D — slightly fuller but still localized

```text
    .--.
 .-(    ).
(___.__)__)
     ‘ ‘ ‘
      ‘
```

Pick for now:

- `Variant A`

## Light Rain

For **light rain**, keep the `cloudy` parent and make rain full-width but sparse.

### Variant A — current anchor / keep

```text
    .--.
 .-(    ).
(___.__)__)
   ‘ ‘ ‘ ‘
  ‘ ‘ ‘ ‘
```

### Variant B — lighter, more airy

```text
    .--.
 .-(    ).
(___.__)__)
   ‘   ‘
     ‘   ‘
```

### Variant C — full-width but sparse

```text
    .--.
 .-(    ).
(___.__)__)
  ‘   ‘   ‘
    ‘   ‘
```

### Variant D — slightly diagonal drift

```text
    .--.
 .-(    ).
(___.__)__)
   ‘  ‘  ‘
  ‘  ‘  ‘
```

Pick for now:

- `Variant C`

## Heavy Showers

For **heavy showers**, keep the same `cloudy` parent, but make rain denser and still localized.

### Variant A — localized heavy column

```text
    .--.
 .-(    ).
(___.__)__)
    ‘ ‘ ‘
   ‘ ‘ ‘
```

### Variant B — right-weighted burst

```text
    .--.
 .-(    ).
(___.__)__)
     ‘ ‘ ‘
    ‘ ‘ ‘
```

### Variant C — denser but narrow

```text
    .--.
 .-(    ).
(___.__)__)
    ‘‘‘‘
    ‘‘‘‘
```

### Variant D — heavier diagonal shower

```text
    .--.
 .-(    ).
(___.__)__)
     ‘ ‘ ‘
   ‘ ‘ ‘
```

Pick for now:

- `Variant A`

## Heavy Rain

For **heavy rain**, keep the `cloudy` parent but make precipitation full-width and dense.

### Variant A — dense full-width rain

```text
    .--.
 .-(    ).
(___.__)__)
  ‘ ‘ ‘ ‘ ‘
 ‘ ‘ ‘ ‘ ‘
```

### Variant B — current rain, tightened

```text
    .--.
 .-(    ).
(___.__)__)
  ‘ ‘ ‘ ‘
 ‘ ‘ ‘ ‘
```

### Variant C — maximum density, still readable

```text
    .--.
 .-(    ).
(___.__)__)
  ‘‘‘‘‘‘‘
 ‘‘‘‘‘‘‘
```

### Variant D — denser diagonal sheet

```text
    .--.
 .-(    ).
(___.__)__)
  ‘ ‘ ‘ ‘ ‘
‘ ‘ ‘ ‘ ‘
```

Pick for now:

- `Variant A`

## Light Snow

For **light snow**, keep the `cloudy` parent and use sparse `*` marks across the lower rows.

### Variant A — current anchor / likely keep

```text
    .--.
 .-(    ).
(___.__)__)
   *  *  *
  *  *  *
```

### Variant B — lighter / more clearly light

```text
    .--.
 .-(    ).
(___.__)__)
   *     *
      *
```

### Variant C — airy full-width drift

```text
    .--.
 .-(    ).
(___.__)__)
  *    *
    *    *
```

### Variant D — sparse staggered flakes

```text
    .--.
 .-(    ).
(___.__)__)
    *   *
  *
```

Pick for now:

- `Variant B`

## Heavy Snow

For **heavy snow**, keep the `cloudy` parent and make flakes full-width and denser than `light_snow`.

### Variant A — dense but still airy

```text
    .--.
 .-(    ).
(___.__)__)
  * * * *
 * * * *
```

### Variant B — current snow intensified

```text
    .--.
 .-(    ).
(___.__)__)
   * * * *
  * * * *
```

### Variant C — fuller snowfall sheet

```text
    .--.
 .-(    ).
(___.__)__)
  * * * * *
   * * * *
```

### Variant D — heavy, slightly drifting

```text
    .--.
 .-(    ).
(___.__)__)
 *  * *  *
  * *  * *
```

Pick for now:

- `Variant A`

## Light Snow Showers

For **light snow showers**, keep the `cloudy` parent and make snow localized, not full-width.

### Variant A — right-side sparse snow shower

```text
    .--.
 .-(    ).
(___.__)__)
      * *
     *
```

### Variant B — centered sparse snow shower

```text
    .--.
 .-(    ).
(___.__)__)
    * *
      *
```

### Variant C — diagonal drifting shower

```text
    .--.
 .-(    ).
(___.__)__)
      *
    *   *
```

### Variant D — minimal localized flakes

```text
    .--.
 .-(    ).
(___.__)__)
       *
     *
```

Pick for now:

- `Variant A`

## Heavy Snow Showers

For **heavy snow showers**, keep the `cloudy` parent and make snow dense but localized.

### Variant A — localized heavy snow column

```text
    .--.
 .-(    ).
(___.__)__)
    * * *
   * * *
```

### Variant B — right-weighted heavy shower

```text
    .--.
 .-(    ).
(___.__)__)
     * * *
    * * *
```

### Variant C — denser, narrow shaft

```text
    .--.
 .-(    ).
(___.__)__)
     ****
     ****
```

### Variant D — drifting heavy shower

```text
    .--.
 .-(    ).
(___.__)__)
     * * *
   * * *
```

Pick for now:

- `Variant A`

## Light Sleet

For **light sleet**, keep the `cloudy` parent and use a sparse mixed overlay.

### Variant A — sparse mixed fall

```text
    .--.
 .-(    ).
(___.__)__)
   *     ‘
      *
```

### Variant B — airy full-width mix

```text
    .--.
 .-(    ).
(___.__)__)
   *   ‘
     ‘   *
```

### Variant C — closest to current sleet, reduced

```text
    .--.
 .-(    ).
(___.__)__)
   *  '
  '     *
```

### Variant D — colder / pellet-forward

```text
    .--.
 .-(    ).
(___.__)__)
    *   *
      '
```

Pick for now:

- `Variant B`

## Light Sleet Showers

For **light sleet showers**, keep the `cloudy` parent and make the mixed precipitation localized and sparse.

### Variant A — right-side sparse mixed shower

```text
    .--.
 .-(    ).
(___.__)__)
      * ‘
     ‘
```

### Variant B — centered sparse mix

```text
    .--.
 .-(    ).
(___.__)__)
    * ‘
      *
```

### Variant C — colder / pellet-forward shower

```text
    .--.
 .-(    ).
(___.__)__)
      * *
     ‘
```

### Variant D — diagonal mixed trace

```text
    .--.
 .-(    ).
(___.__)__)
      *
    ‘   *
```

Pick for now:

- `Variant A`

## Sleet

For **sleet**, keep the `cloudy` parent and use a full-width mixed precipitation field.

### Variant A — current anchor / balanced mix

```text
    .--.
 .-(    ).
(___.__)__)
   *  '  *
  '  *  '
```

### Variant B — fuller mixed field

```text
    .--.
 .-(    ).
(___.__)__)
  *  '  * '
 '  *  '  *
```

### Variant C — pellet-forward sleet

```text
    .--.
 .-(    ).
(___.__)__)
  * * ' *
 ' * ' *
```

### Variant D — rain-forward sleet

```text
    .--.
 .-(    ).
(___.__)__)
  ' * ' *
 * ' * '
```

Pick for now:

- `Variant B`

## Thundery Showers

For **thundery showers**, keep the `cloudy` parent, add a small stroke-built lightning mark, and keep rain localized.

### Variant A — compact bolt + localized rain

```text
    .--.
 .-(    ).
(___.__)__)
    /_ ‘
   /'
```

### Variant B — bolt slightly centered

```text
    .--.
 .-(    ).
(___.__)__)
     /_ ‘
    /' ‘
```

### Variant C — shower remains dominant

```text
    .--.
 .-(    ).
(___.__)__)
    ‘ /_
   ‘ /'
```

### Variant D — sharper ASCII bolt

```text
    .--.
 .-(    ).
(___.__)__)
     /,
    /' ‘
```

Pick for now:

- `Variant A`

## Thundery Heavy Rain

For **thundery heavy rain**, use the heavy-rain full-width lower field plus a stroke-built bolt.

### Variant A — bolt + dense full-width rain

```text
    .--.
 .-(    ).
(___.__)__)
   /_ ‘ ‘ ‘
  /' ‘ ‘ ‘
```

### Variant B — centered bolt, rain sheet preserved

```text
    .--.
 .-(    ).
(___.__)__)
  ‘ /_ ‘ ‘
 ‘ /' ‘ ‘
```

### Variant C — stronger bolt, still compact

```text
    .--.
 .-(    ).
(___.__)__)
   /_,‘ ‘ ‘
  /' ‘ ‘ ‘
```

### Variant D — stormier, heavier visual weight

```text
    .--.
 .-(    ).
(___.__)__)
  /_ ‘ ‘ ‘ ‘
 /' ‘ ‘ ‘ ‘
```

Pick for now:

- `Variant A`

## Thundery Snow Showers

For **thundery snow showers**, combine the localized snow-shower column with a small stroke-built bolt.

### Variant A — compact bolt + localized snow

```text
    .--.
 .-(    ).
(___.__)__)
    /_ * *
   /' *
```

### Variant B — bolt centered, snow right-weighted

```text
    .--.
 .-(    ).
(___.__)__)
     /_ * *
    /'  *
```

### Variant C — snow shower dominant

```text
    .--.
 .-(    ).
(___.__)__)
    * /_ *
   * /'
```

### Variant D — sharper thunder, sparse flakes

```text
    .--.
 .-(    ).
(___.__)__)
     /, *
    /' *
```

Pick for now:

- `Variant A`

## Storm

For **storm**, switch from the normal `cloudy` parent to the heavier `overcast/slab` parent, then add full-width heavy rain plus a stronger stroke-built bolt.

### Variant A — slab cloud + heavy rain + bolt

```text
 .----------.
(            )
(___ ____ ___)
   /_ ‘ ‘ ‘ ‘
  /' ‘ ‘ ‘ ‘
```

### Variant B — centered stronger bolt

```text
 .----------.
(            )
(___ ____ ___)
  ‘ /_ ‘ ‘ ‘
 ‘ /' ‘ ‘ ‘
```

### Variant C — most stormy / oppressive

```text
 .----------.
(____________)
(___ ____ ___)
   /_,‘ ‘ ‘ ‘
  /' ‘ ‘ ‘ ‘
```

### Variant D — compact storm, closer to thundery rain

```text
   .------.
 .(        ).
(___________)
   /_ ‘ ‘ ‘
  /' ‘ ‘ ‘
```

Pick for now:

- `Variant A`
