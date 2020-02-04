#!/usr/bin/env python3
print('.section .data.boot')
print('.align 0x1000')

print()
print('.global PHYS_IDENT_PML4')
print('PHYS_IDENT_PML4:')
for i in range(512):
    if i == 0x1ff:
        print('.quad PHYS_IDENT_PDP_0 + 0x1 + (((1 << 10) - 1) << 52)')
    elif i == 0 or i == 0x110:
        print('.quad PHYS_IDENT_PDP_1 + 0x1 + (((1 << 10) - 1) << 52)')
    else:
        print('.quad 0x0')

print()
print('PHYS_IDENT_PDP_0:')
for i in range(512):
    if i == 0x1fe:
        print('.quad PHYS_IDENT_PD_0 + 0x1 + (((1 << 10) - 1) << 52)')
    else:
        print('.quad 0x0')


print()
print('PHYS_IDENT_PDP_1:')
for i in range(512):
    print(f'.quad PHYS_IDENT_PD_{i} + 0x1 + (((1 << 10) - 1) << 52)')

for pd_i in range(512):
    print()
    print(f'PHYS_IDENT_PD_{pd_i}:')
    for pde_i in range(512):
        print(f'.quad ({pd_i * 512 + pde_i} * 0x200000) | 0x1 | (0x1 << 7)')
