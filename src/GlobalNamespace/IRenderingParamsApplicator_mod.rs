#[cfg(feature = "IRenderingParamsApplicator")]
#[repr(C)]
#[derive(Debug)]
pub struct IRenderingParamsApplicator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IRenderingParamsApplicator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IRenderingParamsApplicator =>
    ""."IRenderingParamsApplicator"
);
#[cfg(feature = "IRenderingParamsApplicator")]
impl std::ops::Deref for crate::GlobalNamespace::IRenderingParamsApplicator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IRenderingParamsApplicator")]
impl std::ops::DerefMut for crate::GlobalNamespace::IRenderingParamsApplicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IRenderingParamsApplicator")]
impl crate::GlobalNamespace::IRenderingParamsApplicator {
    pub fn Apply(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (sceneType, optionalEnvironmentSerializedName))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IRenderingParamsApplicator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IRenderingParamsApplicator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
