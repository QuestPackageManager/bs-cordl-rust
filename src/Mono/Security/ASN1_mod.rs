#[cfg(feature = "Mono+Security+ASN1")]
#[repr(C)]
#[derive(Debug)]
pub struct ASN1 {
    __cordl_parent: crate::System::Object,
    pub m_nTag: u8,
    pub m_aValue: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub elist: *mut crate::System::Collections::ArrayList,
}
#[cfg(feature = "Mono+Security+ASN1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::ASN1 => "Mono.Security"."ASN1"
);
#[cfg(feature = "Mono+Security+ASN1")]
impl std::ops::Deref for crate::Mono::Security::ASN1 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+ASN1")]
impl std::ops::DerefMut for crate::Mono::Security::ASN1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+ASN1")]
impl crate::Mono::Security::ASN1 {
    pub fn Add(
        &mut self,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("Add", (asn1))?;
        Ok(__cordl_ret)
    }
    pub fn CompareArray(
        &mut self,
        array1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        array2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompareArray", (array1, array2))?;
        Ok(__cordl_ret)
    }
    pub fn CompareValue(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompareValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        asn1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        anPos: quest_hook::libil2cpp::ByRefMut<i32>,
        anLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decode", (asn1, anPos, anLength))?;
        Ok(__cordl_ret)
    }
    pub fn DecodeTLV(
        &mut self,
        asn1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
        tag: quest_hook::libil2cpp::ByRefMut<u8>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        content: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecodeTLV", (asn1, pos, tag, length, content))?;
        Ok(__cordl_ret)
    }
    pub fn Element(
        &mut self,
        index: i32,
        anTag: u8,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("Element", (index, anTag))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray2(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object)
    }
    pub fn New_u8_0(tag: u8) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag))?;
        Ok(__cordl_object)
    }
    pub fn New_u8_Il2CppArray1(
        tag: u8,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag, data))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_0(
        &mut self,
        tag: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_Il2CppArray1(
        &mut self,
        tag: u8,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag, data))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Tag(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_Tag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Value(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+ASN1")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::ASN1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
