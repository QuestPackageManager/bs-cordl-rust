#[cfg(feature = "System+IO+PathInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct PathInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+PathInternal")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::PathInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "PathInternal";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                bool,
                1usize,
            >("EndsInDirectorySeparator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndsInDirectorySeparator", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetIsCaseSensitive() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("GetIsCaseSensitive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetIsCaseSensitive", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRootLength(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                i32,
                1usize,
            >("GetRootLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRootLength", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsDirectorySeparator(c: char) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), bool, 1usize>("IsDirectorySeparator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsDirectorySeparator", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsPartiallyQualified(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsPartiallyQualified")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsPartiallyQualified", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsRoot(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                bool,
                1usize,
            >("IsRoot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsRoot", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn StartsWithDirectorySeparator(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                bool,
                1usize,
            >("StartsWithDirectorySeparator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartsWithDirectorySeparator", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEndingDirectorySeparator_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("TrimEndingDirectorySeparator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrimEndingDirectorySeparator", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEndingDirectorySeparator_ReadOnlySpan_1_1(
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                crate::System::ReadOnlySpan_1<char>,
                1usize,
            >("TrimEndingDirectorySeparator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrimEndingDirectorySeparator", 1usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (path))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCaseSensitive() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsCaseSensitive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsCaseSensitive", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
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
