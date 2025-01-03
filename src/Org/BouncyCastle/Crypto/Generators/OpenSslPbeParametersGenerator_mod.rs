#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+OpenSslPbeParametersGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OpenSslPbeParametersGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::PbeParametersGenerator,
    pub digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+OpenSslPbeParametersGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::OpenSslPbeParametersGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."OpenSslPbeParametersGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+OpenSslPbeParametersGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::OpenSslPbeParametersGenerator {
    type Target = crate::Org::BouncyCastle::Crypto::PbeParametersGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+OpenSslPbeParametersGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::OpenSslPbeParametersGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+OpenSslPbeParametersGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::OpenSslPbeParametersGenerator {
    pub fn GenerateDerivedKey(
        &mut self,
        bytesNeeded: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateDerivedKey", (bytesNeeded))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDerivedMacParameters(
        &mut self,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object.invoke("GenerateDerivedMacParameters", (keySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDerivedParameters_Il2CppString_i32_1(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object.invoke("GenerateDerivedParameters", (algorithm, keySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDerivedParameters_Il2CppString_i32_i32_3(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keySize: i32,
        ivSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object
            .invoke("GenerateDerivedParameters", (algorithm, keySize, ivSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDerivedParameters_i32_0(
        &mut self,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object.invoke("GenerateDerivedParameters", (keySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDerivedParameters_i32_i32_2(
        &mut self,
        keySize: i32,
        ivSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object.invoke("GenerateDerivedParameters", (keySize, ivSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_Il2CppArray_Il2CppArray1(
        &mut self,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (password, salt))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_i32_0(
        &mut self,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (password, salt, iterationCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+OpenSslPbeParametersGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::OpenSslPbeParametersGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
