#[cfg(feature = "UnityEngine+UIElements+Cursor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Cursor {
    pub _texture_k__BackingField: *mut crate::UnityEngine::Texture2D,
    pub _hotspot_k__BackingField: crate::UnityEngine::Vector2,
    pub _defaultCursorId_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+UIElements+Cursor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Cursor =>
    "UnityEngine.UIElements"."Cursor"
);
#[cfg(feature = "UnityEngine+UIElements+Cursor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Cursor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Cursor")]
impl crate::UnityEngine::UIElements::Cursor {
    pub fn Equals_Cursor1(
        &mut self,
        other: crate::UnityEngine::UIElements::Cursor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultCursorId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_defaultCursorId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_hotspot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hotspot",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_texture",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultCursorId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_defaultCursorId",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_hotspot(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_hotspot",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_texture(
        &mut self,
        value: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_texture",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
