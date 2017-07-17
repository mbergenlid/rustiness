
nmi:
    rti

irq:
    rti

main:
    lda #$80
    sta $6000
    jsr test
    jsr test + $4000
    jmp done

test:
    lda #$3F
    sta $00
    lda $00
    cmp #$3F
    beq test_done
    ldx #$01
    stx $6000
test_done:
    rts

done:
    lda #$00
    sta $6000
    jmp done

.segment "VECTORS"
    .word nmi, main, irq

.segment "HEADER"
    .byte "NES",$1A
    .byte 1,0

