#[cfg(feature = "System+Reflection+SignatureArrayType")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureArrayType {
    __cordl_parent: crate::System::Reflection::SignatureHasElementType,
    pub _rank: i32,
    pub _isMultiDim: bool,
}
#[cfg(feature = "System+Reflection+SignatureArrayType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::SignatureArrayType =>
    "System.Reflection"."SignatureArrayType"
);
#[cfg(feature = "System+Reflection+SignatureArrayType")]
impl std::ops::Deref for crate::System::Reflection::SignatureArrayType {
    type Target = crate::System::Reflection::SignatureHasElementType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+SignatureArrayType")]
impl std::ops::DerefMut for crate::System::Reflection::SignatureArrayType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+SignatureArrayType")]
impl crate::System::Reflection::SignatureArrayType {
    pub fn GetArrayRank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetArrayRank", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsArrayImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsArrayImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsByRefImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsByRefImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPointerImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPointerImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Reflection::SignatureType>,
        rank: i32,
        isMultiDim: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementType, rank, isMultiDim))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        elementType: quest_hook::libil2cpp::Gc<crate::System::Reflection::SignatureType>,
        rank: i32,
        isMultiDim: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementType, rank, isMultiDim))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSZArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSZArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsVariableBoundArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsVariableBoundArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Suffix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Suffix", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+SignatureArrayType")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::SignatureArrayType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
