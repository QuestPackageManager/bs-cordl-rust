#[cfg(feature = "System+Net+Http+PlatformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::PlatformHelper =>
    "System.Net.Http"."PlatformHelper"
);
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl std::ops::Deref for crate::System::Net::Http::PlatformHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl std::ops::DerefMut for crate::System::Net::Http::PlatformHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl crate::System::Net::Http::PlatformHelper {
    pub fn CreateStreamContent(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::StreamContent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::StreamContent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStreamContent", (stream, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSingleHeaderString(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSingleHeaderString", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsContentHeader(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsContentHeader", (name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::PlatformHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
