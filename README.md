# nesemu
tiny wip nes emulator,, for fun
## about
cpu kind of works. :')

Currently focusing mainly on getting the CPU to run test suites in browser with easily interactable ram. 

![image](https://github.com/sqrtM/nesemu/assets/79169638/942fe747-a222-4817-9b6f-51c1f2d0cda6)

Not looking to be cycle perfect. Just learnin 

## structure

I'm currently going through and restructuring things to allow for better
multiplatform support, but the general idea is going to be something like:

- workspace/
    - cpu/  --6502 emulation
    - gui/  --gui stuff 
    - core/ --traits and generic implementations
    - .../ --ppu, assets, ...

