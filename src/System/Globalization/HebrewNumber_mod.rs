#[cfg(feature = "System+Globalization+HebrewNumber")]
#[repr(C)]
#[derive(Debug)]
pub struct HebrewNumber {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+HebrewNumber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::HebrewNumber =>
    "System.Globalization"."HebrewNumber"
);
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl std::ops::Deref for crate::System::Globalization::HebrewNumber {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl std::ops::DerefMut for crate::System::Globalization::HebrewNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl crate::System::Globalization::HebrewNumber {
    #[cfg(feature = "System+Globalization+HebrewNumber+HS")]
    pub type HS = crate::System::Globalization::HebrewNumber_HS;
    #[cfg(feature = "System+Globalization+HebrewNumber+HebrewToken")]
    pub type HebrewToken = crate::System::Globalization::HebrewNumber_HebrewToken;
    #[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
    pub type HebrewValue = crate::System::Globalization::HebrewNumber_HebrewValue;
}
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::HebrewNumber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber+HS")]
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HebrewNumber_HS {
    END = 100i8,
    S400 = 1i8,
    S400_400 = 2i8,
    S400_400_100 = 13i8,
    S400_400_DQ = 12i8,
    S400_DQ = 11i8,
    S400_X0 = 4i8,
    S400_X00 = 3i8,
    S400_X00_X0 = 6i8,
    S9 = 14i8,
    S9_DQ = 16i8,
    Start = 0i8,
    X = 8i8,
    X0 = 9i8,
    X00 = 10i8,
    X00_DQ = 5i8,
    X00_S9 = 15i8,
    X0_DQ = 7i8,
    _err = -1i8,
}
#[cfg(feature = "System+Globalization+HebrewNumber+HS")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::HebrewNumber_HS =>
    "System.Globalization"."HebrewNumber/HS"
);
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewToken")]
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HebrewNumber_HebrewToken {
    Digit1 = 4i16,
    Digit10 = 3i16,
    Digit100 = 2i16,
    Digit200_300 = 1i16,
    Digit400 = 0i16,
    Digit6_7 = 5i16,
    Digit7 = 6i16,
    Digit9 = 7i16,
    DoubleQuote = 9i16,
    Invalid = -1i16,
    SingleQuote = 8i16,
}
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::HebrewNumber_HebrewToken
    => "System.Globalization"."HebrewNumber/HebrewToken"
);
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HebrewNumber_HebrewValue {
    pub token: crate::System::Globalization::HebrewNumber_HebrewToken,
    pub value: i16,
}
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::HebrewNumber_HebrewValue
    => "System.Globalization"."HebrewNumber/HebrewValue"
);
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::HebrewNumber_HebrewValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
impl crate::System::Globalization::HebrewNumber_HebrewValue {
    pub fn _ctor(
        &mut self,
        token: crate::System::Globalization::HebrewNumber_HebrewToken,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (token, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
