#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
#[repr(C)]
#[derive(Debug)]
pub struct AddToExistingGameObjectComponentProviderGetter {
    __cordl_parent: crate::Zenject::AddToGameObjectComponentProviderBase,
    pub _gameObjectGetter: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
}
#[cfg(feature = "Zenject+AddToExistingGameObjectComponentProviderGetter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::AddToExistingGameObjectComponentProviderGetter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "AddToExistingGameObjectComponentProviderGetter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("GetGameObject", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldToggleActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShouldToggleActive", ())?;
        Ok(__cordl_ret.into())
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
