#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultVerifierCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSignerSink: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::SignerSink,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator =>
    "Org.BouncyCastle.Crypto.Operators"."DefaultVerifierCalculator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
impl crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator {
    pub fn GetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        signer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Stream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("get_Stream", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IStreamCalculator>
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IStreamCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierCalculator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IStreamCalculator>
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierCalculator {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IStreamCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
