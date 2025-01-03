#[cfg(feature = "UnityEngine+TextCore+Text+TextBackingContainer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TextBackingContainer {
    pub m_Array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub m_Count: i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextBackingContainer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::TextBackingContainer => "UnityEngine.TextCore.Text"
    ."TextBackingContainer"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextBackingContainer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextBackingContainer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextBackingContainer")]
impl crate::UnityEngine::TextCore::Text::TextBackingContainer {
    pub fn Resize(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Resize",
            (_cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Capacity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Count(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Count",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
