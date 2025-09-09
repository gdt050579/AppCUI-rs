# AppCUI-rs Gallery

A visual tour of AppCUI-rs. Each category shows one quick preview; expand for more.

```
Currently this is a work in progress - expect first version to be ready by 1st of September.
```

- [AppCUI-rs Gallery](#appcui-rs-gallery)
  - [Animations](#animations)
  - [Lists \& Trees](#lists--trees)


---

## Animations

![Animations — preview](img/animations/matrix.gif)

* **Code:** [examples/matrix](https://github.com/gdt050579/AppCUI-rs/tree/main/examples/matrix)
* **Elements:** [Custom Desktop](https://gdt050579.github.io/AppCUI-rs/chapter-4/custom_desktop.html), [Menus](https://gdt050579.github.io/AppCUI-rs/chapter-4/menu.html), [Timers](https://gdt050579.github.io/AppCUI-rs/chapter-7/timers.html)

<details>
  <summary>More Anmations</summary>

| Image                                   | Descrption                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| <img src="img/animations/spiral.gif" >  | **Spiral Animation** <br> * **Code:** [examples/spiral](https://github.com/gdt050579/AppCUI-rs/tree/main/examples/spiral) <br> * **Description:** The Spiral example defines a struct that incrementally computes spiral coordinates in polar form, applies aspect-ratio correction, and updates them each frame to illustrate animated geometry generation.                                                                                                                                                       |
| <img src="img/animations/fractal.gif" > | **Fractal Animation** <br> * **Code:** [examples/fractal](https://github.com/gdt050579/AppCUI-rs/tree/main/examples/fractal) <br> * **Description:** The Fractal example constructs a recursive tree-like structure by generating line segments from a start point, computing endpoints using trigonometric functions, and branching at ±45° angles. Each update advances rotation, modulates scale, and regenerates points up to a configurable depth, producing an evolving visualization of recursive geometry. |
| <img src="img/animations/timer.gif" >   | **Timer** <br> * **Code:** [examples/timer](https://github.com/gdt050579/AppCUI-rs/tree/main/examples/timer) <br> * **Description:** The Timer example displays elapsed time using ASCII-art digits on a Canvas, updated via periodic TimerEvents. It provides Start, Pause, and Resume buttons to control execution, dynamically changing digit rendering based on paused state, and demonstrates integrating timers, custom rendering, and event handling within AppCUI.                                         |

</details>

---

## Lists & Trees

![Lists & Trees — preview](img/lists/listview.gif)

* **Code:** [examples/listview](https://github.com/gdt050579/AppCUI-rs/tree/main/examples/listview)
* **Elements:** [List View](https://gdt050579.github.io/AppCUI-rs/chapter-3/stock-controls/listview.html), [List item](https://gdt050579.github.io/AppCUI-rs/chapter-3/object-traits/listitem.html), [Menus](https://gdt050579.github.io/AppCUI-rs/chapter-4/menu.html)

<details>
  <summary>More Anmations</summary>

| Image        | Descrption |
| ------------ | ---------- |
| <img src="img/lists/graphview.gif" >   | **Spiral Animation** <br> * **Code:** [examples/graphs](https://github.com/gdt050579/AppCUI-rs/tree/main/examples/graphs) <br> * **Description:** Various animation with graphs/trees where you can move nodes, display orthogonal lines, search and filter data, etc            |

</details>