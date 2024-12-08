#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerSrpParams")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerSrpParams {
    __cordl_parent: crate::System::Object,
    pub m_N: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_B: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_s: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerSrpParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::ServerSrpParams
    => "Org.BouncyCastle.Crypto.Tls"."ServerSrpParams"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerSrpParams")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::ServerSrpParams {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerSrpParams")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::ServerSrpParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerSrpParams")]
impl crate::Org::BouncyCastle::Crypto::Tls::ServerSrpParams {
    pub fn get_B(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_B", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        N: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        s: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        B: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (N, g, s, B))?;
        Ok(__cordl_ret)
    }
    pub fn get_S(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_S", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_G(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_G", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_N(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_N", ())?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        N: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        s: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        B: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (N, g, s, B))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerSrpParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::ServerSrpParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
