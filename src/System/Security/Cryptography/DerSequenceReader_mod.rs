#[cfg(feature = "System+Security+Cryptography+DerSequenceReader")]
#[repr(C)]
#[derive(Debug)]
pub struct DerSequenceReader {
    __cordl_parent: crate::System::Object,
    pub _data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _end: i32,
    pub _position: i32,
    pub _ContentLength_k__BackingField: i32,
}
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::DerSequenceReader => "System.Security.Cryptography"
    ."DerSequenceReader"
);
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader")]
impl std::ops::Deref for crate::System::Security::Cryptography::DerSequenceReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::DerSequenceReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader")]
impl crate::System::Security::Cryptography::DerSequenceReader {
    #[cfg(feature = "System+Security+Cryptography+DerSequenceReader+__c")]
    pub type __c = crate::System::Security::Cryptography::DerSequenceReader___c;
    #[cfg(feature = "System+Security+Cryptography+DerSequenceReader+DerTag")]
    pub type DerTag = crate::System::Security::Cryptography::DerSequenceReader_DerTag;
    pub fn SkipValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadSequence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::DerSequenceReader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::DerSequenceReader = __cordl_object
            .invoke("ReadSequence", ())?;
        Ok(__cordl_ret)
    }
    pub fn PeekTag(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("PeekTag", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadIA5String(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadIA5String", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadPrintableString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadPrintableString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadBitString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadBitString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadTime(
        &mut self,
        timeTag: crate::System::Security::Cryptography::DerSequenceReader_DerTag,
        formatString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ReadTime", (timeTag, formatString))?;
        Ok(__cordl_ret)
    }
    pub fn ReadT61String(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadT61String", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadOidAsString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadOidAsString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
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
    pub fn _ctor_Il2CppArray_i32_i32_1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerSequenceReader_DerTag_Il2CppArray_i32_i32_2(
        &mut self,
        tagToEat: crate::System::Security::Cryptography::DerSequenceReader_DerTag,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagToEat, data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn ReadUtf8String(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadUtf8String", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadOctetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadOctetString", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ContentLength(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentLength", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReadContentAsBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadContentAsBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadNextEncodedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadNextEncodedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadBoolean(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadBoolean", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadX509Date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ReadX509Date", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::DerSequenceReader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::DerSequenceReader = __cordl_object
            .invoke("ReadSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInteger(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadInteger", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadCollectionWithTag(
        &mut self,
        expected: crate::System::Security::Cryptography::DerSequenceReader_DerTag,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::DerSequenceReader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::DerSequenceReader = __cordl_object
            .invoke("ReadCollectionWithTag", (expected))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBMPString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadBMPString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadIntegerBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadIntegerBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadGeneralizedTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ReadGeneralizedTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn EatTag(
        &mut self,
        expected: crate::System::Security::Cryptography::DerSequenceReader_DerTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EatTag", (expected))?;
        Ok(__cordl_ret)
    }
    pub fn EatLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EatLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadUtcTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ReadUtcTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_i32_1(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data, offset, length))?;
        Ok(__cordl_object)
    }
    pub fn New_DerSequenceReader_DerTag_Il2CppArray_i32_i32_2(
        tagToEat: crate::System::Security::Cryptography::DerSequenceReader_DerTag,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagToEat, data, offset, length))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::DerSequenceReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader+DerTag")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerSequenceReader_DerTag {
    BMPString = 30u8,
    BitString = 3u8,
    Boolean = 1u8,
    GeneralizedTime = 24u8,
    IA5String = 22u8,
    Integer = 2u8,
    Null = 5u8,
    ObjectIdentifier = 6u8,
    OctetString = 4u8,
    PrintableString = 19u8,
    Sequence = 16u8,
    Set = 17u8,
    T61String = 20u8,
    UTCTime = 23u8,
    UTF8String = 12u8,
}
#[cfg(feature = "System+Security+Cryptography+DerSequenceReader+DerTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::DerSequenceReader_DerTag =>
    "System.Security.Cryptography"."DerSequenceReader/DerTag"
);
