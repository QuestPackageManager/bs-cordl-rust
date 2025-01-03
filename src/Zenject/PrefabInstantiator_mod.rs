#[cfg(feature = "Zenject+PrefabInstantiator")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabInstantiator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _prefabProvider: *mut crate::Zenject::IPrefabProvider,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _extraArguments: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::TypeValuePair,
    >,
    pub _gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    pub _argumentTarget: *mut crate::System::Type,
    pub _instantiateCallbackTypes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
    pub _instantiateCallback: *mut crate::System::Action_2<
        *mut crate::Zenject::InjectContext,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "Zenject+PrefabInstantiator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabInstantiator => "Zenject"
    ."PrefabInstantiator"
);
#[cfg(feature = "Zenject+PrefabInstantiator")]
impl std::ops::Deref for crate::Zenject::PrefabInstantiator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabInstantiator")]
impl std::ops::DerefMut for crate::Zenject::PrefabInstantiator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabInstantiator")]
impl crate::Zenject::PrefabInstantiator {
    pub fn GetPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("GetPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("Instantiate", (context, args, injectAction))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        argumentTarget: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instantiateCallbackTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::Zenject::InjectContext,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    container,
                    gameObjectBindInfo,
                    argumentTarget,
                    instantiateCallbackTypes,
                    extraArguments,
                    prefabProvider,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        argumentTarget: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instantiateCallbackTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::Zenject::InjectContext,
                *mut quest_hook::libil2cpp::Il2CppObject,
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
                    container,
                    gameObjectBindInfo,
                    argumentTarget,
                    instantiateCallbackTypes,
                    extraArguments,
                    prefabProvider,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgumentTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ArgumentTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtraArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        > = __cordl_object.invoke("get_ExtraArguments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GameObjectCreationParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectCreationParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        > = __cordl_object.invoke("get_GameObjectCreationParameters", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PrefabInstantiator")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabInstantiator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PrefabInstantiator")]
impl AsRef<crate::Zenject::IPrefabInstantiator> for crate::Zenject::PrefabInstantiator {
    fn as_ref(&self) -> &crate::Zenject::IPrefabInstantiator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabInstantiator")]
impl AsMut<crate::Zenject::IPrefabInstantiator> for crate::Zenject::PrefabInstantiator {
    fn as_mut(&mut self) -> &mut crate::Zenject::IPrefabInstantiator {
        unsafe { std::mem::transmute(self) }
    }
}
