#[cfg(feature = "System+IO+Error")]
#[repr(C)]
#[derive(Debug)]
pub struct Error {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+Error")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::Error {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "Error";
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
#[cfg(feature = "System+IO+Error")]
impl std::ops::Deref for crate::System::IO::Error {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Error")]
impl std::ops::DerefMut for crate::System::IO::Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Error")]
impl crate::System::IO::Error {
    pub fn GetEndOfFile() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEndOfFile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReadNotSupported() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReadNotSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStreamIsClosed() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStreamIsClosed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWriteNotSupported() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWriteNotSupported", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Error")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::Error {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
