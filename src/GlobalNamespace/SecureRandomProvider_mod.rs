#[cfg(feature = "SecureRandomProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureRandomProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SecureRandomProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SecureRandomProvider => ""."SecureRandomProvider"
);
#[cfg(feature = "SecureRandomProvider")]
impl std::ops::Deref for SecureRandomProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SecureRandomProvider")]
impl std::ops::DerefMut for SecureRandomProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SecureRandomProvider")]
impl SecureRandomProvider {
    #[cfg(feature = "SecureRandomProvider+SecureRandomState")]
    pub type SecureRandomState = crate::GlobalNamespace::SecureRandomProvider_SecureRandomState;
}
#[cfg(feature = "SecureRandomProvider")]
impl quest_hook::libil2cpp::ObjectType for SecureRandomProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SecureRandomProvider+SecureRandomState")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureRandomProvider_SecureRandomState {
    __cordl_parent: crate::System::Object,
    pub _random: *mut crate::System::Security::Cryptography::RNGCryptoServiceProvider,
    pub _randomBuffer0: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _randomBuffer1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _index: i32,
}
#[cfg(feature = "SecureRandomProvider+SecureRandomState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SecureRandomProvider_SecureRandomState => ""
    ."SecureRandomProvider/SecureRandomState"
);
#[cfg(feature = "SecureRandomProvider+SecureRandomState")]
impl std::ops::Deref for crate::GlobalNamespace::SecureRandomProvider_SecureRandomState {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SecureRandomProvider+SecureRandomState")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SecureRandomProvider_SecureRandomState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SecureRandomProvider+SecureRandomState")]
impl crate::GlobalNamespace::SecureRandomProvider_SecureRandomState {
    pub const kBufferSize: i32 = 16384i32;
    pub fn FillBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBytes", (buffer, offset, length))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "SecureRandomProvider+SecureRandomState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SecureRandomProvider_SecureRandomState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
