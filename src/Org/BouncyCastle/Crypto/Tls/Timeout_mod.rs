#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
#[repr(C)]
#[derive(Debug)]
pub struct Timeout {
    __cordl_parent: crate::System::Object,
    pub durationMillis: i64,
    pub startMillis: i64,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::Timeout =>
    "Org.BouncyCastle.Crypto.Tls"."Timeout"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    pub fn RemainingMillis(
        &mut self,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("RemainingMillis", (currentTimeMillis))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_0(
        &mut self,
        durationMillis: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMillis))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_1(
        &mut self,
        durationMillis: i64,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMillis, currentTimeMillis))?;
        Ok(__cordl_ret)
    }
    pub fn New_i64_0(
        durationMillis: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMillis))?;
        Ok(__cordl_object)
    }
    pub fn New_i64_1(
        durationMillis: i64,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMillis, currentTimeMillis))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
