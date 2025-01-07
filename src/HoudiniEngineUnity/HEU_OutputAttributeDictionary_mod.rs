#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttributeDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_OutputAttributeDictionary {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_SerializableDictionary_2<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_OutputAttribute>,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttributeDictionary")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_OutputAttributeDictionary {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_OutputAttributeDictionary";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttributeDictionary")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_OutputAttributeDictionary {
    type Target = crate::HoudiniEngineUnity::HEU_SerializableDictionary_2<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_OutputAttribute>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttributeDictionary")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_OutputAttributeDictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttributeDictionary")]
impl crate::HoudiniEngineUnity::HEU_OutputAttributeDictionary {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_OutputAttributeDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_OutputAttributeDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
