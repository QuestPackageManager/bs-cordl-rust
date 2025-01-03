#[cfg(feature = "SecureRandomProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureRandomProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SecureRandomProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SecureRandomProvider => ""
    ."SecureRandomProvider"
);
#[cfg(feature = "SecureRandomProvider")]
impl std::ops::Deref for crate::GlobalNamespace::SecureRandomProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SecureRandomProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::SecureRandomProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SecureRandomProvider")]
impl crate::GlobalNamespace::SecureRandomProvider {
    #[cfg(feature = "SecureRandomProvider+SecureRandomState")]
    pub type SecureRandomState = crate::GlobalNamespace::SecureRandomProvider_SecureRandomState;
    pub fn GetByte() -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_Il2CppArray2(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_Il2CppArray_i32_i32_1(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (buffer, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_i32_0(
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetBytes", (length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SecureRandomProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SecureRandomProvider {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _random: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::RNGCryptoServiceProvider,
    >,
    pub _randomBuffer0: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _randomBuffer1: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn GetByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBytes", (buffer, offset, length))?;
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
