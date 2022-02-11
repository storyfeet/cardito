Cardito
=======

A program for building game cards based on the templito library.

It reads cards and the info about them from a card file, and can then produce an SVG with the cards laid out in an easily printable format.

The easiest way to use it is to install it with:

```cargo install -u cardito```

Once installed you can run ```cardito init``` which will create the following files:

main.ito:
```text

{{export 
    card_files = ["cards.crd"];
    card_width = 45;
    card_height= 60;
    padding = 1;
    margin=4;
}}

{{@export extra -}}
    {{# Anything to appear on every page once #}}
{{- /export}}

{{global front}}
    <rect {{xywh 0 0 45 60}} {{fl_stk .color "black" 2}} />
    <text {{font 3 "Arial"}} {{xy 22.5 20}} text-anchor="middle" {{fl_stk "black" "none" 0}}>{{.Name}}:£{{.price}}</text>
{{/global}}

{{global back}}
    <rect {{xywh 0 0 45 60}} {{fl_stk "blue" "black" 2}} />
{{/global}}

```

and cards.crd

```
param price

2*Apple ,100:
.color : "red"

3*Pear, 50:
.color : "green"
```

Edit the files to discribe the cards you want.

Then call ```cardito build -f main.ito```

This will output svgs with the cards laid out nicely

