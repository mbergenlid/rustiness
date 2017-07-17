; Switches to Segment and places Line there.
; Line can be an .align directive, .res, .byte, etc.
; Examples:
; seg_data BSS, .align 256
; seg_data RODATA, {message: .byte "Test",0}
.macro seg_data Segment, Line
	.pushseg
	.segment .string(Segment)
		Line
	.popseg
.endmacro

; Reserves Size bytes in Segment for Name.
; If Size is omitted, reserves one byte.
.macro seg_res Segment, Name, Size
	.ifblank Size
		seg_data Segment, Name: .res 1
	.else
		seg_data Segment, Name: .res Size
	.endif
.endmacro

; Shortcuts for zeropage, bss, and stack
.define zp_res  seg_res ZEROPAGE,
.define nv_res  seg_res NVRAM,
.define bss_res seg_res BSS,
.define sp_res  seg_res STACK,
.define zp_byte zp_res
