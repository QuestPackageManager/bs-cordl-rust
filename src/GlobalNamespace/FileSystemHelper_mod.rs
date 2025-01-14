#[cfg(feature = "FileSystemHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "FileSystemHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::FileSystemHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FileSystemHelper";
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
#[cfg(feature = "FileSystemHelper")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileSystemHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemHelper")]
impl crate::GlobalNamespace::FileSystemHelper {
    pub fn FindFirstExistedParentPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("FindFirstExistedParentPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindFirstExistedParentPath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn HasWritePermission(
        accessControlList: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::FileSystemSecurity,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::AccessControl::FileSystemSecurity,
                >),
                bool,
                1usize,
            >("HasWritePermission")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasWritePermission", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (accessControlList))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasWritePermissionOnDirectory(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("HasWritePermissionOnDirectory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasWritePermissionOnDirectory", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn HasWritePermissionOnFile(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("HasWritePermissionOnFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasWritePermissionOnFile", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsFileWritable(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsFileWritable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsFileWritable", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (path)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FileSystemHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
