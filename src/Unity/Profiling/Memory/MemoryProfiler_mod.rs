#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryProfiler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::Memory::MemoryProfiler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Profiling.Memory";
    const CLASS_NAME: &'static str = "MemoryProfiler";
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
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl std::ops::Deref for crate::Unity::Profiling::Memory::MemoryProfiler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl std::ops::DerefMut for crate::Unity::Profiling::Memory::MemoryProfiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl crate::Unity::Profiling::Memory::MemoryProfiler {
    pub fn FinalizeSnapshot(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("FinalizeSnapshot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FinalizeSnapshot", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (path, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareMetadata() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("PrepareMetadata")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PrepareMetadata", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn SaveScreenshotToDisk(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: bool,
        pixelsPtr: crate::System::IntPtr,
        pixelsCount: i32,
        format: crate::UnityEngine::TextureFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    crate::System::IntPtr,
                    i32,
                    crate::UnityEngine::TextureFormat,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("SaveScreenshotToDisk")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveScreenshotToDisk", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (path, result, pixelsPtr, pixelsCount, format, width, height),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteIntToByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                i32,
                3usize,
            >("WriteIntToByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteIntToByteArray", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (array, offset, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteStringToByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                3usize,
            >("WriteStringToByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteStringToByteArray", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (array, offset, value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Profiling::Memory::MemoryProfiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
