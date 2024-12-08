#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+HkdfParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct HkdfParameters {
    __cordl_parent: crate::System::Object,
    pub ikm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub skipExpand: bool,
    pub salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub info: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+HkdfParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::HkdfParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."HkdfParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+HkdfParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::HkdfParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+HkdfParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::HkdfParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+HkdfParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::HkdfParameters {
    pub fn GetInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SkipExtract(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SkipExtract", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_Il2CppArray0(
        &mut self,
        ikm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        skip: bool,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        info: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ikm, skip, salt, info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        ikm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        info: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ikm, salt, info))?;
        Ok(__cordl_ret)
    }
    pub fn GetIkm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetIkm", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSalt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSalt", ())?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool_Il2CppArray0(
        ikm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        skip: bool,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        info: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ikm, skip, salt, info))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        ikm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        info: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ikm, salt, info))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+HkdfParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::HkdfParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
