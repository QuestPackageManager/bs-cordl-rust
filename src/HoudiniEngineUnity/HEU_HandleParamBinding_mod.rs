#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HandleParamBinding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _paramType: crate::HoudiniEngineUnity::HEU_HandleParamBinding_HEU_HandleParamType,
    pub _parmID: i32,
    pub _paramName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _bDisabled: bool,
    pub _boundChannels: *mut quest_hook::libil2cpp::Il2CppArray<bool>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_HandleParamBinding =>
    "HoudiniEngineUnity"."HEU_HandleParamBinding"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HandleParamBinding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_HandleParamBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding")]
impl crate::HoudiniEngineUnity::HEU_HandleParamBinding {
    #[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding+HEU_HandleParamType")]
    pub type HEU_HandleParamType = crate::HoudiniEngineUnity::HEU_HandleParamBinding_HEU_HandleParamType;
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HandleParamBinding,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_HandleParamBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding+HEU_HandleParamType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_HandleParamBinding_HEU_HandleParamType {
    ROTATE = 1i32,
    SCALE = 2i32,
    TRANSLATE = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HandleParamBinding+HEU_HandleParamType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HandleParamBinding_HEU_HandleParamType =>
    "HoudiniEngineUnity"."HEU_HandleParamBinding/HEU_HandleParamType"
);
