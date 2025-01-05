#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
#[repr(C)]
#[derive(Debug)]
pub struct Clipping {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Clipping =>
    "UnityEngine.ProBuilder"."Clipping"
);
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Clipping {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Clipping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl crate::UnityEngine::ProBuilder::Clipping {
    #[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
    pub type OutCode = crate::UnityEngine::ProBuilder::Clipping_OutCode;
    pub fn ComputeOutCode(
        rect: crate::UnityEngine::Rect,
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::Clipping_OutCode,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Clipping_OutCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeOutCode", (rect, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn RectContainsLineSegment(
        rect: crate::UnityEngine::Rect,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectContainsLineSegment", (rect, x0, y0, x1, y1))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Clipping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Clipping_OutCode {
    #[default]
    Bottom = 4i32,
    Inside = 0i32,
    Left = 1i32,
    Right = 2i32,
    Top = 8i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Clipping_OutCode =>
    "UnityEngine.ProBuilder"."Clipping/OutCode"
);
