#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct PemUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::PemUtilities =>
    "Org.BouncyCastle.OpenSsl"."PemUtilities"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::PemUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::PemUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities")]
impl crate::Org::BouncyCastle::OpenSsl::PemUtilities {
    #[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities+PemBaseAlg")]
    pub type PemBaseAlg = crate::Org::BouncyCastle::OpenSsl::PemUtilities_PemBaseAlg;
    #[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities+PemMode")]
    pub type PemMode = crate::Org::BouncyCastle::OpenSsl::PemUtilities_PemMode;
    pub fn Crypt(
        encrypt: bool,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        dekAlgName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Crypt", (encrypt, bytes, password, dekAlgName, iv))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCipherParameters(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        baseAlg: crate::Org::BouncyCastle::OpenSsl::PemUtilities_PemBaseAlg,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCipherParameters", (password, baseAlg, salt))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseDekAlgName(
        dekAlgName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseAlg: quest_hook::libil2cpp::ByRefMut<
            crate::Org::BouncyCastle::OpenSsl::PemUtilities_PemBaseAlg,
        >,
        mode: quest_hook::libil2cpp::ByRefMut<
            crate::Org::BouncyCastle::OpenSsl::PemUtilities_PemMode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDekAlgName", (dekAlgName, baseAlg, mode))?;
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
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::OpenSsl::PemUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities+PemBaseAlg")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PemUtilities_PemBaseAlg {
    AES_128 = 0i32,
    AES_192 = 1i32,
    AES_256 = 2i32,
    BF = 3i32,
    DES = 4i32,
    DES_EDE = 5i32,
    DES_EDE3 = 6i32,
    RC2 = 7i32,
    RC2_40 = 8i32,
    RC2_64 = 9i32,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities+PemBaseAlg")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::OpenSsl::PemUtilities_PemBaseAlg => "Org.BouncyCastle.OpenSsl"
    ."PemUtilities/PemBaseAlg"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities+PemMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PemUtilities_PemMode {
    CBC = 0i32,
    CFB = 1i32,
    ECB = 2i32,
    OFB = 3i32,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemUtilities+PemMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::PemUtilities_PemMode
    => "Org.BouncyCastle.OpenSsl"."PemUtilities/PemMode"
);
