#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ShelfToolData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _toolType: crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType,
    pub _toolTip: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _iconPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _helpURL: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _targets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _jsonPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ShelfToolData =>
    "HoudiniEngineUnity"."HEU_ShelfToolData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl crate::HoudiniEngineUnity::HEU_ShelfToolData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
    pub type ToolType = crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_ShelfToolData_ToolType {
    BATCH = 3i32,
    GENERATOR = 0i32,
    OPERATOR_MULTI = 2i32,
    OPERATOR_SINGLE = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ShelfToolData_ToolType
    => "HoudiniEngineUnity"."HEU_ShelfToolData/ToolType"
);
