#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519ctxSigner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub buffer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer,
    >,
    pub context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub forSigning: bool,
    pub privateKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::Ed25519PrivateKeyParameters,
    >,
    pub publicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::Ed25519PublicKeyParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner =>
    "Org.BouncyCastle.Crypto.Signers"."Ed25519ctxSigner"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
impl crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
    pub type Buffer = crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer;
    pub fn BlockUpdate(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlockUpdate", (buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateSignature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        forSigning: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forSigning, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature(
        &mut self,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifySignature", (signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ISigner>
for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ISigner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ISigner>
for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ISigner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519ctxSigner_Buffer {
    __cordl_parent: crate::System::IO::MemoryStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer =>
    "Org.BouncyCastle.Crypto.Signers"."Ed25519ctxSigner/Buffer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer {
    type Target = crate::System::IO::MemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
impl crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer {
    pub fn GenerateSignature(
        &mut self,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Ed25519PrivateKeyParameters,
        >,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateSignature", (privateKey, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature(
        &mut self,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Ed25519PublicKeyParameters,
        >,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifySignature", (publicKey, ctx, signature))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Ed25519ctxSigner+Buffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::Ed25519ctxSigner_Buffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
