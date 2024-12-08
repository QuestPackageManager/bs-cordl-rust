#[cfg(feature = "Zenject+SceneDecoratorContext")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneDecoratorContext {
    __cordl_parent: crate::Zenject::Context,
    pub _lateInstallers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::MonoInstaller,
    >,
    pub _lateInstallerPrefabs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::MonoInstaller,
    >,
    pub _lateScriptableObjectInstallers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::ScriptableObjectInstaller,
    >,
    pub _decoratedContractName: *mut crate::System::String,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _injectableMonoBehaviours: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::MonoBehaviour,
    >,
}
#[cfg(feature = "Zenject+SceneDecoratorContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SceneDecoratorContext => "Zenject"
    ."SceneDecoratorContext"
);
#[cfg(feature = "Zenject+SceneDecoratorContext")]
impl std::ops::Deref for crate::Zenject::SceneDecoratorContext {
    type Target = crate::Zenject::Context;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SceneDecoratorContext")]
impl std::ops::DerefMut for crate::Zenject::SceneDecoratorContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SceneDecoratorContext")]
impl crate::Zenject::SceneDecoratorContext {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LateInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        > = __cordl_object.invoke("get_LateInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_LateInstallers(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LateInstallers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_LateScriptableObjectInstallers(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::ScriptableObjectInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LateScriptableObjectInstallers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("get_Container", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (container))?;
        Ok(__cordl_ret)
    }
    pub fn InstallDecoratorInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallDecoratorInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRootGameObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object.invoke("GetRootGameObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LateScriptableObjectInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::ScriptableObjectInstaller,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::ScriptableObjectInstaller,
        > = __cordl_object.invoke("get_LateScriptableObjectInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInjectableMonoBehaviours(
        &mut self,
        monoBehaviours: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInjectableMonoBehaviours", (monoBehaviours))?;
        Ok(__cordl_ret)
    }
    pub fn InstallLateDecoratorInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallLateDecoratorInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DecoratedContractName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DecoratedContractName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LateInstallerPrefabs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        > = __cordl_object.invoke("get_LateInstallerPrefabs", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallDecoratorSceneBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallDecoratorSceneBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_LateInstallerPrefabs(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LateInstallerPrefabs", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SceneDecoratorContext")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SceneDecoratorContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
