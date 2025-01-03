#[cfg(feature = "System+IO+PathInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct PathInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+PathInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::PathInternal => "System.IO"
    ."PathInternal"
);
#[cfg(feature = "System+IO+PathInternal")]
impl std::ops::Deref for crate::System::IO::PathInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+PathInternal")]
impl std::ops::DerefMut for crate::System::IO::PathInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+PathInternal")]
impl crate::System::IO::PathInternal {
    pub fn EndsInDirectorySeparator(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsInDirectorySeparator", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIsCaseSensitive() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsCaseSensitive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootLength(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRootLength", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDirectorySeparator(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDirectorySeparator", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartiallyQualified(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPartiallyQualified", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRoot(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRoot", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartsWithDirectorySeparator(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartsWithDirectorySeparator", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrimEndingDirectorySeparator_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrimEndingDirectorySeparator", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrimEndingDirectorySeparator_ReadOnlySpan_1_1(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrimEndingDirectorySeparator", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCaseSensitive() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsCaseSensitive", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+PathInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::PathInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
