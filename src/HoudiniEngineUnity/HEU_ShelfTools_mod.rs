#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ShelfTools {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ShelfTools =>
    "HoudiniEngineUnity"."HEU_ShelfTools"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ShelfTools {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ShelfTools {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl crate::HoudiniEngineUnity::HEU_ShelfTools {
    pub const TARGET_ALL: &'static str = "all";
    pub const TARGET_UNITY: &'static str = "unity";
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ShelfTools {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
