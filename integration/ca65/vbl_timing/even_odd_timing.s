; Tests timing of skipped clock every other frame
; when BG is enabled.
;
; Output: 08 08 09 07 

.include "shell.inc"
.include "sync_vbl.s"

adjust = 2359

; (82182+13*3)+4+(13+4+2351+4)*3-262*341
; (82217+4)+(13+4+2351+4)*3-262*341
.align 256
; There are 7 160 PPU cycles between VBLANK start and skip cycle.
test:   jsr disable_rendering
	jsr sync_vbl_delay ; E403
	delay 13 ; E406

    ; 5+(13+13+4+2351+4)*3
	; $2001=X for most of VBL, Y for part of frame, then 0
    ; This block is 29 777 cycles long
	stx $2001 ; E40B   4
	delay adjust-4-4   ; 
    ; 7104
	sty $2001 ; E41A
	delay 20000
	lda #0    ; E429
	sta $2001
	delay 29781-adjust-4-20000-6  ; = 7412
	
	; Two frames with BG off
	delay 29781
	delay 29781-1
	
	; Third frame same as first. Since clock is skipped every
	; other frame, only one of these two will have the skipped
	; clock, so its effect on later frame timing won't be a
	; problem.
    ; This block is 29 781 cycles long
	stx $2001 ; $E458
	delay adjust-4
	sty $2001 ; E467
	delay 20000
	lda #0    ; E476
	sta $2001
	delay 29781-adjust-4-20000-6 ; = 7412

	; Find number of PPU clocks until VBL
	delay 29781-3-22 ; = 29 756
	ldx #0
:       delay 29781-2-4-3
	inx              ; E4A7
	bit PPUSTATUS
	bpl :-
	
	jsr print_x
	rts

main:   jsr console_hide
	
	set_test 2,"Clock is skipped too soon, relative to enabling BG"
	lda #4
	ldx #0
	ldy #8
	jsr test
	cpx #8
	jne test_failed
	
	set_test 3,"Clock is skipped too late, relative to enabling BG"
	lda #5
	ldx #0
	ldy #8
	jsr test
	cpx #8
	jne test_failed
	
	set_test 4,"Clock is skipped too soon, relative to disabling BG"
	lda #4
	ldx #8
	ldy #0
	jsr test
	cpx #9
	jne test_failed
	
	set_test 5,"Clock is skipped too late, relative to disabling BG"
	lda #5
	ldx #8
	ldy #0
	jsr test
	cpx #7
	jne test_failed
	
	jmp tests_passed
