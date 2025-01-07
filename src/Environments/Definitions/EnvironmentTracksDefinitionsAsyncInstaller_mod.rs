#[cfg(feature = "Environments+Definitions+EnvironmentTracksDefinitionsAsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTracksDefinitionsAsyncInstaller {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
        >,
    >,
    pub _environmentTracksDefinitions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
            >,
        >,
    >,
}
#[cfg(feature = "Environments+Definitions+EnvironmentTracksDefinitionsAsyncInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Environments::Definitions::EnvironmentTracksDefinitionsAsyncInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Environments.Definitions";
    const CLASS_NAME: &'static str = "EnvironmentTracksDefinitionsAsyncInstaller";
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
#[cfg(feature = "Environments+Definitions+EnvironmentTracksDefinitionsAsyncInstaller")]
impl std::ops::Deref
for crate::Environments::Definitions::EnvironmentTracksDefinitionsAsyncInstaller {
    type Target = crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Environments+Definitions+EnvironmentTracksDefinitionsAsyncInstaller")]
impl std::ops::DerefMut
for crate::Environments::Definitions::EnvironmentTracksDefinitionsAsyncInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Environments+Definitions+EnvironmentTracksDefinitionsAsyncInstaller")]
impl crate::Environments::Definitions::EnvironmentTracksDefinitionsAsyncInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        tracksDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
                >,
            >,
        >,
        registry: quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (tracksDefinitions, registry))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_assetLabelRuntimeKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_assetLabelRuntimeKey", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Environments+Definitions+EnvironmentTracksDefinitionsAsyncInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Environments::Definitions::EnvironmentTracksDefinitionsAsyncInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
