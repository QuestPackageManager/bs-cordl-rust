#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AttributeData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _attributeInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _attributeType: crate::HoudiniEngineUnity::HEU_AttributeData_AttributeType,
    pub _intValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _floatValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _stringValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _attributeState: crate::HoudiniEngineUnity::HEU_AttributeData_AttributeState,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AttributeData =>
    "HoudiniEngineUnity"."HEU_AttributeData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AttributeData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AttributeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
impl crate::HoudiniEngineUnity::HEU_AttributeData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData+AttributeState")]
    pub type AttributeState = crate::HoudiniEngineUnity::HEU_AttributeData_AttributeState;
    #[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData+AttributeType")]
    pub type AttributeType = crate::HoudiniEngineUnity::HEU_AttributeData_AttributeType;
    pub fn CopyValuesTo(
        &mut self,
        destAttrData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValuesTo", (destAttrData))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsColorAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsColorAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
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
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_AttributeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    >,
> for crate::HoudiniEngineUnity::HEU_AttributeData {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    >,
> for crate::HoudiniEngineUnity::HEU_AttributeData {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData+AttributeState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_AttributeData_AttributeState {
    #[default]
    INVALID = 0i32,
    LOCAL_DIRTY = 2i32,
    SYNCED = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData+AttributeState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_AttributeData_AttributeState => "HoudiniEngineUnity"
    ."HEU_AttributeData/AttributeState"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData+AttributeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_AttributeData_AttributeType {
    #[default]
    BOOL = 0i32,
    FLOAT = 2i32,
    INT = 1i32,
    MAX = 4i32,
    STRING = 3i32,
    UNDEFINED = -1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributeData+AttributeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_AttributeData_AttributeType => "HoudiniEngineUnity"
    ."HEU_AttributeData/AttributeType"
);
