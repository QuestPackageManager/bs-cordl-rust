#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+SignatureExpirationTime")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureExpirationTime {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+SignatureExpirationTime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::Sig::SignatureExpirationTime =>
    "Org.BouncyCastle.Bcpg.Sig"."SignatureExpirationTime"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+SignatureExpirationTime")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::Sig::SignatureExpirationTime {
    type Target = crate::Org::BouncyCastle::Bcpg::SignatureSubpacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+SignatureExpirationTime")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::Sig::SignatureExpirationTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+SignatureExpirationTime")]
impl crate::Org::BouncyCastle::Bcpg::Sig::SignatureExpirationTime {
    pub fn _ctor__cordl_bool_Il2CppArray0(
        &mut self,
        critical: bool,
        isLongLength: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (critical, isLongLength, data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_1(
        &mut self,
        critical: bool,
        seconds: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (critical, seconds))?;
        Ok(__cordl_ret)
    }
    pub fn get_Time(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Time", ())?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool_Il2CppArray0(
        critical: bool,
        isLongLength: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (critical, isLongLength, data))?;
        Ok(__cordl_object)
    }
    pub fn New_i64_1(
        critical: bool,
        seconds: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (critical, seconds))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+SignatureExpirationTime")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::Sig::SignatureExpirationTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
