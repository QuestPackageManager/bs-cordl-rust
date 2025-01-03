#[cfg(feature = "UnityEngine+UI+MultipleDisplayUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct MultipleDisplayUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+MultipleDisplayUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::MultipleDisplayUtilities =>
    "UnityEngine.UI"."MultipleDisplayUtilities"
);
#[cfg(feature = "UnityEngine+UI+MultipleDisplayUtilities")]
impl std::ops::Deref for crate::UnityEngine::UI::MultipleDisplayUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MultipleDisplayUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::UI::MultipleDisplayUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MultipleDisplayUtilities")]
impl crate::UnityEngine::UI::MultipleDisplayUtilities {
    pub fn GetRelativeMousePositionForDrag(
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRelativeMousePositionForDrag", (eventData, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRelativeMousePositionForRaycast(
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRelativeMousePositionForRaycast", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RelativeMouseAtScaled(
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RelativeMouseAtScaled", (position))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+MultipleDisplayUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::MultipleDisplayUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
