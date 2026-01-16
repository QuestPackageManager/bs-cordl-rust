#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManager")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncReadManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.IO.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "AsyncReadManager";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManager")]
impl std::ops::Deref for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManager")]
impl std::ops::DerefMut for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManager")]
impl crate::Unity::IO::LowLevel::Unsafe::AsyncReadManager {
    pub fn CloseFileAsync(
        fileHandle: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::FileHandle>,
        dependency: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        >,
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 2usize>("CloseFileAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CloseFileAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (fileHandle, dependency))? };
        Ok(__cordl_ret.into())
    }
    pub fn CloseFileAsync_Injected(
        fileHandle: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::FileHandle>,
        dependency: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "CloseFileAsync_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CloseFileAsync_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fileHandle, dependency, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFileInfo(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IO::LowLevel::Unsafe::ReadHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::Unity::IO::LowLevel::Unsafe::ReadHandle, 2usize>(
                        "GetFileInfo"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFileInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IO::LowLevel::Unsafe::ReadHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (filename, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFileInfoInternal(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cmd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IO::LowLevel::Unsafe::ReadHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::Unity::IO::LowLevel::Unsafe::ReadHandle, 2usize>(
                        "GetFileInfoInternal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFileInfoInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IO::LowLevel::Unsafe::ReadHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (filename, cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFileInfoInternal_Injected(
        filename: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        cmd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::ReadHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::ReadHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetFileInfoInternal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFileInfoInternal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (filename, cmd, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpenFileAsync(
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IO::LowLevel::Unsafe::FileHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        1usize,
                    >("OpenFileAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpenFileAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IO::LowLevel::Unsafe::FileHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (fileName))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpenFileAsync_Internal(
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IO::LowLevel::Unsafe::FileHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        1usize,
                    >("OpenFileAsync_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpenFileAsync_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IO::LowLevel::Unsafe::FileHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (fileName))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpenFileAsync_Internal_Injected(
        fileName: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::FileHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "OpenFileAsync_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpenFileAsync_Internal_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fileName, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn Read(
        fileHandle: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::FileHandle>,
        readCmdArray: crate::Unity::IO::LowLevel::Unsafe::ReadCommandArray,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IO::LowLevel::Unsafe::ReadHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        >,
                        crate::Unity::IO::LowLevel::Unsafe::ReadCommandArray,
                    ), crate::Unity::IO::LowLevel::Unsafe::ReadHandle, 2usize>(
                        "Read"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Read",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IO::LowLevel::Unsafe::ReadHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (fileHandle, readCmdArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadWithHandlesInternal_NativeCopy(
        fileHandle: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::FileHandle>,
        readCmdArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IO::LowLevel::Unsafe::ReadHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::Unity::IO::LowLevel::Unsafe::ReadHandle, 2usize>(
                        "ReadWithHandlesInternal_NativeCopy",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadWithHandlesInternal_NativeCopy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IO::LowLevel::Unsafe::ReadHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (fileHandle, readCmdArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadWithHandlesInternal_NativeCopy_Injected(
        fileHandle: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::FileHandle>,
        readCmdArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::IO::LowLevel::Unsafe::ReadHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::FileHandle,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::IO::LowLevel::Unsafe::ReadHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ReadWithHandlesInternal_NativeCopy_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadWithHandlesInternal_NativeCopy_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fileHandle, readCmdArray, ret))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
