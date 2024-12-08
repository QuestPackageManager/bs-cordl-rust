#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
#[repr(C)]
#[derive(Debug)]
pub struct AddToExistingGameObjectComponentProviderGetter {
    __cordl_parent: crate::Zenject::AddToGameObjectComponentProviderBase,
    pub _gameObjectGetter: *mut crate::System::Func_2<
        *mut crate::Zenject::InjectContext,
        *mut crate::UnityEngine::GameObject,
    >,
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::AddToExistingGameObjectComponentProviderGetter => "Zenject"
    ."AddToExistingGameObjectComponentProviderGetter"
);
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
impl std::ops::Deref for crate::Zenject::AddToExistingGameObjectComponentProviderGetter {
    type Target = crate::Zenject::AddToGameObjectComponentProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
impl std::ops::DerefMut
for crate::Zenject::AddToExistingGameObjectComponentProviderGetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
impl crate::Zenject::AddToExistingGameObjectComponentProviderGetter {
    pub fn GetGameObject(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("GetGameObject", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        gameObjectGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::UnityEngine::GameObject,
        >,
        container: *mut crate::Zenject::DiContainer,
        componentType: *mut crate::System::Type,
        extraArguments: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        concreteIdentifier: *mut crate::System::Object,
        instantiateCallback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    gameObjectGetter,
                    container,
                    componentType,
                    extraArguments,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        gameObjectGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::UnityEngine::GameObject,
        >,
        container: *mut crate::Zenject::DiContainer,
        componentType: *mut crate::System::Type,
        extraArguments: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        concreteIdentifier: *mut crate::System::Object,
        instantiateCallback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    gameObjectGetter,
                    container,
                    componentType,
                    extraArguments,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldToggleActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShouldToggleActive", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::AddToExistingGameObjectComponentProviderGetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
