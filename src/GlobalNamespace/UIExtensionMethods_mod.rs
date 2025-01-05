#[cfg(feature = "UIExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct UIExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UIExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UIExtensionMethods => ""
    ."UIExtensionMethods"
);
#[cfg(feature = "UIExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::UIExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UIExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::UIExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UIExtensionMethods")]
impl crate::GlobalNamespace::UIExtensionMethods {
    pub fn CopySizeAndPositionFrom(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopySizeAndPositionFrom", (target, source))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UIExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UIExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
