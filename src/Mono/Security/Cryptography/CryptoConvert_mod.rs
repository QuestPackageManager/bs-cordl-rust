#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoConvert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::CryptoConvert =>
    "Mono.Security.Cryptography"."CryptoConvert"
);
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::CryptoConvert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::CryptoConvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl crate::Mono::Security::Cryptography::CryptoConvert {
    pub fn FromCapiPrivateKeyBlob_Il2CppArray0(
        blob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCapiPrivateKeyBlob", (blob))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCapiPrivateKeyBlob_i32_1(
        blob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCapiPrivateKeyBlob", (blob, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParametersFromCapiPrivateKeyBlob(
        blob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParametersFromCapiPrivateKeyBlob", (blob, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHex(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToHex", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32LE(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32LE", (bytes, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32LE(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32LE", (bytes, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Trim", (array))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::CryptoConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
