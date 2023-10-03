# nesemu
tiny wip nes emulator,, for fun
## about
cpu kind of works. :')

Currently focusing mainly on getting the CPU to run test suites in browser with easily interactable ram. 

![image](https://github.com/sqrtM/nesemu/assets/79169638/6712c887-241a-4c4c-8f8a-0d42bd1311e0)


Not looking to be cycle perfect. Just learnin 

## structure

I'm currently going through and restructuring things to allow for better
multiplatform support, but the general idea is going to be something like:

- workspace/
    - cpu/  --6502 emulation
    - gui/  --gui stuff 
    - core/ --traits and generic implementations
    - .../ --ppu, assets, ...

