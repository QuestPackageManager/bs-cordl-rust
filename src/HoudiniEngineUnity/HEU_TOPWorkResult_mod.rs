#[cfg(feature = "HoudiniEngineUnity+HEU_TOPWorkResult")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TOPWorkResult {
    __cordl_parent: crate::System::Object,
    pub _workItemIndex: i32,
    pub _workItemID: i32,
    pub _generatedGOs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::GameObject,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPWorkResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_TOPWorkResult =>
    "HoudiniEngineUnity"."HEU_TOPWorkResult"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPWorkResult")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_TOPWorkResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPWorkResult")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_TOPWorkResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPWorkResult")]
impl crate::HoudiniEngineUnity::HEU_TOPWorkResult {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPWorkResult")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_TOPWorkResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
