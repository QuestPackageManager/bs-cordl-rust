#[cfg(feature = "MenuSceneSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuSceneSetup {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rootFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _hierarchyManager: *mut crate::HMUI::HierarchyManager,
}
#[cfg(feature = "MenuSceneSetup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MenuSceneSetup => ""."MenuSceneSetup"
);
#[cfg(feature = "MenuSceneSetup")]
impl std::ops::Deref for MenuSceneSetup {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuSceneSetup")]
impl std::ops::DerefMut for MenuSceneSetup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuSceneSetup")]
impl MenuSceneSetup {
    #[cfg(feature = "MenuSceneSetup+_Start_d__3")]
    pub type _Start_d__3 = crate::GlobalNamespace::MenuSceneSetup__Start_d__3;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
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
}
#[cfg(feature = "MenuSceneSetup")]
impl quest_hook::libil2cpp::ObjectType for MenuSceneSetup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
