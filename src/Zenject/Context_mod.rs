#[cfg(feature = "Zenject+Context")]
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scriptableObjectInstallers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::ScriptableObjectInstaller,
    >,
    pub _monoInstallers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::MonoInstaller,
    >,
    pub _installerPrefabs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::MonoInstaller,
    >,
    pub _normalInstallers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::InstallerBase,
    >,
    pub _normalInstallerTypes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
}
#[cfg(feature = "Zenject+Context")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Context => "Zenject"."Context"
);
#[cfg(feature = "Zenject+Context")]
impl std::ops::Deref for crate::Zenject::Context {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Context")]
impl std::ops::DerefMut for crate::Zenject::Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Context")]
impl crate::Zenject::Context {
    #[cfg(feature = "Zenject+Context+__c")]
    pub type __c = crate::Zenject::Context___c;
    pub fn AddNormalInstaller(
        &mut self,
        installer: *mut crate::Zenject::InstallerBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNormalInstaller", (installer))?;
        Ok(__cordl_ret)
    }
    pub fn AddNormalInstallerType(
        &mut self,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNormalInstallerType", (installerType))?;
        Ok(__cordl_ret)
    }
    pub fn CheckInstallerPrefabTypes(
        &mut self,
        installers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::MonoInstaller,
        >,
        installerPrefabs: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckInstallerPrefabTypes", (installers, installerPrefabs))?;
        Ok(__cordl_ret)
    }
    pub fn GetInjectableMonoBehaviours(
        &mut self,
        components: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInjectableMonoBehaviours", (components))?;
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
    pub fn InstallInstallers_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallInstallers_List_1_List_1_List_1_List_1_List_1_1(
        &mut self,
        normalInstallers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::InstallerBase,
        >,
        normalInstallerTypes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Type,
        >,
        scriptableObjectInstallers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::ScriptableObjectInstaller,
        >,
        installers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::MonoInstaller,
        >,
        installerPrefabs: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InstallInstallers",
                (
                    normalInstallers,
                    normalInstallerTypes,
                    scriptableObjectInstallers,
                    installers,
                    installerPrefabs,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstallSceneBindings(
        &mut self,
        injectableMonoBehaviours: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallSceneBindings", (injectableMonoBehaviours))?;
        Ok(__cordl_ret)
    }
    pub fn InstallZenjectBinding(
        &mut self,
        binding: *mut crate::Zenject::ZenjectBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallZenjectBinding", (binding))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_InstallerPrefabs(
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
        > = __cordl_object.invoke("get_InstallerPrefabs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Installers(
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
        > = __cordl_object.invoke("get_Installers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NormalInstallerTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("get_NormalInstallerTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NormalInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::InstallerBase,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::InstallerBase,
        > = __cordl_object.invoke("get_NormalInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ScriptableObjectInstallers(
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
        > = __cordl_object.invoke("get_ScriptableObjectInstallers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InstallerPrefabs(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InstallerPrefabs", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Installers(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::MonoInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Installers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_NormalInstallerTypes(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NormalInstallerTypes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_NormalInstallers(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::InstallerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NormalInstallers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ScriptableObjectInstallers(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::ScriptableObjectInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ScriptableObjectInstallers", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+Context")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Context {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
