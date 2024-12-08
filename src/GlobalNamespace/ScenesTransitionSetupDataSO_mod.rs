#[cfg(feature = "ScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct ScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _scenes_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SceneInfo,
    >,
    pub _sceneSetupDataArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SceneSetupData,
    >,
}
#[cfg(feature = "ScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScenesTransitionSetupDataSO =>
    ""."ScenesTransitionSetupDataSO"
);
#[cfg(feature = "ScenesTransitionSetupDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::ScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::ScenesTransitionSetupDataSO {
    pub fn BeforeScenesWillBeActivatedAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("BeforeScenesWillBeActivatedAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        scenes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SceneInfo,
        >,
        sceneSetupData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SceneSetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (scenes, sceneSetupData))?;
        Ok(__cordl_ret)
    }
    pub fn InstallBindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container))?;
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
    pub fn get_scenes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::SceneInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SceneInfo,
        > = __cordl_object.invoke("get_scenes", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_scenes(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SceneInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scenes", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
