; WIP

SECTION "Boot ROM", ROM0[$0000]

Start:
    di
    ld sp, $FFFE

    xor a
    ldh [$FF40], a

    ld hl, $8000
    ld bc, $2000

.clear_vram:
    ld [hl+], a
    dec bc
    ld a, b
    or c
    jr nz, .clear_vram

ds $100 - 4 - @, 0

    ld a, $01
    ldh [$FF50], a