#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_OutputAttribute {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _class: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
    pub _type: crate::HoudiniEngineUnity::HAPI_StorageType,
    pub _count: i32,
    pub _tupleSize: i32,
    pub _intValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _floatValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _stringValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_OutputAttribute =>
    "HoudiniEngineUnity"."HEU_OutputAttribute"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttribute")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_OutputAttribute {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttribute")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_OutputAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttribute")]
impl crate::HoudiniEngineUnity::HEU_OutputAttribute {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_OutputAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
