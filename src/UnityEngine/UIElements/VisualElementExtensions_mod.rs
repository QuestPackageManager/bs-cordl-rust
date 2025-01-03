#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElementExtensions
    => "UnityEngine.UIElements"."VisualElementExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElementExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl crate::UnityEngine::UIElements::VisualElementExtensions {
    pub fn AddManipulator(
        ele: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        manipulator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IManipulator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddManipulator", (ele, manipulator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeCoordinatesTo(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeCoordinatesTo", (src, dest, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalToWorld(
        ele: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        p: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalToWorld", (ele, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveManipulator(
        ele: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        manipulator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IManipulator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveManipulator", (ele, manipulator))?;
        Ok(__cordl_ret.into())
    }
    pub fn StretchToParentSize(
        elem: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StretchToParentSize", (elem))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToLocal_Rect1(
        ele: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        r: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WorldToLocal", (ele, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToLocal_Vector2_0(
        ele: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        p: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WorldToLocal", (ele, p))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
