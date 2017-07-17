
nmi:
    rti

irq:
    rti

main:
    lda #$80
    sta $6000
    lda #$3F
    sta $00
    lda $00
    cmp #$3F
    beq done
    ldx #$01
    stx $6000
done:
    lda #$00
    sta $6000
    jmp done

.segment "VECTORS"
    .word nmi, main, irq

.segment "HEADER"
    .byte "NES",$1A
    .byte 1,0

