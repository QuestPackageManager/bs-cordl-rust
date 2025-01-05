#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSRTPUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSRTPUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSRTPUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsSRTPUtils =>
    "Org.BouncyCastle.Crypto.Tls"."TlsSRTPUtils"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSRTPUtils")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSRTPUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSRTPUtils")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsSRTPUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSRTPUtils")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsSRTPUtils {
    pub fn AddUseSrtpExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        useSRTPData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddUseSrtpExtension", (extensions, useSRTPData))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUseSrtpExtension(
        useSrtpData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUseSrtpExtension", (useSrtpData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUseSrtpExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUseSrtpExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadUseSrtpExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::UseSrtpData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUseSrtpExtension", (extensionData))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSRTPUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsSRTPUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
