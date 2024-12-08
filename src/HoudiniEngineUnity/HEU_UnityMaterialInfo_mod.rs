#[cfg(feature = "HoudiniEngineUnity+HEU_UnityMaterialInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_UnityMaterialInfo {
    __cordl_parent: crate::System::Object,
    pub _unityMaterialPath: *mut crate::System::String,
    pub _substancePath: *mut crate::System::String,
    pub _substanceIndex: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_UnityMaterialInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_UnityMaterialInfo =>
    "HoudiniEngineUnity"."HEU_UnityMaterialInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_UnityMaterialInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_UnityMaterialInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_UnityMaterialInfo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_UnityMaterialInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_UnityMaterialInfo")]
impl crate::HoudiniEngineUnity::HEU_UnityMaterialInfo {
    pub fn _ctor(
        &mut self,
        unityMaterialPath: *mut crate::System::String,
        substancePath: *mut crate::System::String,
        substanceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unityMaterialPath, substancePath, substanceIndex))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        unityMaterialPath: *mut crate::System::String,
        substancePath: *mut crate::System::String,
        substanceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unityMaterialPath, substancePath, substanceIndex))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_UnityMaterialInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_UnityMaterialInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
