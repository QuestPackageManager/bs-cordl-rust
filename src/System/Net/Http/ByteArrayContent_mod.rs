#[cfg(feature = "System+Net+Http+ByteArrayContent")]
#[repr(C)]
#[derive(Debug)]
pub struct ByteArrayContent {
    __cordl_parent: crate::System::Net::Http::HttpContent,
    pub content: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub offset: i32,
    pub count: i32,
}
#[cfg(feature = "System+Net+Http+ByteArrayContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::ByteArrayContent =>
    "System.Net.Http"."ByteArrayContent"
);
#[cfg(feature = "System+Net+Http+ByteArrayContent")]
impl std::ops::Deref for crate::System::Net::Http::ByteArrayContent {
    type Target = crate::System::Net::Http::HttpContent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+ByteArrayContent")]
impl std::ops::DerefMut for crate::System::Net::Http::ByteArrayContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+ByteArrayContent")]
impl crate::System::Net::Http::ByteArrayContent {
    pub fn New(
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content))?;
        Ok(__cordl_object.into())
    }
    pub fn SerializeToStreamAsync(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        context: quest_hook::libil2cpp::Gc<crate::System::Net::TransportContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SerializeToStreamAsync", (stream, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryComputeLength(
        &mut self,
        length: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryComputeLength", (length))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+ByteArrayContent")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::ByteArrayContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
