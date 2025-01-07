#[cfg(feature = "Mono+Security+Cryptography+PKCS1")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Cryptography+PKCS1")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Security::Cryptography::PKCS1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security.Cryptography";
    const CLASS_NAME: &'static str = "PKCS1";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Mono+Security+Cryptography+PKCS1")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::PKCS1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+PKCS1")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::PKCS1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+PKCS1")]
impl crate::Mono::Security::Cryptography::PKCS1 {
    pub fn Compare(
        array1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        array2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (array1, array2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::HashAlgorithm>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::HashAlgorithm,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromOid(
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::HashAlgorithm>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::HashAlgorithm,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromOid", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode_v15(
        hash: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::HashAlgorithm,
        >,
        hashValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        emLength: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Encode_v15", (hash, hashValue, emLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashNameFromOid(
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwOnError: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashNameFromOid", (oid, throwOnError))?;
        Ok(__cordl_ret.into())
    }
    pub fn I2OSP(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("I2OSP", (x, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn OS2IP(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("OS2IP", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn RSAVP1(
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("RSAVP1", (rsa, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_v15(
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
        hash: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::HashAlgorithm,
        >,
        hashValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        tryNonStandardEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Verify_v15",
                (rsa, hash, hashValue, signature, tryNonStandardEncoding),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Cryptography+PKCS1")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::Cryptography::PKCS1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
