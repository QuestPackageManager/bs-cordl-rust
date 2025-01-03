#[cfg(feature = "System+Net+ContentDecodeStream")]
#[repr(C)]
#[derive(Debug)]
pub struct ContentDecodeStream {
    __cordl_parent: crate::System::Net::WebReadStream,
    pub _OriginalInnerStream_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::IO::Stream,
    >,
}
#[cfg(feature = "System+Net+ContentDecodeStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ContentDecodeStream => "System.Net"
    ."ContentDecodeStream"
);
#[cfg(feature = "System+Net+ContentDecodeStream")]
impl std::ops::Deref for crate::System::Net::ContentDecodeStream {
    type Target = crate::System::Net::WebReadStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ContentDecodeStream")]
impl std::ops::DerefMut for crate::System::Net::ContentDecodeStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ContentDecodeStream")]
impl crate::System::Net::ContentDecodeStream {
    #[cfg(feature = "System+Net+ContentDecodeStream+Mode")]
    pub type Mode = crate::System::Net::ContentDecodeStream_Mode;
    pub fn Create(
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        mode: crate::System::Net::ContentDecodeStream_Mode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ContentDecodeStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ContentDecodeStream,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (operation, innerStream, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishReading(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("FinishReading", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        decodeStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        originalInnerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operation, decodeStream, originalInnerStream))?;
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
    pub fn _ctor(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        decodeStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        originalInnerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operation, decodeStream, originalInnerStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OriginalInnerStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("get_OriginalInnerStream", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ContentDecodeStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ContentDecodeStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ContentDecodeStream+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentDecodeStream_Mode {
    Deflate = 1i32,
    GZip = 0i32,
}
#[cfg(feature = "System+Net+ContentDecodeStream+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ContentDecodeStream_Mode =>
    "System.Net"."ContentDecodeStream/Mode"
);
