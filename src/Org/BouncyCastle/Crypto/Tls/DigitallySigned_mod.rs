#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DigitallySigned")]
#[repr(C)]
#[derive(Debug)]
pub struct DigitallySigned {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mAlgorithm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
    >,
    pub mSignature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DigitallySigned")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::DigitallySigned
    => "Org.BouncyCastle.Crypto.Tls"."DigitallySigned"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DigitallySigned")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DigitallySigned")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DigitallySigned")]
impl crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned {
    pub fn Encode(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, signature))?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (context, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        > = __cordl_object.invoke("get_Algorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Signature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Signature", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DigitallySigned")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
