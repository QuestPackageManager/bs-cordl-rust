#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SupplementalDataEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct SupplementalDataEntry {
    __cordl_parent: crate::System::Object,
    pub mDataType: i32,
    pub mData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SupplementalDataEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::SupplementalDataEntry =>
    "Org.BouncyCastle.Crypto.Tls"."SupplementalDataEntry"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SupplementalDataEntry")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SupplementalDataEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SupplementalDataEntry")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::SupplementalDataEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SupplementalDataEntry")]
impl crate::Org::BouncyCastle::Crypto::Tls::SupplementalDataEntry {
    pub fn New(
        dataType: i32,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataType, data))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        dataType: i32,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataType, data))?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Data", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DataType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DataType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SupplementalDataEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SupplementalDataEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}