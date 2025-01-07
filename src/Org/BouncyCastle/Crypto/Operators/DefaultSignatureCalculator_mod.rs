#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultSignatureCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSignerSink: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::SignerSink,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "DefaultSignatureCalculator";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
impl crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IStreamCalculator>
for crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IStreamCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultSignatureCalculator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IStreamCalculator>
for crate::Org::BouncyCastle::Crypto::Operators::DefaultSignatureCalculator {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IStreamCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
