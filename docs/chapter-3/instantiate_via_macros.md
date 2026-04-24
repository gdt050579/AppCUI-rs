# Instantiate a control using macros

All controls can be built via their constructors, but also via specialized macros meant to ease setting up a control. The general format is as follows: `controller!("<parameter_list>")`, where `controller!` is a specialized macro (for example `button!`) and `parameter_list` is a *JSON-like/Python-like* format made up of:
* *positional parameters* — for example, in this case `"10,20,30"` has three positional parameters: `10`, `20`, and `30`.
* *named parameters* — parameters described using a name and a value. The usual format is `name:value`, but `name=value` is also supported.

All parameters are separated from one another using `,` or `;`. The overall format of a parameter list is:

```
PositionalParam-1, ... PositionalParam-n, NamedParam-1, ... NamedParam-m
```

Both positional parameters and named parameters are optional. Their order is not arbitrary: positional parameters (if used) must always appear before named parameters.

## Values

The values used as parameters can be:
* **regular words** (e.g. `align:center`)
* **numerical values** (e.g. `x=10` or `y=-2`)
* **percentages** (e.g. `width:10%` or `height=25%`)
* **strings** — a string can be wrapped in double quotes or single quotes and can contain newlines (e.g. `"..."` or `'...'`). If both single quotes and double quotes must appear inside a string, three consecutive double or single quotes can be used (e.g. `"""..."""` or `'''...'''`).
* **lists of values** — written using `[` and `]`. The general format is `[value-1, value-2, ... value-n]`; for example `[10,20,30]` is a list with three values: 10, 20, and 30.
* **nested parameter lists** — written using `{` and `}`. For example, `point={x:10,y:20}` means parameter `point` is defined as two parameters: `x` with value `10` and `y` with value `20`.
* **flags** — a meta-interpretation of the parameters described above. Flags can be a regular word or string, or a list of values. If using a word, you can separate flags with `|` or `+`. Additionally, when using strings, spaces, `,`, and `;` can also be used as separators. When using a list of values, each value is a separate flag. For example, the following declarations are equivalent: 
    - `flags=[flag1,flag2]`
    - `flags=flag1+flag2`
    - `flags=flag1|flag2`
    - `flags="flags1,flags2"`
    - `flags="flags1;flags2"`
    - `flags="flags1 flags2"`

## Common parameters

All controls have a set of common parameters that are required for layout or to change some of their states (such as visibility or if a control is enabled - can receive input).

| Parameter names                                                                 | Type                                              | Purpose                                    |
| ------------------------------------------------------------------------------- | ------------------------------------------------- | ------------------------------------------ |
| `x`, `y`, `width`, `height`, `left`, `right`, `top`, `bottom` and their aliases | Numerical or percentage                           | Used for control layout                    |
| `align`, `dock`, and their aliases                                              | Alignment value (left, topleft, top, center, ...) | Used for control layout                    |
| `enabled` or `enable`                                                           | bool (**true** or **false**)                      | Used to set the enabled state of a control |
| `visible`                                                                       | bool (**true** or **false**)                      | Used to set the visibility of a control    |
