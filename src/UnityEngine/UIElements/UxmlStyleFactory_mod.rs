#[cfg(feature = "UnityEngine+UIElements+UxmlStyleFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlStyleFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlStyleTraits>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlStyleFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UxmlStyleFactory =>
    "UnityEngine.UIElements"."UxmlStyleFactory"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlStyleFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlStyleFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlStyleTraits>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlStyleFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlStyleFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlStyleFactory")]
impl crate::UnityEngine::UIElements::UxmlStyleFactory {
    pub fn Create(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("Create", (bag, cc))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_uxmlName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_uxmlName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uxmlQualifiedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_uxmlQualifiedName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlStyleFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlStyleFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
