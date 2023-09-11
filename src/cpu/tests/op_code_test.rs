mod common;

#[cfg(test)]
mod tests {
    use nesemu_cpu::op_code::Opcode;
    use crate::common::setup;

    #[test]
    fn and() {
        let mut cpu = setup();
        cpu.acc_reg = 0b1011;
        cpu.operation(Opcode::AND);
        assert_eq!(cpu.acc_reg, 0b1001)
    }
}