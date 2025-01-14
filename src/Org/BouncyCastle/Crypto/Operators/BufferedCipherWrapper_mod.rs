#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedCipherWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub bufferedCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    >,
    pub stream: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::CipherStream,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "BufferedCipherWrapper";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    pub fn GetMaxOutputSize(
        &mut self,
        inputLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetMaxOutputSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMaxOutputSize", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (inputLen)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetUpdateOutputSize(
        &mut self,
        inputLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetUpdateOutputSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUpdateOutputSize", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (inputLen)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bufferedCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        >,
        source: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bufferedCipher, source))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bufferedCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        >,
        source: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::IBufferedCipher,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bufferedCipher, source))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Stream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                0usize,
            >("get_Stream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Stream", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipher>
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipher>
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipher {
        unsafe { std::mem::transmute(self) }
    }
}
