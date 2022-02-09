Cardito Keywords
=================


Template Builtins
================

first
------

    Used in templates to select the first OK value from a list
    eg : {{first NULL "hello"}} will return "hello"
    This is useful when a variable may or may not exist
    eg : {{first $0.color "GREEN}}

select
------

    Used to choose a value from a list depending on the first value
    eg {{select .num "red" "green" "blue"}}
    if (num == 0): "red" will be chosen else if (n== 1) :"green" ...
    Also works with booleans. True will select the first item, False, the second

global
------

    Used to define a template that Cardito can use 
    eg : {{global front}}<what the front of a card should look like>{{/global}}

Required Templates 
=================

The following templates may be defined in the primary template as global. only "front" is required.

front   (card, config)    Required
-----

    How the front of the cards should look when printed

back    (card, config)
----

    How the Back of the cards should look when printed.
    If not provided card backs are not printed

page (config)
----
   
    The default will create an a4 page with 'mm' units. But you can set the page size if you prefer.
    It wraps the cards as {{.cards}} these will be set. 
    It uses the following config variables {{.units .page_width .page_height .extra .cards}}

front_temp (config)
-------
    A template describing the file out path.  default = "out/front_{{$page_number}}.svg"

back_temp (config)
-------
    A template describing the file out path.  default = "out/back_{{$page_number}}.svg"

card_wrap (config)
---------
    Used to place the cards where they need to go and provide some standard things for all cards. Shouldn't need changing
    Default wraps the cards in a <g> translating them by {{.current_x,.current_y}} (These variables are not settable)



Optional Exports
===============

Everything Exported from the main template, or included in the cli as -v <key> <value> will be placed into the config object as a property. You can set and use any variables this way, but the following are used by the program and default templates in the program.

units :string
----
    The units the document will be built in. Default:'mm'
    All number variables use these units.

page_width && page_height : number
----------

    The width and height of the page. This will be used to calculate card positions.


card_width && card_height: number
--------------------------

    Will be used to calculate card positions. 

margin : number
----------

    Space at the edge of the page. Used to calculate card positions

padding : number
------------

    The space between cards. Used to calculate card positions


front_path  && back_path : string
--------

    The output path for the front and back files  '{{$page_number}}.svg' will be appended so the files don't overlap
    Only used if a template is not provided
    
    
extra
-----

    A string that you wish to appear at the beginning of the SVG in order to be used be all the cards or provide a background.  It is intended to allow the SVG 'use' command.


