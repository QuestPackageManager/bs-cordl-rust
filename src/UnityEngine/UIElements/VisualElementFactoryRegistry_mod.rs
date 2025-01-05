#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementFactoryRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementFactoryRegistry => "UnityEngine.UIElements"
    ."VisualElementFactoryRegistry"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    pub fn GetMovedUIControlTypeName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        attr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMovedUIControlTypeName", (_cordl_type, attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterEngineFactories() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterEngineFactories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterFactory(
        factory: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlFactory>,
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
    pub fn TryGetValue(
        fullTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        factoryList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IUxmlFactory,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetValue", (fullTypeName, factoryList))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_factories() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::IUxmlFactory,
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
                            crate::UnityEngine::UIElements::IUxmlFactory,
                        >,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_factories", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
