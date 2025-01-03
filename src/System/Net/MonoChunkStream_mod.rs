#[cfg(feature = "System+Net+MonoChunkStream")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoChunkStream {
    __cordl_parent: crate::System::Net::WebReadStream,
    pub _Headers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::WebHeaderCollection,
    >,
    pub _Decoder_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::MonoChunkParser,
    >,
}
#[cfg(feature = "System+Net+MonoChunkStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::MonoChunkStream => "System.Net"
    ."MonoChunkStream"
);
#[cfg(feature = "System+Net+MonoChunkStream")]
impl std::ops::Deref for crate::System::Net::MonoChunkStream {
    type Target = crate::System::Net::WebReadStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+MonoChunkStream")]
impl std::ops::DerefMut for crate::System::Net::MonoChunkStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+MonoChunkStream")]
impl crate::System::Net::MonoChunkStream {
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
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        headers: quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operation, innerStream, headers))?;
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
    pub fn ThrowExpectingChunkTrailer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowExpectingChunkTrailer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __n__0(
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
        > = __cordl_object.invoke("<>n__0", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        headers: quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operation, innerStream, headers))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Decoder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::MonoChunkParser>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::MonoChunkParser,
        > = __cordl_object.invoke("get_Decoder", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+MonoChunkStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::MonoChunkStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
