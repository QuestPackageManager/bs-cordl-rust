#[cfg(feature = "TMPro+KerningPairKey")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KerningPairKey {
    pub ascii_Left: u32,
    pub ascii_Right: u32,
    pub key: u32,
}
#[cfg(feature = "TMPro+KerningPairKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::KerningPairKey => "TMPro"
    ."KerningPairKey"
);
#[cfg(feature = "TMPro+KerningPairKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::KerningPairKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+KerningPairKey")]
impl crate::TMPro::KerningPairKey {
    pub fn _ctor(
        &mut self,
        ascii_left: u32,
        ascii_right: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ascii_left, ascii_right),
        )?;
        Ok(__cordl_ret.into())
    }
}
