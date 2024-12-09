#[cfg(feature = "System+Net+HeaderVariantInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HeaderVariantInfo {
    pub m_name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_variant: crate::System::Net::CookieVariant,
}
#[cfg(feature = "System+Net+HeaderVariantInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HeaderVariantInfo => "System.Net"
    ."HeaderVariantInfo"
);
#[cfg(feature = "System+Net+HeaderVariantInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::HeaderVariantInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+HeaderVariantInfo")]
impl crate::System::Net::HeaderVariantInfo {
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        variant: crate::System::Net::CookieVariant,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, variant),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Name",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Variant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieVariant> {
        let __cordl_ret: crate::System::Net::CookieVariant = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Variant",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
