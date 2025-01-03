#[cfg(feature = "Mono+Security+Cryptography+DSAManaged")]
#[repr(C)]
#[derive(Debug)]
pub struct DSAManaged {
    __cordl_parent: crate::System::Security::Cryptography::DSA,
    pub keypairGenerated: bool,
    pub m_disposed: bool,
    pub p: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub q: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub g: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub x: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub y: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub j: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub seed: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub counter: i32,
    pub j_missing: bool,
    pub rng: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::RandomNumberGenerator,
    >,
    pub KeyGenerated: quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler,
    >,
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::DSAManaged =>
    "Mono.Security.Cryptography"."DSAManaged"
);
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::DSAManaged {
    type Target = crate::System::Security::Cryptography::DSA;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::DSAManaged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged")]
impl crate::Mono::Security::Cryptography::DSAManaged {
    #[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
    pub type KeyGeneratedEventHandler = crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler;
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExportParameters(
        &mut self,
        includePrivateParameters: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::DSAParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::DSAParameters = __cordl_object
            .invoke("ExportParameters", (includePrivateParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Generate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKeyPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateKeyPair", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateParams(
        &mut self,
        keyLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateParams", (keyLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportParameters(
        &mut self,
        parameters: crate::System::Security::Cryptography::DSAParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportParameters", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dwKeySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dwKeySize))?;
        Ok(__cordl_object.into())
    }
    pub fn NormalizeArray(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("NormalizeArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature(
        &mut self,
        rgbHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rgbSignature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifySignature", (rgbHash, rgbSignature))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dwKeySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dwKeySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn add(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add", (a, b, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_KeyGenerated(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_KeyGenerated", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_KeySize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PublicOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        > = __cordl_object.invoke("get_Random", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_KeyGenerated(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_KeyGenerated", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::DSAManaged {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct DSAManaged_KeyGeneratedEventHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler =>
    "Mono.Security.Cryptography"."DSAManaged/KeyGeneratedEventHandler"
);
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
impl std::ops::Deref
for crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
impl std::ops::DerefMut
for crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
impl crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler {
    pub fn Invoke(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (sender, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Cryptography+DSAManaged+KeyGeneratedEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::DSAManaged_KeyGeneratedEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
