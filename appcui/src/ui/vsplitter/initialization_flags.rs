

/*

poate fi: verfical sau orizantorl --> posibil bool
- potenal autoscale --> daca vreu sa pastreze aspect ratio
- auto collapse ? (cand pierde focus - sa se inchida, in sus, jos, stanga, dreapte)
- dimensiuni minimale si maximale pentru fiecare panel 
- dimensiunile pot fi date fie ca procente, fie ca valori absolute


- facem 2 (vsplitter si hsplitter)

vspliter:
    - 2 paneluri (stanga si dreapta)
    - flags: preserve_aspect_ratio, auto-collapse on focus lost
    - VSplitter::new(layout,flags) -> VSplitter
    - VSplitter::set_left_panel_bounds(min,max)
    - VSplitter::set_right_panel_bounds(min,max)
    - VSplitter::set_left_panel_width(size) (poate fi si float)
    - VSplitter::set_right_panel_width(size) (poate fi si float)

    Auto collapse / auto-focus:
        - cand pierde focusul isi pierde dimensiunea pana la minimul posibil
        - doar un panel poate avea auto collapse ==> 2 flaguri (AutoCollapseLeft, AutoCollapseRight)

*/