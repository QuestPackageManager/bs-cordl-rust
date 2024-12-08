#[cfg(feature = "HoudiniEngineUnity+HEU_InputPreset")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputPreset {
    __cordl_parent: crate::System::Object,
    pub _inputObjectType: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    pub _inputObjectPresets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputObjectPreset,
    >,
    pub _inputAssetName: *mut crate::System::String,
    pub _inputIndex: i32,
    pub _inputName: *mut crate::System::String,
    pub _keepWorldTransform: bool,
    pub _packGeometryBeforeMerging: bool,
    pub _inputAssetPresets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputAssetPreset,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputPreset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputPreset =>
    "HoudiniEngineUnity"."HEU_InputPreset"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputPreset")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputPreset {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputPreset")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputPreset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputPreset")]
impl crate::HoudiniEngineUnity::HEU_InputPreset {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputPreset")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_InputPreset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
