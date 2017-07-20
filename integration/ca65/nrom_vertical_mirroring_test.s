
nmi:
    rti

irq:
    rti

main:
    lda #$80
    sta $6000
    jsr test
    jmp done

test:
    lda #$20
    sta $2006
    lda #$00
    sta $2006

    lda #$01
    sta $2007
    lda #$02
    sta $2007
    lda #$03
    sta $2007

    lda #$28
    sta $2006
    lda #$00
    sta $2006
    lda $2007 ;Dummy read

    ldx #$01
    lda $2007
    cmp #$01
    bne fail

    ldx #$03
    lda $2007
    cmp #$02
    bne fail

    ldx #$03
    lda $2007
    cmp #$03
    bne fail
    jmp test_done
fail:
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
    .byte 1,0,$01

