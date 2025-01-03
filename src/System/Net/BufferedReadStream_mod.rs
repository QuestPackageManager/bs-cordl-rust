#[cfg(feature = "System+Net+BufferedReadStream")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedReadStream {
    __cordl_parent: crate::System::Net::WebReadStream,
    pub readBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
}
#[cfg(feature = "System+Net+BufferedReadStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::BufferedReadStream => "System.Net"
    ."BufferedReadStream"
);
#[cfg(feature = "System+Net+BufferedReadStream")]
impl std::ops::Deref for crate::System::Net::BufferedReadStream {
    type Target = crate::System::Net::WebReadStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+BufferedReadStream")]
impl std::ops::DerefMut for crate::System::Net::BufferedReadStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+BufferedReadStream")]
impl crate::System::Net::BufferedReadStream {
    pub fn New(
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        readBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operation, innerStream, readBuffer))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessReadAsync(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object
            .invoke(
                "ProcessReadAsync",
                (buffer, offset, _cordl_size, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryReadFromBuffer(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryReadFromBuffer", (buffer, offset, _cordl_size, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        readBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operation, innerStream, readBuffer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+BufferedReadStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::BufferedReadStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
