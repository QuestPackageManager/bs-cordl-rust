#[cfg(feature = "System+Net+Http+Headers+Token")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Token {
    pub _cordl_type: crate::System::Net::Http::Headers::Token_Type,
    pub _StartPosition_k__BackingField: i32,
    pub _EndPosition_k__BackingField: i32,
}
#[cfg(feature = "System+Net+Http+Headers+Token")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Token =>
    "System.Net.Http.Headers"."Token"
);
#[cfg(feature = "System+Net+Http+Headers+Token")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::Http::Headers::Token {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Token")]
impl crate::System::Net::Http::Headers::Token {
    #[cfg(feature = "System+Net+Http+Headers+Token+Type")]
    pub type Type = crate::System::Net::Http::Headers::Token_Type;
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::System::Net::Http::Headers::Token_Type,
        startPosition: i32,
        endPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, startPosition, endPosition),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EndPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_EndPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Kind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Http::Headers::Token_Type> {
        let __cordl_ret: crate::System::Net::Http::Headers::Token_Type = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Kind",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StartPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_StartPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Http::Headers::Token_Type> {
        let __cordl_ret: crate::System::Net::Http::Headers::Token_Type = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EndPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_EndPosition",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_StartPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_StartPosition",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Token+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token_Type {
    End = 1i32,
    Error = 0i32,
    OpenParens = 9i32,
    QuotedString = 3i32,
    SeparatorComma = 8i32,
    SeparatorDash = 7i32,
    SeparatorEqual = 4i32,
    SeparatorSemicolon = 5i32,
    SeparatorSlash = 6i32,
    Token = 2i32,
}
#[cfg(feature = "System+Net+Http+Headers+Token+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Token_Type =>
    "System.Net.Http.Headers"."Token/Type"
);
