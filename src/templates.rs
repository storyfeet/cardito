pub const PAGE_TEMPLATE: &'static str = r##"<svg version="1.1" 
    width="{{first .page_width 210}}{{first .units "mm"}}" 
    height="{{first .page_height 297}}{{first .units "mm"}}" 
    xmlns="http://www.w3.org/2000/svg"
>
{{first .extra ""}}
{{.cards}}
</svg>
"##;

pub const CARD_WRAP: &'static str = r##"{{let u = first .units "mm"}}
<g transform="translate({{.current_x}}{{$u}},{{.current_y}}{{$u}})">
{{.current_card}}
</g>"##;
