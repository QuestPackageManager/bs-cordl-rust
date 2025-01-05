#[cfg(feature = "UnityEngine+UIElements+ProgressBar")]
#[repr(C)]
#[derive(Debug)]
pub struct ProgressBar {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::AbstractProgressBar,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ProgressBar =>
    "UnityEngine.UIElements"."ProgressBar"
);
#[cfg(feature = "UnityEngine+UIElements+ProgressBar")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ProgressBar {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::AbstractProgressBar,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ProgressBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar")]
impl crate::UnityEngine::UIElements::ProgressBar {
    #[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::ProgressBar_UxmlFactory;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::ProgressBar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct ProgressBar_UxmlFactory {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ProgressBar>,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AbstractProgressBar_UxmlTraits,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ProgressBar_UxmlFactory
    => "UnityEngine.UIElements"."ProgressBar/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ProgressBar_UxmlFactory {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ProgressBar>,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AbstractProgressBar_UxmlTraits,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ProgressBar_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
impl crate::UnityEngine::UIElements::ProgressBar_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ProgressBar+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ProgressBar_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
