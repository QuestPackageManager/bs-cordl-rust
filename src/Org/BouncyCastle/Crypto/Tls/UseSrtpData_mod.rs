#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+UseSrtpData")]
#[repr(C)]
#[derive(Debug)]
pub struct UseSrtpData {
    __cordl_parent: crate::System::Object,
    pub mProtectionProfiles: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub mMki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+UseSrtpData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::UseSrtpData =>
    "Org.BouncyCastle.Crypto.Tls"."UseSrtpData"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+UseSrtpData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+UseSrtpData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+UseSrtpData")]
impl crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData {
    pub fn get_Mki(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Mki", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        protectionProfiles: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        mki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (protectionProfiles, mki))?;
        Ok(__cordl_ret)
    }
    pub fn get_ProtectionProfiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("get_ProtectionProfiles", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        protectionProfiles: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        mki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (protectionProfiles, mki))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+UseSrtpData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
