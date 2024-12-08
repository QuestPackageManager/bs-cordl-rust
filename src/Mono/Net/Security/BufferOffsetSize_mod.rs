#[cfg(feature = "Mono+Net+Security+BufferOffsetSize")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferOffsetSize {
    __cordl_parent: crate::System::Object,
    pub Buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Offset: i32,
    pub Size: i32,
    pub TotalBytes: i32,
    pub Complete: bool,
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::BufferOffsetSize =>
    "Mono.Net.Security"."BufferOffsetSize"
);
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize")]
impl std::ops::Deref for crate::Mono::Net::Security::BufferOffsetSize {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize")]
impl std::ops::DerefMut for crate::Mono::Net::Security::BufferOffsetSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize")]
impl crate::Mono::Net::Security::BufferOffsetSize {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn get_EndOffset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_EndOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Remaining(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Remaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (buffer, offset, _cordl_size))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Net+Security+BufferOffsetSize")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Net::Security::BufferOffsetSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
