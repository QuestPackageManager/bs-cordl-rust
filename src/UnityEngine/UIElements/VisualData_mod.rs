#[cfg(feature = "UnityEngine+UIElements+VisualData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisualData {
    pub backgroundColor: crate::UnityEngine::Color,
    pub backgroundImage: crate::UnityEngine::UIElements::Background,
    pub backgroundPositionX: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundPositionY: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundRepeat: crate::UnityEngine::UIElements::BackgroundRepeat,
    pub backgroundSize: crate::UnityEngine::UIElements::BackgroundSize,
    pub borderBottomColor: crate::UnityEngine::Color,
    pub borderBottomLeftRadius: crate::UnityEngine::UIElements::Length,
    pub borderBottomRightRadius: crate::UnityEngine::UIElements::Length,
    pub borderLeftColor: crate::UnityEngine::Color,
    pub borderRightColor: crate::UnityEngine::Color,
    pub borderTopColor: crate::UnityEngine::Color,
    pub borderTopLeftRadius: crate::UnityEngine::UIElements::Length,
    pub borderTopRightRadius: crate::UnityEngine::UIElements::Length,
    pub opacity: f32,
    pub overflow: crate::UnityEngine::UIElements::OverflowInternal,
}
#[cfg(feature = "UnityEngine+UIElements+VisualData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualData =>
    "UnityEngine.UIElements"."VisualData"
);
#[cfg(feature = "UnityEngine+UIElements+VisualData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualData")]
impl crate::UnityEngine::UIElements::VisualData {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::VisualData> {
        let __cordl_ret: crate::UnityEngine::UIElements::VisualData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Copy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::VisualData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_VisualData0(
        &mut self,
        other: crate::UnityEngine::UIElements::VisualData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
