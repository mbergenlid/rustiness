; Tests NMI timing.
;
; Prints which instruction NMI occurred
; after. Test is run one PPU clock later
; each line.
;
; 00 4 4
; 01 4 3
; 02 4 3
; 03 3 3
; 04 3 3
; 05 3 3
; 06 3 3
; 07 3 2
; 08 3 2
; 09 2 2

CUSTOM_NMI=1
.include "shell.inc"
.include "sync_vbl.s"

zp_byte nmi_data

nmi:    stx nmi_data
	rti

main:   jsr console_hide
	loop_n_times test,10
	check_crc $A6CCB10A
	jmp tests_passed

; (82221+0)+(29749+29781+2+3+2+2+4)*3-262*341*2
test:   jsr print_a
	jsr disable_rendering
    ;; Either sync_vbl_delay is wrong or I misunderstand when VBL starts
	jsr sync_vbl_delay  ; 82_221 ; 82_217
	delay 29749+29781   ; 89_248 ; 82_123
	lda #$FF            ; 82_135 ; 
	sta nmi_data        ; 82_141
	ldx #0              ; 82_150
	lda #$80            ; 82_156
	sta $2000           ; 82_162
landing:
	; NMI occurs after one of these
	; instructions and prints X
	ldx #1              ; 82_174
	ldx #2              ; 82_180
	ldx #3              ; 82_186
	ldx #4              ; 82_192
	ldx #5              ; 82_1
	
	lda #0
	sta $2000
	lda nmi_data
	jsr print_dec
	jsr print_newline
	rts
