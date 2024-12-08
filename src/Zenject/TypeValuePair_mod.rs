#[cfg(feature = "Zenject+TypeValuePair")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TypeValuePair {
    pub Type: *mut crate::System::Type,
    pub Value: *mut crate::System::Object,
}
#[cfg(feature = "Zenject+TypeValuePair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TypeValuePair => "Zenject"
    ."TypeValuePair"
);
#[cfg(feature = "Zenject+TypeValuePair")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Zenject::TypeValuePair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Zenject+TypeValuePair")]
impl crate::Zenject::TypeValuePair {
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, value),
        )?;
        Ok(__cordl_ret)
    }
}
