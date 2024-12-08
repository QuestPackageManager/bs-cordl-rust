#[cfg(feature = "System+Resources+ResourceLocator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ResourceLocator {
    pub _value: *mut crate::System::Object,
    pub _dataPos: i32,
}
#[cfg(feature = "System+Resources+ResourceLocator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::ResourceLocator =>
    "System.Resources"."ResourceLocator"
);
#[cfg(feature = "System+Resources+ResourceLocator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Resources::ResourceLocator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Resources+ResourceLocator")]
impl crate::System::Resources::ResourceLocator {
    pub fn _ctor(
        &mut self,
        dataPos: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dataPos, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_DataPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DataPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Value(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Value",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
