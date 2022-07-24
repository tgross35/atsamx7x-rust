#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "WKUP Wakeup Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPS_A {
    #[doc = "0: No wakeup due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wakeup due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<WKUPS_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPS` reader - WKUP Wakeup Status (cleared on read)"]
pub type WKUPS_R = crate::BitReader<WKUPS_A>;
impl WKUPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPS_A {
        match self.bits {
            false => WKUPS_A::NO,
            true => WKUPS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WKUPS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == WKUPS_A::PRESENT
    }
}
#[doc = "Supply Monitor Detection Wakeup Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMWS_A {
    #[doc = "0: No wakeup due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wakeup due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<SMWS_A> for bool {
    #[inline(always)]
    fn from(variant: SMWS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMWS` reader - Supply Monitor Detection Wakeup Status (cleared on read)"]
pub type SMWS_R = crate::BitReader<SMWS_A>;
impl SMWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMWS_A {
        match self.bits {
            false => SMWS_A::NO,
            true => SMWS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMWS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SMWS_A::PRESENT
    }
}
#[doc = "Brownout Detector Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTS_A {
    #[doc = "0: No core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    PRESENT = 1,
}
impl From<BODRSTS_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTS` reader - Brownout Detector Reset Status (cleared on read)"]
pub type BODRSTS_R = crate::BitReader<BODRSTS_A>;
impl BODRSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTS_A {
        match self.bits {
            false => BODRSTS_A::NO,
            true => BODRSTS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BODRSTS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == BODRSTS_A::PRESENT
    }
}
#[doc = "Supply Monitor Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTS_A {
    #[doc = "0: No supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    PRESENT = 1,
}
impl From<SMRSTS_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTS` reader - Supply Monitor Reset Status (cleared on read)"]
pub type SMRSTS_R = crate::BitReader<SMRSTS_A>;
impl SMRSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTS_A {
        match self.bits {
            false => SMRSTS_A::NO,
            true => SMRSTS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMRSTS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SMRSTS_A::PRESENT
    }
}
#[doc = "Supply Monitor Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMS_A {
    #[doc = "0: No supply monitor detection since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one supply monitor detection since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<SMS_A> for bool {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS` reader - Supply Monitor Status (cleared on read)"]
