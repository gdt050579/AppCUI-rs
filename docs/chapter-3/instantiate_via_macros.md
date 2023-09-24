# Instantiate a control using macros

All controls can be build via their constructors, but also via some specialized macros that are meant to ease setting up a controller. The general format is as follows: `controller!("<parameter_list>")`, where the `controller!` is a specialized macro (for example `button!`) and the `parameter_list` is a *json-like/python-like* format, form out of:
* *positional parameters* - for example in this case: `"10,20,30"` has three positional parameters `10`, `20` and `30`
* *named parameters* - are parameters described using a name and a value. The usual format is `name:value` but `name=value` is also supported.

All parameters are separated one from onether using `,` or `;`. The overall format of a parameter list is:

```
PostionalParam-1, ... PostionalParam-n, NamedParam-1,  ... NamedParam-m
```

Both positional parameters and named parameters are optionally. Their order however it is not. Positional parameters (if used) should always be placed before named parameters.

## Values

The values used as parameters can be:
* **regula word** (ex: `align:center`)
* **numerical values** (ex: `x=10`or `y=-2`)
* **percentages** (ex: `width:10%` or `height=25%`)
* **strings** - a string can be separated between douple quotes or single quotes and can contain new lines (ex: `"..."` or `'...'`). If both a single quote and a double quote has to be used in a string, three consecuitev double or single quotes can be used (ex: `"""..."""` or `'''...'''`)
* **list of values** - obtained using `[` and `]` characters. The general format is `[value-1, value-2, ... value-n]` - ex: `[10,20,30]` is a list with three values 10,20 and 30.
* **another parameter list** - obtained using `[` and `]` characters. The general format is `{parameter list}` - for example the following syntax `point={x:10,y:20}` translates into parameter `point` being defined as a set of two parameters `x` with value `10` and `y` with value `20`.
* **flags** - flags are a meta interpretation for the previously described parameters. It can be a regular word / string or a list of values. If using a word you can separate flags using one of the following characters: `|` and `+`.  Aditionally when using a strings, spaces, `,` and `;` cand also be used as a separator. When using a list of values, each one of the values is a separate flag. For example the following declarations are equivalent: 
    - `flags=[flag1,flag2]`
    - `flags=flag1+flag2`
    - `flags=flag1|flag2`
    - `flags="flags1,flags2"`
    - `flags="flags1;flags2"`
    - `flags="flags1 flags2"`
