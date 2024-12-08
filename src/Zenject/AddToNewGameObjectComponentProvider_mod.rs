#[cfg(feature = "Zenject+AddToNewGameObjectComponentProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct AddToNewGameObjectComponentProvider {
    __cordl_parent: crate::Zenject::AddToGameObjectComponentProviderBase,
    pub _gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
}
#[cfg(feature = "Zenject+AddToNewGameObjectComponentProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::AddToNewGameObjectComponentProvider =>
    "Zenject"."AddToNewGameObjectComponentProvider"
);
#[cfg(feature = "Zenject+AddToNewGameObjectComponentProvider")]
impl std::ops::Deref for crate::Zenject::AddToNewGameObjectComponentProvider {
    type Target = crate::Zenject::AddToGameObjectComponentProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+AddToNewGameObjectComponentProvider")]
impl std::ops::DerefMut for crate::Zenject::AddToNewGameObjectComponentProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+AddToNewGameObjectComponentProvider")]
impl crate::Zenject::AddToNewGameObjectComponentProvider {
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
        container: *mut crate::Zenject::DiContainer,
        componentType: *mut crate::System::Type,
        extraArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
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
                    container,
                    componentType,
                    extraArguments,
                    gameObjectBindInfo,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        componentType: *mut crate::System::Type,
        extraArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
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
                    container,
                    componentType,
                    extraArguments,
                    gameObjectBindInfo,
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
#[cfg(feature = "Zenject+AddToNewGameObjectComponentProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::AddToNewGameObjectComponentProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
