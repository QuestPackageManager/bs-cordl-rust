#[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct RSAManaged_KeyGeneratedEventHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler =>
    "Mono.Security.Cryptography"."RSAManaged/KeyGeneratedEventHandler"
);
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
impl std::ops::Deref
for crate::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
impl std::ops::DerefMut
for crate::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
impl crate::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler {
    pub fn Invoke(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::EventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged")]
#[repr(C)]
#[derive(Debug)]
pub struct RSAManaged {
    __cordl_parent: crate::System::Security::Cryptography::RSA,
    pub isCRTpossible: bool,
    pub keyBlinding: bool,
    pub keypairGenerated: bool,
    pub m_disposed: bool,
    pub d: *mut crate::Mono::Math::BigInteger,
    pub p: *mut crate::Mono::Math::BigInteger,
    pub q: *mut crate::Mono::Math::BigInteger,
    pub dp: *mut crate::Mono::Math::BigInteger,
    pub dq: *mut crate::Mono::Math::BigInteger,
    pub qInv: *mut crate::Mono::Math::BigInteger,
    pub n: *mut crate::Mono::Math::BigInteger,
    pub e: *mut crate::Mono::Math::BigInteger,
    pub KeyGenerated: *mut crate::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler,
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::RSAManaged =>
    "Mono.Security.Cryptography"."RSAManaged"
);
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::RSAManaged {
    type Target = crate::System::Security::Cryptography::RSA;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::RSAManaged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged")]
impl crate::Mono::Security::Cryptography::RSAManaged {
    #[cfg(feature = "Mono+Security+Cryptography+RSAManaged+KeyGeneratedEventHandler")]
    pub type KeyGeneratedEventHandler = crate::Mono::Security::Cryptography::RSAManaged_KeyGeneratedEventHandler;
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptValue(
        &mut self,
        rgb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("EncryptValue", (rgb))?;
        Ok(__cordl_ret)
    }
    pub fn ExportParameters(
        &mut self,
        includePrivateParameters: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = __cordl_object
            .invoke("ExportParameters", (includePrivateParameters))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateKeyPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateKeyPair", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPaddedValue(
        &mut self,
        value: *mut crate::Mono::Math::BigInteger,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetPaddedValue", (value, length))?;
        Ok(__cordl_ret)
    }
    pub fn ImportParameters(
        &mut self,
        parameters: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportParameters", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(keySize: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keySize))?;
        Ok(__cordl_object)
    }
    pub fn ToXmlString(
        &mut self,
        includePrivateParameters: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToXmlString", (includePrivateParameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keySize))?;
        Ok(__cordl_ret)
    }
    pub fn get_KeySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_KeySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PublicOnly", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+RSAManaged")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::RSAManaged {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}