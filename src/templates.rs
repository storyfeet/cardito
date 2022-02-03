pub const PAGE_TEMPLATE: &'static str = r##"
{{- let u=first .units "mm" ; w=first .page_width 210 ; h = first .page_height 297 -}}
<svg version="1.1" 
    width="{{$w}}{{$u}}" 
    height="{{$h}}{{$u}}" 
    viewBox="0 0 {{$w}} {{$h}}" 
    xmlns="http://www.w3.org/2000/svg"
>
{{first .extra ""}}
{{.cards}}
</svg>
"##;

pub const CARD_WRAP: &'static str = r##"{{let u = first .units "mm"}}
<g transform="translate({{.current_x}} {{.current_y}})">
{{.current_card}}
</g>"##;
