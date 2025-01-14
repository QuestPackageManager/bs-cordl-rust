#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabWithParams {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
    pub _installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _gameObjectBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::GameObjectCreationParameters,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SubContainerCreatorByNewPrefabWithParams";
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
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    pub fn CreateSubContainer(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        parentContext: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Zenject::TypeValuePair,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                ),
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                2usize,
            >("CreateSubContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateSubContainer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            method.invoke_unchecked(self, (args, parentContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTempContainer(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::Zenject::TypeValuePair,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                1usize,
            >("CreateTempContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateTempContainer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            method.invoke_unchecked(self, (args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (installerType, container, prefabProvider, gameObjectBindInfo),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::GameObjectCreationParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (installerType, container, prefabProvider, gameObjectBindInfo),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                0usize,
            >("get_Container")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Container", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl AsRef<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn as_ref(&self) -> &crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl AsMut<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn as_mut(&mut self) -> &mut crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
