#[cfg(feature = "TestFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct TestFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _leftViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _rightViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _bottomScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _topScreenViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
}
#[cfg(feature = "TestFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TestFlowCoordinator => ""
    ."TestFlowCoordinator"
);
#[cfg(feature = "TestFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::TestFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TestFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::TestFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TestFlowCoordinator")]
impl crate::GlobalNamespace::TestFlowCoordinator {
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
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
}
#[cfg(feature = "TestFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TestFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
