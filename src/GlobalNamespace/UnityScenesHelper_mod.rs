#[cfg(feature = "UnityScenesHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityScenesHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityScenesHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UnityScenesHelper => ""
    ."UnityScenesHelper"
);
#[cfg(feature = "UnityScenesHelper")]
impl std::ops::Deref for crate::GlobalNamespace::UnityScenesHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityScenesHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl crate::GlobalNamespace::UnityScenesHelper {
    pub fn GetComponentsInScene<T>(
        scene: crate::UnityEngine::SceneManagement::Scene,
        components: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetComponentsInScene", (scene, components, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveRootObjectsInScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetActiveRootObjectsInScene", (scene, active))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UnityScenesHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
