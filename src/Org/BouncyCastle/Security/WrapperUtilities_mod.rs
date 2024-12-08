#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct WrapperUtilities_BufferedCipherWrapper {
    __cordl_parent: crate::System::Object,
    pub cipher: *mut crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    pub forWrapping: bool,
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::WrapperUtilities_BufferedCipherWrapper =>
    "Org.BouncyCastle.Security"."WrapperUtilities/BufferedCipherWrapper"
);
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Security::WrapperUtilities_BufferedCipherWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Security::WrapperUtilities_BufferedCipherWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
impl crate::Org::BouncyCastle::Security::WrapperUtilities_BufferedCipherWrapper {
    pub fn Init(
        &mut self,
        forWrapping: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forWrapping, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher))?;
        Ok(__cordl_object)
    }
    pub fn Unwrap(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Unwrap", (input, inOff, length))?;
        Ok(__cordl_ret)
    }
    pub fn Wrap(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Wrap", (input, inOff, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher))?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::WrapperUtilities_BufferedCipherWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+WrapAlgorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapperUtilities_WrapAlgorithm {
    AESRFC3211WRAP = 6i32,
    AESWRAP = 0i32,
    CAMELLIARFC3211WRAP = 7i32,
    CAMELLIAWRAP = 1i32,
    DESEDERFC3211WRAP = 5i32,
    DESEDEWRAP = 2i32,
    RC2WRAP = 3i32,
    SEEDWRAP = 4i32,
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+WrapAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::WrapperUtilities_WrapAlgorithm =>
    "Org.BouncyCastle.Security"."WrapperUtilities/WrapAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct WrapperUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::WrapperUtilities =>
    "Org.BouncyCastle.Security"."WrapperUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::WrapperUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::WrapperUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities")]
impl crate::Org::BouncyCastle::Security::WrapperUtilities {
    #[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+BufferedCipherWrapper")]
    pub type BufferedCipherWrapper = crate::Org::BouncyCastle::Security::WrapperUtilities_BufferedCipherWrapper;
    #[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities+WrapAlgorithm")]
    pub type WrapAlgorithm = crate::Org::BouncyCastle::Security::WrapperUtilities_WrapAlgorithm;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+WrapperUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::WrapperUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
