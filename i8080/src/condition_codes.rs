#[derive(Clone, Debug, Default)]
pub struct ConditionCodes {
    pub carry: bool,
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub aux_carry: bool,
}

impl ConditionCodes {
    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry
    }

    pub fn reset_carry(&mut self) {
        self.carry = false
    }

    pub fn set_zero(&mut self, val: u8) {
        self.zero = (val & 0xFF) == 0
    }

    pub fn set_sign(&mut self, val: u8) {
        self.sign = (val & 0x80) != 0
    }

    pub fn set_parity(&mut self, val: u8) {
        self.parity = val.count_ones() % 2 == 0
    }

    pub fn set_aux_carry(&mut self, aux_carry: bool) {
        self.aux_carry = aux_carry
    }

    pub fn reset_aux_carry(&mut self) {
        self.aux_carry = false
    }

    pub fn flags_to_psw(&self) -> u8 {
        let mut psw: u8 = 0;
        psw |= (self.sign as u8) << 7;
        psw |= (self.zero as u8) << 6;
        psw |= (self.aux_carry as u8) << 4;
        psw |= (self.parity as u8) << 2;
        psw |= 0x1 << 1;
        psw |= self.carry as u8;

        psw
    }

    pub fn psw_to_flags(&mut self, psw: u8) {
        self.sign = ((psw >> 7) & 0x1) > 0;
        self.zero = ((psw >> 6) & 0x1) > 0;
        self.aux_carry = ((psw >> 4) & 0x1) > 0;
        self.parity = ((psw >> 2) & 0x1) > 0;
        self.carry = (psw & 0x1) > 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_to_psw() {
        let mut flags: ConditionCodes = Default::default();
        flags.sign = true;
        flags.carry = true;
        let psw = flags.flags_to_psw();
        assert_eq!(psw, 0x83);
    }

    #[test]
    fn test_psw_to_flags() {
        let mut flags: ConditionCodes = Default::default();
        flags.psw_to_flags(0x93);
        assert_eq!(flags.sign, true);
        assert_eq!(flags.carry, true);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.parity, false);
        assert_eq!(flags.aux_carry, true);
    }
}
