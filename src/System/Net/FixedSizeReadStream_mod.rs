#[cfg(feature = "System+Net+FixedSizeReadStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedSizeReadStream {
    __cordl_parent: crate::System::Net::WebReadStream,
    pub _ContentLength_k__BackingField: i64,
    pub position: i64,
}
#[cfg(feature = "System+Net+FixedSizeReadStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FixedSizeReadStream => "System.Net"
    ."FixedSizeReadStream"
);
#[cfg(feature = "System+Net+FixedSizeReadStream")]
impl std::ops::Deref for crate::System::Net::FixedSizeReadStream {
    type Target = crate::System::Net::WebReadStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FixedSizeReadStream")]
impl std::ops::DerefMut for crate::System::Net::FixedSizeReadStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FixedSizeReadStream")]
impl crate::System::Net::FixedSizeReadStream {
    #[cfg(feature = "System+Net+FixedSizeReadStream+_ProcessReadAsync_d__5")]
    pub type _ProcessReadAsync_d__5 = crate::System::Net::FixedSizeReadStream__ProcessReadAsync_d__5;
    pub fn _ctor(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        innerStream: *mut crate::System::IO::Stream,
        contentLength: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operation, innerStream, contentLength))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessReadAsync(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<i32> = __cordl_object
            .invoke(
                "ProcessReadAsync",
                (buffer, offset, _cordl_size, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContentLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        operation: *mut crate::System::Net::WebOperation,
        innerStream: *mut crate::System::IO::Stream,
        contentLength: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operation, innerStream, contentLength))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+FixedSizeReadStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FixedSizeReadStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
