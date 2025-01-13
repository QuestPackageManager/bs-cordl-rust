#[cfg(feature = "ENet+Library")]
#[repr(C)]
#[derive(Debug)]
pub struct Library {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ENet+Library")]
unsafe impl quest_hook::libil2cpp::Type for crate::ENet::Library {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "ENet";
    const CLASS_NAME: &'static str = "Library";
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
#[cfg(feature = "ENet+Library")]
impl std::ops::Deref for crate::ENet::Library {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Library")]
impl std::ops::DerefMut for crate::ENet::Library {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Library")]
impl crate::ENet::Library {
    pub const maxChannelCount: u32 = 255u32;
    pub const maxPacketSize: u32 = 33554432u32;
    pub const maxPeers: u32 = 4095u32;
    pub const throttleAcceleration: u32 = 2u32;
    pub const throttleDeceleration: u32 = 2u32;
    pub const throttleInterval: u32 = 5000u32;
    pub const throttleScale: u32 = 32u32;
    pub const throttleThreshold: u32 = 40u32;
    pub const timeoutLimit: u32 = 32u32;
    pub const timeoutMaximum: u32 = 30000u32;
    pub const timeoutMinimum: u32 = 5000u32;
    pub const version: u32 = 132103u32;
    pub fn CRC64(
        buffers: crate::System::IntPtr,
        bufferCount: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CRC64", (buffers, bufferCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deinitialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deinitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitOpenSSL() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitOpenSSL", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_0() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_Callbacks1(
        callbacks: quest_hook::libil2cpp::Gc<crate::ENet::Callbacks>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", (callbacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Time() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Time", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ENet+Library")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::Library {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
