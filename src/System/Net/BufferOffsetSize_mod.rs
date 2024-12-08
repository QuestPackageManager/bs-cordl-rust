#[cfg(feature = "System+Net+BufferOffsetSize")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferOffsetSize {
    __cordl_parent: crate::System::Object,
    pub Buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Offset: i32,
    pub Size: i32,
}
#[cfg(feature = "System+Net+BufferOffsetSize")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::BufferOffsetSize => "System.Net"
    ."BufferOffsetSize"
);
#[cfg(feature = "System+Net+BufferOffsetSize")]
impl std::ops::Deref for crate::System::Net::BufferOffsetSize {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+BufferOffsetSize")]
impl std::ops::DerefMut for crate::System::Net::BufferOffsetSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+BufferOffsetSize")]
impl crate::System::Net::BufferOffsetSize {
    pub fn _ctor_i32_i32__cordl_bool0(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        copyBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (buffer, offset, _cordl_size, copyBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        copyBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (buffer, copyBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_i32__cordl_bool0(
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        copyBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (buffer, offset, _cordl_size, copyBuffer))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        copyBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (buffer, copyBuffer))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+BufferOffsetSize")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::BufferOffsetSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
