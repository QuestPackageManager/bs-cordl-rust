#[cfg(feature = "System+Net+Http+PlatformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Http::PlatformHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Http";
    const CLASS_NAME: &'static str = "PlatformHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl std::ops::Deref for crate::System::Net::Http::PlatformHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
