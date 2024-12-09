#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct AddToExistingGameObjectComponentProvider {
    __cordl_parent: crate::Zenject::AddToGameObjectComponentProviderBase,
    pub _gameObject: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::AddToExistingGameObjectComponentProvider => "Zenject"
    ."AddToExistingGameObjectComponentProvider"
);
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProvider")]
impl std::ops::Deref for crate::Zenject::AddToExistingGameObjectComponentProvider {
    type Target = crate::Zenject::AddToGameObjectComponentProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProvider")]
impl std::ops::DerefMut for crate::Zenject::AddToExistingGameObjectComponentProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProvider")]
impl crate::Zenject::AddToExistingGameObjectComponentProvider {
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
        gameObject: *mut crate::UnityEngine::GameObject,
        container: *mut crate::Zenject::DiContainer,
        componentType: *mut crate::System::Type,
        extraArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
        concreteIdentifier: *mut quest_hook::libil2cpp::Il2CppObject,
        instantiateCallback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    gameObject,
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
        gameObject: *mut crate::UnityEngine::GameObject,
        container: *mut crate::Zenject::DiContainer,
        componentType: *mut crate::System::Type,
        extraArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
        concreteIdentifier: *mut quest_hook::libil2cpp::Il2CppObject,
        instantiateCallback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    gameObject,
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
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::AddToExistingGameObjectComponentProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
