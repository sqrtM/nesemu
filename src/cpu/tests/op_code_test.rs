mod common;

#[cfg(test)]
mod tests {
    use nesemu_cpu::cpu::StatusFlag::{N, Z};
    use nesemu_cpu::op_code::Opcode;

    use crate::common::setup;

    #[test]
    fn adc() {
        // todo!
    }

    #[test]
    fn and() {
        let mut cpu = setup(0b1000_1101);
        cpu.acc_reg = 0b1000_0010;
        cpu.operation(Opcode::AND);
        assert_eq!(cpu.get_flag(N), 1);
        assert_eq!(cpu.get_flag(Z), 0);
        assert_eq!(cpu.acc_reg, 0b1000_0000);
    }

    #[test]
    fn asl() {
        //let mut cpu = setup(0b1000_1101);
        //cpu.acc_reg = 0b1000_0010;
        //cpu.operation(Opcode::AND);
        //cpu.opcode = 1;
        //dbg!(cpu.acc_reg, cpu.addr_abs);
    }
}
