#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIRUtility =>
    "UnityEngine.UIElements"."UIRUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIRUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIRUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl crate::UnityEngine::UIElements::UIRUtility {
    pub fn Destroy(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Destroy", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextPow2(n: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextPow2", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextPow2Exp(n: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextPow2Exp", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrevPow2(n: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrevPow2", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsElementSelfHidden(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsElementSelfHidden", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRoundRect(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRoundRect", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVectorImageBackground(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVectorImageBackground", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShapeWindingIsClockwise(
        maskDepth: i32,
        stencilRef: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShapeWindingIsClockwise", (maskDepth, stencilRef))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIRUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
