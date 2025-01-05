#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlObjectFactoryRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UxmlObjectFactoryRegistry => "UnityEngine.UIElements"
    ."UxmlObjectFactoryRegistry"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    pub fn RegisterEngineFactories() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterEngineFactories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterFactory(
        factory: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterFactory", (factory))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterUserFactories() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterUserFactories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetFactories(
        fullTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        factoryList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetFactories", (fullTypeName, factoryList))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_factories() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
                        >,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
                        >,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_factories", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
