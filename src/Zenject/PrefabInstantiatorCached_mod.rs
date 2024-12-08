#[cfg(feature = "Zenject+PrefabInstantiatorCached")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabInstantiatorCached {
    __cordl_parent: crate::System::Object,
    pub _subInstantiator: *mut crate::Zenject::IPrefabInstantiator,
    pub _gameObject: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "Zenject+PrefabInstantiatorCached")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabInstantiatorCached => "Zenject"
    ."PrefabInstantiatorCached"
);
#[cfg(feature = "Zenject+PrefabInstantiatorCached")]
impl std::ops::Deref for crate::Zenject::PrefabInstantiatorCached {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabInstantiatorCached")]
impl std::ops::DerefMut for crate::Zenject::PrefabInstantiatorCached {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabInstantiatorCached")]
impl crate::Zenject::PrefabInstantiatorCached {
    pub fn get_ExtraArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        > = __cordl_object.invoke("get_ExtraArguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgumentTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ArgumentTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GameObjectCreationParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::GameObjectCreationParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::GameObjectCreationParameters = __cordl_object
            .invoke("get_GameObjectCreationParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        subInstantiator: *mut crate::Zenject::IPrefabInstantiator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subInstantiator))?;
        Ok(__cordl_ret)
    }
    pub fn GetPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("GetPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("Instantiate", (context, args, injectAction))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        subInstantiator: *mut crate::Zenject::IPrefabInstantiator,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subInstantiator))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+PrefabInstantiatorCached")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabInstantiatorCached {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
