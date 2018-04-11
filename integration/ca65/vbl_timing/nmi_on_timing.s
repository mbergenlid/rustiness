; Tests NMI occurrence when enabled near time
; VBL flag is cleared.
;
; Enables NMI one PPU clock later on each line.
; Prints whether NMI occurred.
;
; 00 N 88_984
; 01 N
; 02 N
; 03 N
; 04 N 88_988
; 05 - 88_989
; 06 -
; 07 -
; 08 -

CUSTOM_NMI=1
.include "shell.inc"
.include "sync_vbl.s"
	
zp_byte nmi_count

nmi:    inc nmi_count
	rti

main:   jsr console_hide
	loop_n_times test,9
	check_crc $91410411
	jmp tests_passed        
	
test:   jsr print_a         ; 29768
	jsr disable_rendering
	jsr sync_vbl_delay      ; 0
	delay 29742+2287        ; 2261
	
	lda #0          ; +2    ; 32031
	sta <nmi_count  ; +3    ; 32034
	lda #$80        ; +2    ; 32036
	sta $2000       ; +4    ; 320
	nop             ; +2
	nop             ; +2
	lda #0          ; +2
	sta $2000       ; +4
	
	lda nmi_count
	print_cc bne,'N','-'
	jsr print_newline
	rts

