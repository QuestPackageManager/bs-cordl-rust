#[cfg(feature = "System+Net+Http+StringContent")]
#[repr(C)]
#[derive(Debug)]
pub struct StringContent {
    __cordl_parent: crate::System::Net::Http::ByteArrayContent,
}
#[cfg(feature = "System+Net+Http+StringContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::StringContent =>
    "System.Net.Http"."StringContent"
);
#[cfg(feature = "System+Net+Http+StringContent")]
impl std::ops::Deref for crate::System::Net::Http::StringContent {
    type Target = crate::System::Net::Http::ByteArrayContent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+StringContent")]
impl std::ops::DerefMut for crate::System::Net::Http::StringContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+StringContent")]
impl crate::System::Net::Http::StringContent {
    pub fn New(
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        mediaType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content, encoding, mediaType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        mediaType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content, encoding, mediaType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+StringContent")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::StringContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
