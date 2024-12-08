#[cfg(feature = "UnityEngine+Color32")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Color32 {
    padding: [u8; 4usize],
}
#[cfg(feature = "UnityEngine+Color32")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Color32 => "UnityEngine"."Color32"
);
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Color32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Color32")]
impl crate::UnityEngine::Color32 {
    pub fn _ctor(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (r, g, b, a),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider1(
        &mut self,
        format: *mut crate::System::String,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret)
    }
}
