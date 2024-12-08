#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Pkcs12ParametersGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs12ParametersGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::PbeParametersGenerator,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub u: i32,
    pub v: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Pkcs12ParametersGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::Pkcs12ParametersGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."Pkcs12ParametersGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Pkcs12ParametersGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::Pkcs12ParametersGenerator {
    type Target = crate::Org::BouncyCastle::Crypto::PbeParametersGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Pkcs12ParametersGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::Pkcs12ParametersGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Pkcs12ParametersGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::Pkcs12ParametersGenerator {
    pub const IVMaterial: i32 = 2i32;
    pub const KeyMaterial: i32 = 1i32;
    pub const MacMaterial: i32 = 3i32;
    pub fn Adjust(
        &mut self,
        a: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        aOff: i32,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Adjust", (a, aOff, b))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateDerivedKey(
        &mut self,
        idByte: i32,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateDerivedKey", (idByte, n))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateDerivedMacParameters(
        &mut self,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("GenerateDerivedMacParameters", (keySize))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateDerivedParameters_String_i32_1(
        &mut self,
        algorithm: *mut crate::System::String,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("GenerateDerivedParameters", (algorithm, keySize))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateDerivedParameters_String_i32_i32_3(
        &mut self,
        algorithm: *mut crate::System::String,
        keySize: i32,
        ivSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("GenerateDerivedParameters", (algorithm, keySize, ivSize))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateDerivedParameters_i32_0(
        &mut self,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("GenerateDerivedParameters", (keySize))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateDerivedParameters_i32_i32_2(
        &mut self,
        keySize: i32,
        ivSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("GenerateDerivedParameters", (keySize, ivSize))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Pkcs12ParametersGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::Pkcs12ParametersGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
