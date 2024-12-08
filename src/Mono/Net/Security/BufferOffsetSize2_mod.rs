#[cfg(feature = "Mono+Net+Security+BufferOffsetSize2")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferOffsetSize2 {
    __cordl_parent: crate::Mono::Net::Security::BufferOffsetSize,
    pub InitialSize: i32,
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::BufferOffsetSize2 =>
    "Mono.Net.Security"."BufferOffsetSize2"
);
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize2")]
impl std::ops::Deref for crate::Mono::Net::Security::BufferOffsetSize2 {
    type Target = crate::Mono::Net::Security::BufferOffsetSize;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize2")]
impl std::ops::DerefMut for crate::Mono::Net::Security::BufferOffsetSize2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize2")]
impl crate::Mono::Net::Security::BufferOffsetSize2 {
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn MakeRoom(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeRoom", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn AppendData(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendData", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn New(_cordl_size: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize2")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::BufferOffsetSize2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