pub type SMS_R = crate::BitReader<SMS_A>;
impl SMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            false => SMS_A::NO,
            true => SMS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SMS_A::PRESENT
    }
}
#[doc = "Supply Monitor Output Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMOS_A {
    #[doc = "0: The supply monitor detected VDDIO higher than its threshold at its last measurement."]
    HIGH = 0,
    #[doc = "1: The supply monitor detected VDDIO lower than its threshold at its last measurement."]
    LOW = 1,
}
impl From<SMOS_A> for bool {
    #[inline(always)]
    fn from(variant: SMOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMOS` reader - Supply Monitor Output Status"]
pub type SMOS_R = crate::BitReader<SMOS_A>;
impl SMOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMOS_A {
        match self.bits {
            false => SMOS_A::HIGH,
            true => SMOS_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SMOS_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SMOS_A::LOW
    }
}
#[doc = "32-kHz Oscillator Selection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSEL_A {
    #[doc = "0: The slow clock, SLCK, is generated by the embedded 32 kHz RC oscillator."]
    RC = 0,
    #[doc = "1: The slow clock, SLCK, is generated by the 32 kHz crystal oscillator."]
    CRYST = 1,
}
impl From<OSCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSEL` reader - 32-kHz Oscillator Selection Status"]
pub type OSCSEL_R = crate::BitReader<OSCSEL_A>;
impl OSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSEL_A {
        match self.bits {
            false => OSCSEL_A::RC,
            true => OSCSEL_A::CRYST,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == OSCSEL_A::RC
    }
    #[doc = "Checks if the value of the field is `CRYST`"]
    #[inline(always)]
    pub fn is_cryst(&self) -> bool {
        *self == OSCSEL_A::CRYST
    }
}
#[doc = "Low-power Debouncer Wakeup Status on WKUP0 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCS0_A {
    #[doc = "0: No wakeup due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wakeup due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<LPDBCS0_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCS0` reader - Low-power Debouncer Wakeup Status on WKUP0 (cleared on read)"]
pub type LPDBCS0_R = crate::BitReader<LPDBCS0_A>;
impl LPDBCS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCS0_A {
        match self.bits {
            false => LPDBCS0_A::NO,
            true => LPDBCS0_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS0_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == LPDBCS0_A::PRESENT
    }
}
#[doc = "Low-power Debouncer Wakeup Status on WKUP1 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCS1_A {
    #[doc = "0: No wakeup due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wakeup due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<LPDBCS1_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCS1` reader - Low-power Debouncer Wakeup Status on WKUP1 (cleared on read)"]
pub type LPDBCS1_R = crate::BitReader<LPDBCS1_A>;
impl LPDBCS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCS1_A {
        match self.bits {
            false => LPDBCS1_A::NO,
            true => LPDBCS1_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS1_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == LPDBCS1_A::PRESENT
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS0_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS0_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS0` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS0_R = crate::BitReader<WKUPIS0_A>;
impl WKUPIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS0_A {
        match self.bits {
            false => WKUPIS0_A::DIS,
            true => WKUPIS0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS0_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS1_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS1_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS1` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS1_R = crate::BitReader<WKUPIS1_A>;
impl WKUPIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS1_A {
        match self.bits {
            false => WKUPIS1_A::DIS,
            true => WKUPIS1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS1_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS2_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS2_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS2` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS2_R = crate::BitReader<WKUPIS2_A>;
impl WKUPIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS2_A {
        match self.bits {
            false => WKUPIS2_A::DIS,
            true => WKUPIS2_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS2_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS2_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS3_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS3_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS3` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS3_R = crate::BitReader<WKUPIS3_A>;
impl WKUPIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS3_A {
        match self.bits {
            false => WKUPIS3_A::DIS,
            true => WKUPIS3_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS3_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS3_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS4_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS4_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS4` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS4_R = crate::BitReader<WKUPIS4_A>;
impl WKUPIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS4_A {
        match self.bits {
            false => WKUPIS4_A::DIS,
            true => WKUPIS4_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS4_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS4_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS5_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS5_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS5` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS5_R = crate::BitReader<WKUPIS5_A>;
impl WKUPIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS5_A {
        match self.bits {
            false => WKUPIS5_A::DIS,
            true => WKUPIS5_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS5_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS5_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS6_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS6_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS6` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS6_R = crate::BitReader<WKUPIS6_A>;
impl WKUPIS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS6_A {
        match self.bits {
            false => WKUPIS6_A::DIS,
            true => WKUPIS6_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS6_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS6_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS7_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS7_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS7` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS7_R = crate::BitReader<WKUPIS7_A>;
impl WKUPIS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS7_A {
        match self.bits {
            false => WKUPIS7_A::DIS,
            true => WKUPIS7_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS7_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS7_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS8_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS8_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS8` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS8_R = crate::BitReader<WKUPIS8_A>;
impl WKUPIS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS8_A {
        match self.bits {
            false => WKUPIS8_A::DIS,
            true => WKUPIS8_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS8_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS8_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS9_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS9_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS9` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS9_R = crate::BitReader<WKUPIS9_A>;
impl WKUPIS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS9_A {
        match self.bits {
            false => WKUPIS9_A::DIS,
            true => WKUPIS9_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS9_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS9_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS10_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS10_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS10` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS10_R = crate::BitReader<WKUPIS10_A>;
impl WKUPIS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS10_A {
        match self.bits {
            false => WKUPIS10_A::DIS,
            true => WKUPIS10_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS10_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS10_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS11_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS11_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS11` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS11_R = crate::BitReader<WKUPIS11_A>;
impl WKUPIS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS11_A {
        match self.bits {
            false => WKUPIS11_A::DIS,
            true => WKUPIS11_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS11_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS11_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS12_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS12_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS12` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS12_R = crate::BitReader<WKUPIS12_A>;
impl WKUPIS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS12_A {
        match self.bits {
            false => WKUPIS12_A::DIS,
            true => WKUPIS12_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS12_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS12_A::EN
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS13_A {
    #[doc = "0: The corresponding wakeup input is disabled, or was inactive at the time the debouncer triggered a wakeup event."]
    DIS = 0,
    #[doc = "1: The corresponding wakeup input was active at the time the debouncer triggered a wakeup event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS13_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS13` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS13_R = crate::BitReader<WKUPIS13_A>;
impl WKUPIS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS13_A {
        match self.bits {
            false => WKUPIS13_A::DIS,
            true => WKUPIS13_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS13_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS13_A::EN
    }
}
impl R {
    #[doc = "Bit 1 - WKUP Wakeup Status (cleared on read)"]
    #[inline(always)]
    pub fn wkups(&self) -> WKUPS_R {
        WKUPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wakeup Status (cleared on read)"]
    #[inline(always)]
    pub fn smws(&self) -> SMWS_R {
        SMWS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn bodrsts(&self) -> BODRSTS_R {
        BODRSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn smrsts(&self) -> SMRSTS_R {
        SMRSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Supply Monitor Status (cleared on read)"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline(always)]
    pub fn smos(&self) -> SMOS_R {
        SMOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-power Debouncer Wakeup Status on WKUP0 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs0(&self) -> LPDBCS0_R {
        LPDBCS0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low-power Debouncer Wakeup Status on WKUP1 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs1(&self) -> LPDBCS1_R {
        LPDBCS1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis0(&self) -> WKUPIS0_R {
        WKUPIS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis1(&self) -> WKUPIS1_R {
        WKUPIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis2(&self) -> WKUPIS2_R {
        WKUPIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis3(&self) -> WKUPIS3_R {
        WKUPIS3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis4(&self) -> WKUPIS4_R {
        WKUPIS4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis5(&self) -> WKUPIS5_R {
        WKUPIS5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis6(&self) -> WKUPIS6_R {
        WKUPIS6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis7(&self) -> WKUPIS7_R {
        WKUPIS7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis8(&self) -> WKUPIS8_R {
        WKUPIS8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis9(&self) -> WKUPIS9_R {
        WKUPIS9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis10(&self) -> WKUPIS10_R {
        WKUPIS10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis11(&self) -> WKUPIS11_R {
        WKUPIS11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis12(&self) -> WKUPIS12_R {
        WKUPIS12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis13(&self) -> WKUPIS13_R {
        WKUPIS13_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Supply Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
