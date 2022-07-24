#[doc = "Register `DRIVER` reader"]
pub struct R(crate::R<DRIVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRIVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRIVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRIVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRIVER` writer"]
pub struct W(crate::W<DRIVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRIVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DRIVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRIVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Drive of PIO Line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE0_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE0_A> for bool {
    #[inline(always)]
    fn from(variant: LINE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE0` reader - Drive of PIO Line 0"]
pub type LINE0_R = crate::BitReader<LINE0_A>;
impl LINE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE0_A {
        match self.bits {
            false => LINE0_A::LOW_DRIVE,
            true => LINE0_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE0_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE0_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE0` writer - Drive of PIO Line 0"]
pub type LINE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE0_A, O>;
impl<'a, const O: u8> LINE0_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE0_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE0_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE1_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE1_A> for bool {
    #[inline(always)]
    fn from(variant: LINE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE1` reader - Drive of PIO Line 1"]
pub type LINE1_R = crate::BitReader<LINE1_A>;
impl LINE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE1_A {
        match self.bits {
            false => LINE1_A::LOW_DRIVE,
            true => LINE1_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE1_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE1_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE1` writer - Drive of PIO Line 1"]
pub type LINE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE1_A, O>;
impl<'a, const O: u8> LINE1_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE1_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE1_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE2_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE2_A> for bool {
    #[inline(always)]
    fn from(variant: LINE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE2` reader - Drive of PIO Line 2"]
pub type LINE2_R = crate::BitReader<LINE2_A>;
impl LINE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE2_A {
        match self.bits {
            false => LINE2_A::LOW_DRIVE,
            true => LINE2_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE2_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE2_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE2` writer - Drive of PIO Line 2"]
pub type LINE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE2_A, O>;
impl<'a, const O: u8> LINE2_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE2_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE2_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE3_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE3_A> for bool {
    #[inline(always)]
    fn from(variant: LINE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE3` reader - Drive of PIO Line 3"]
pub type LINE3_R = crate::BitReader<LINE3_A>;
impl LINE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE3_A {
        match self.bits {
            false => LINE3_A::LOW_DRIVE,
            true => LINE3_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE3_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE3_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE3` writer - Drive of PIO Line 3"]
pub type LINE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE3_A, O>;
impl<'a, const O: u8> LINE3_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE3_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE3_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE4_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE4_A> for bool {
    #[inline(always)]
    fn from(variant: LINE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE4` reader - Drive of PIO Line 4"]
pub type LINE4_R = crate::BitReader<LINE4_A>;
impl LINE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE4_A {
        match self.bits {
            false => LINE4_A::LOW_DRIVE,
            true => LINE4_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE4_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE4_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE4` writer - Drive of PIO Line 4"]
pub type LINE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE4_A, O>;
impl<'a, const O: u8> LINE4_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE4_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE4_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE5_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE5_A> for bool {
    #[inline(always)]
    fn from(variant: LINE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE5` reader - Drive of PIO Line 5"]
pub type LINE5_R = crate::BitReader<LINE5_A>;
impl LINE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE5_A {
        match self.bits {
            false => LINE5_A::LOW_DRIVE,
            true => LINE5_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE5_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE5_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE5` writer - Drive of PIO Line 5"]
pub type LINE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE5_A, O>;
impl<'a, const O: u8> LINE5_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE5_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE5_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE6_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE6_A> for bool {
    #[inline(always)]
    fn from(variant: LINE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE6` reader - Drive of PIO Line 6"]
pub type LINE6_R = crate::BitReader<LINE6_A>;
impl LINE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE6_A {
        match self.bits {
            false => LINE6_A::LOW_DRIVE,
            true => LINE6_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE6_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE6_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE6` writer - Drive of PIO Line 6"]
pub type LINE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE6_A, O>;
impl<'a, const O: u8> LINE6_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE6_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE6_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE7_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE7_A> for bool {
    #[inline(always)]
    fn from(variant: LINE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE7` reader - Drive of PIO Line 7"]
pub type LINE7_R = crate::BitReader<LINE7_A>;
impl LINE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE7_A {
        match self.bits {
            false => LINE7_A::LOW_DRIVE,
            true => LINE7_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE7_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE7_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE7` writer - Drive of PIO Line 7"]
pub type LINE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE7_A, O>;
impl<'a, const O: u8> LINE7_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE7_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE7_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE8_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE8_A> for bool {
    #[inline(always)]
    fn from(variant: LINE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE8` reader - Drive of PIO Line 8"]
pub type LINE8_R = crate::BitReader<LINE8_A>;
impl LINE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE8_A {
        match self.bits {
            false => LINE8_A::LOW_DRIVE,
            true => LINE8_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE8_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE8_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE8` writer - Drive of PIO Line 8"]
pub type LINE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE8_A, O>;
impl<'a, const O: u8> LINE8_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE8_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE8_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE9_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE9_A> for bool {
    #[inline(always)]
    fn from(variant: LINE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE9` reader - Drive of PIO Line 9"]
pub type LINE9_R = crate::BitReader<LINE9_A>;
impl LINE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE9_A {
        match self.bits {
            false => LINE9_A::LOW_DRIVE,
            true => LINE9_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE9_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE9_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE9` writer - Drive of PIO Line 9"]
pub type LINE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE9_A, O>;
impl<'a, const O: u8> LINE9_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE9_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE9_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE10_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE10_A> for bool {
    #[inline(always)]
    fn from(variant: LINE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE10` reader - Drive of PIO Line 10"]
pub type LINE10_R = crate::BitReader<LINE10_A>;
impl LINE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE10_A {
        match self.bits {
            false => LINE10_A::LOW_DRIVE,
            true => LINE10_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE10_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE10_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE10` writer - Drive of PIO Line 10"]
pub type LINE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE10_A, O>;
impl<'a, const O: u8> LINE10_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE10_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE10_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE11_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE11_A> for bool {
    #[inline(always)]
    fn from(variant: LINE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE11` reader - Drive of PIO Line 11"]
pub type LINE11_R = crate::BitReader<LINE11_A>;
impl LINE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE11_A {
        match self.bits {
            false => LINE11_A::LOW_DRIVE,
            true => LINE11_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE11_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE11_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE11` writer - Drive of PIO Line 11"]
pub type LINE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE11_A, O>;
impl<'a, const O: u8> LINE11_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE11_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE11_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE12_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE12_A> for bool {
    #[inline(always)]
    fn from(variant: LINE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE12` reader - Drive of PIO Line 12"]
pub type LINE12_R = crate::BitReader<LINE12_A>;
impl LINE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE12_A {
        match self.bits {
            false => LINE12_A::LOW_DRIVE,
            true => LINE12_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE12_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE12_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE12` writer - Drive of PIO Line 12"]
pub type LINE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE12_A, O>;
impl<'a, const O: u8> LINE12_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE12_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE12_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE13_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE13_A> for bool {
    #[inline(always)]
    fn from(variant: LINE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE13` reader - Drive of PIO Line 13"]
pub type LINE13_R = crate::BitReader<LINE13_A>;
impl LINE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE13_A {
        match self.bits {
            false => LINE13_A::LOW_DRIVE,
            true => LINE13_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE13_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE13_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE13` writer - Drive of PIO Line 13"]
pub type LINE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE13_A, O>;
impl<'a, const O: u8> LINE13_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE13_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE13_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE14_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE14_A> for bool {
    #[inline(always)]
    fn from(variant: LINE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE14` reader - Drive of PIO Line 14"]
pub type LINE14_R = crate::BitReader<LINE14_A>;
impl LINE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE14_A {
        match self.bits {
            false => LINE14_A::LOW_DRIVE,
            true => LINE14_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE14_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE14_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE14` writer - Drive of PIO Line 14"]
pub type LINE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE14_A, O>;
impl<'a, const O: u8> LINE14_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE14_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE14_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE15_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE15_A> for bool {
    #[inline(always)]
    fn from(variant: LINE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE15` reader - Drive of PIO Line 15"]
pub type LINE15_R = crate::BitReader<LINE15_A>;
impl LINE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE15_A {
        match self.bits {
            false => LINE15_A::LOW_DRIVE,
            true => LINE15_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE15_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE15_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE15` writer - Drive of PIO Line 15"]
pub type LINE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE15_A, O>;
impl<'a, const O: u8> LINE15_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE15_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE15_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE16_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE16_A> for bool {
    #[inline(always)]
    fn from(variant: LINE16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE16` reader - Drive of PIO Line 16"]
pub type LINE16_R = crate::BitReader<LINE16_A>;
impl LINE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE16_A {
        match self.bits {
            false => LINE16_A::LOW_DRIVE,
            true => LINE16_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE16_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE16_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE16` writer - Drive of PIO Line 16"]
pub type LINE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE16_A, O>;
impl<'a, const O: u8> LINE16_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE16_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE16_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE17_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE17_A> for bool {
    #[inline(always)]
    fn from(variant: LINE17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE17` reader - Drive of PIO Line 17"]
pub type LINE17_R = crate::BitReader<LINE17_A>;
impl LINE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE17_A {
        match self.bits {
            false => LINE17_A::LOW_DRIVE,
            true => LINE17_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE17_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE17_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE17` writer - Drive of PIO Line 17"]
pub type LINE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE17_A, O>;
impl<'a, const O: u8> LINE17_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE17_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE17_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE18_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE18_A> for bool {
    #[inline(always)]
    fn from(variant: LINE18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE18` reader - Drive of PIO Line 18"]
pub type LINE18_R = crate::BitReader<LINE18_A>;
impl LINE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE18_A {
        match self.bits {
            false => LINE18_A::LOW_DRIVE,
            true => LINE18_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE18_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE18_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE18` writer - Drive of PIO Line 18"]
pub type LINE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE18_A, O>;
impl<'a, const O: u8> LINE18_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE18_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE18_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE19_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE19_A> for bool {
    #[inline(always)]
    fn from(variant: LINE19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE19` reader - Drive of PIO Line 19"]
pub type LINE19_R = crate::BitReader<LINE19_A>;
impl LINE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE19_A {
        match self.bits {
            false => LINE19_A::LOW_DRIVE,
            true => LINE19_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE19_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE19_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE19` writer - Drive of PIO Line 19"]
pub type LINE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE19_A, O>;
impl<'a, const O: u8> LINE19_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE19_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE19_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE20_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE20_A> for bool {
    #[inline(always)]
    fn from(variant: LINE20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE20` reader - Drive of PIO Line 20"]
pub type LINE20_R = crate::BitReader<LINE20_A>;
impl LINE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE20_A {
        match self.bits {
            false => LINE20_A::LOW_DRIVE,
            true => LINE20_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE20_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE20_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE20` writer - Drive of PIO Line 20"]
pub type LINE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE20_A, O>;
impl<'a, const O: u8> LINE20_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE20_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE20_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE21_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE21_A> for bool {
    #[inline(always)]
    fn from(variant: LINE21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE21` reader - Drive of PIO Line 21"]
pub type LINE21_R = crate::BitReader<LINE21_A>;
impl LINE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE21_A {
        match self.bits {
            false => LINE21_A::LOW_DRIVE,
            true => LINE21_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE21_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE21_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE21` writer - Drive of PIO Line 21"]
pub type LINE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE21_A, O>;
impl<'a, const O: u8> LINE21_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE21_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE21_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE22_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE22_A> for bool {
    #[inline(always)]
    fn from(variant: LINE22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE22` reader - Drive of PIO Line 22"]
pub type LINE22_R = crate::BitReader<LINE22_A>;
impl LINE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE22_A {
        match self.bits {
            false => LINE22_A::LOW_DRIVE,
            true => LINE22_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE22_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE22_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE22` writer - Drive of PIO Line 22"]
pub type LINE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE22_A, O>;
impl<'a, const O: u8> LINE22_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE22_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE22_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE23_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE23_A> for bool {
    #[inline(always)]
    fn from(variant: LINE23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE23` reader - Drive of PIO Line 23"]
pub type LINE23_R = crate::BitReader<LINE23_A>;
impl LINE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE23_A {
        match self.bits {
            false => LINE23_A::LOW_DRIVE,
            true => LINE23_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE23_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE23_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE23` writer - Drive of PIO Line 23"]
pub type LINE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE23_A, O>;
impl<'a, const O: u8> LINE23_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE23_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE23_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE24_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE24_A> for bool {
    #[inline(always)]
    fn from(variant: LINE24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE24` reader - Drive of PIO Line 24"]
pub type LINE24_R = crate::BitReader<LINE24_A>;
impl LINE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE24_A {
        match self.bits {
            false => LINE24_A::LOW_DRIVE,
            true => LINE24_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE24_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE24_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE24` writer - Drive of PIO Line 24"]
pub type LINE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE24_A, O>;
impl<'a, const O: u8> LINE24_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE24_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE24_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE25_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE25_A> for bool {
    #[inline(always)]
    fn from(variant: LINE25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE25` reader - Drive of PIO Line 25"]
pub type LINE25_R = crate::BitReader<LINE25_A>;
impl LINE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE25_A {
        match self.bits {
            false => LINE25_A::LOW_DRIVE,
            true => LINE25_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE25_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE25_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE25` writer - Drive of PIO Line 25"]
pub type LINE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE25_A, O>;
impl<'a, const O: u8> LINE25_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE25_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE25_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE26_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE26_A> for bool {
    #[inline(always)]
    fn from(variant: LINE26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE26` reader - Drive of PIO Line 26"]
pub type LINE26_R = crate::BitReader<LINE26_A>;
impl LINE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE26_A {
        match self.bits {
            false => LINE26_A::LOW_DRIVE,
            true => LINE26_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE26_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE26_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE26` writer - Drive of PIO Line 26"]
pub type LINE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE26_A, O>;
impl<'a, const O: u8> LINE26_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE26_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE26_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE27_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE27_A> for bool {
    #[inline(always)]
    fn from(variant: LINE27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE27` reader - Drive of PIO Line 27"]
pub type LINE27_R = crate::BitReader<LINE27_A>;
impl LINE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE27_A {
        match self.bits {
            false => LINE27_A::LOW_DRIVE,
            true => LINE27_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE27_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE27_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE27` writer - Drive of PIO Line 27"]
pub type LINE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE27_A, O>;
impl<'a, const O: u8> LINE27_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE27_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE27_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE28_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE28_A> for bool {
    #[inline(always)]
    fn from(variant: LINE28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE28` reader - Drive of PIO Line 28"]
pub type LINE28_R = crate::BitReader<LINE28_A>;
impl LINE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE28_A {
        match self.bits {
            false => LINE28_A::LOW_DRIVE,
            true => LINE28_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE28_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE28_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE28` writer - Drive of PIO Line 28"]
pub type LINE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE28_A, O>;
impl<'a, const O: u8> LINE28_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE28_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE28_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE29_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE29_A> for bool {
    #[inline(always)]
    fn from(variant: LINE29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE29` reader - Drive of PIO Line 29"]
pub type LINE29_R = crate::BitReader<LINE29_A>;
impl LINE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE29_A {
        match self.bits {
            false => LINE29_A::LOW_DRIVE,
            true => LINE29_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE29_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE29_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE29` writer - Drive of PIO Line 29"]
pub type LINE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE29_A, O>;
impl<'a, const O: u8> LINE29_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE29_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE29_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE30_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE30_A> for bool {
    #[inline(always)]
    fn from(variant: LINE30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE30` reader - Drive of PIO Line 30"]
pub type LINE30_R = crate::BitReader<LINE30_A>;
impl LINE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE30_A {
        match self.bits {
            false => LINE30_A::LOW_DRIVE,
            true => LINE30_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE30_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE30_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE30` writer - Drive of PIO Line 30"]
pub type LINE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE30_A, O>;
impl<'a, const O: u8> LINE30_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE30_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE30_A::HIGH_DRIVE)
    }
}
#[doc = "Drive of PIO Line 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE31_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE31_A> for bool {
    #[inline(always)]
    fn from(variant: LINE31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE31` reader - Drive of PIO Line 31"]
pub type LINE31_R = crate::BitReader<LINE31_A>;
impl LINE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE31_A {
        match self.bits {
            false => LINE31_A::LOW_DRIVE,
            true => LINE31_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE31_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE31_A::HIGH_DRIVE
    }
}
#[doc = "Field `LINE31` writer - Drive of PIO Line 31"]
pub type LINE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRIVER_SPEC, LINE31_A, O>;
impl<'a, const O: u8> LINE31_W<'a, O> {
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE31_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE31_A::HIGH_DRIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&self) -> LINE0_R {
        LINE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&self) -> LINE1_R {
        LINE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&self) -> LINE2_R {
        LINE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&self) -> LINE3_R {
        LINE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&self) -> LINE4_R {
        LINE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&self) -> LINE5_R {
        LINE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&self) -> LINE6_R {
        LINE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&self) -> LINE7_R {
        LINE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&self) -> LINE8_R {
        LINE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&self) -> LINE9_R {
        LINE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&self) -> LINE10_R {
        LINE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&self) -> LINE11_R {
        LINE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&self) -> LINE12_R {
        LINE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&self) -> LINE13_R {
        LINE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&self) -> LINE14_R {
        LINE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&self) -> LINE15_R {
        LINE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&self) -> LINE16_R {
        LINE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&self) -> LINE17_R {
        LINE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&self) -> LINE18_R {
        LINE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&self) -> LINE19_R {
        LINE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&self) -> LINE20_R {
        LINE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&self) -> LINE21_R {
        LINE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&self) -> LINE22_R {
        LINE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&self) -> LINE23_R {
        LINE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&self) -> LINE24_R {
        LINE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&self) -> LINE25_R {
        LINE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&self) -> LINE26_R {
        LINE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&self) -> LINE27_R {
        LINE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&self) -> LINE28_R {
        LINE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&self) -> LINE29_R {
        LINE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&self) -> LINE30_R {
        LINE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&self) -> LINE31_R {
        LINE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&mut self) -> LINE0_W<0> {
        LINE0_W::new(self)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&mut self) -> LINE1_W<1> {
        LINE1_W::new(self)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&mut self) -> LINE2_W<2> {
        LINE2_W::new(self)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&mut self) -> LINE3_W<3> {
        LINE3_W::new(self)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&mut self) -> LINE4_W<4> {
        LINE4_W::new(self)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&mut self) -> LINE5_W<5> {
        LINE5_W::new(self)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&mut self) -> LINE6_W<6> {
        LINE6_W::new(self)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&mut self) -> LINE7_W<7> {
        LINE7_W::new(self)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&mut self) -> LINE8_W<8> {
        LINE8_W::new(self)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&mut self) -> LINE9_W<9> {
        LINE9_W::new(self)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&mut self) -> LINE10_W<10> {
        LINE10_W::new(self)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&mut self) -> LINE11_W<11> {
        LINE11_W::new(self)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&mut self) -> LINE12_W<12> {
        LINE12_W::new(self)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&mut self) -> LINE13_W<13> {
        LINE13_W::new(self)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&mut self) -> LINE14_W<14> {
        LINE14_W::new(self)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&mut self) -> LINE15_W<15> {
        LINE15_W::new(self)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&mut self) -> LINE16_W<16> {
        LINE16_W::new(self)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&mut self) -> LINE17_W<17> {
        LINE17_W::new(self)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&mut self) -> LINE18_W<18> {
        LINE18_W::new(self)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&mut self) -> LINE19_W<19> {
        LINE19_W::new(self)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&mut self) -> LINE20_W<20> {
        LINE20_W::new(self)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&mut self) -> LINE21_W<21> {
        LINE21_W::new(self)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&mut self) -> LINE22_W<22> {
        LINE22_W::new(self)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&mut self) -> LINE23_W<23> {
        LINE23_W::new(self)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&mut self) -> LINE24_W<24> {
        LINE24_W::new(self)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&mut self) -> LINE25_W<25> {
        LINE25_W::new(self)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&mut self) -> LINE26_W<26> {
        LINE26_W::new(self)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&mut self) -> LINE27_W<27> {
        LINE27_W::new(self)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&mut self) -> LINE28_W<28> {
        LINE28_W::new(self)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&mut self) -> LINE29_W<29> {
        LINE29_W::new(self)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&mut self) -> LINE30_W<30> {
        LINE30_W::new(self)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&mut self) -> LINE31_W<31> {
        LINE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Drive Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [driver](index.html) module"]
pub struct DRIVER_SPEC;
impl crate::RegisterSpec for DRIVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [driver::R](R) reader structure"]
impl crate::Readable for DRIVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [driver::W](W) writer structure"]
impl crate::Writable for DRIVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRIVER to value 0"]
impl crate::Resettable for DRIVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
