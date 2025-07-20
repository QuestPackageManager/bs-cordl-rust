#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryMarshal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::MemoryMarshal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "MemoryMarshal";
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
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::MemoryMarshal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::MemoryMarshal {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl crate::System::Runtime::InteropServices::MemoryMarshal {
    pub fn AsBytes_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<T>),
                        crate::System::ReadOnlySpan_1<u8>,
                        1usize,
                    >("AsBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AsBytes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), (span))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsBytes_Span_1_0<T>(
        span: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Span_1<T>),
                        crate::System::Span_1<u8>,
                        1usize,
                    >("AsBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AsBytes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Span_1<u8> = unsafe {
            method.invoke_unchecked((), (span))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsMemory<T>(
        memory: crate::System::ReadOnlyMemory_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Memory_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlyMemory_1<T>),
                        crate::System::Memory_1<T>,
                        1usize,
                    >("AsMemory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsMemory", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Memory_1<T> = unsafe {
            method.invoke_unchecked((), (memory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateReadOnlySpan<T>(
        reference: quest_hook::libil2cpp::ByRefMut<T>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, i32),
                        crate::System::ReadOnlySpan_1<T>,
                        2usize,
                    >("CreateReadOnlySpan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateReadOnlySpan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<T> = unsafe {
            method.invoke_unchecked((), (reference, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNonNullPinnableReference_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<T>),
                        quest_hook::libil2cpp::ByRefMut<T>,
                        1usize,
                    >("GetNonNullPinnableReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNonNullPinnableReference", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (span))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNonNullPinnableReference_Span_1_0<T>(
        span: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Span_1<T>),
                        quest_hook::libil2cpp::ByRefMut<T>,
                        1usize,
                    >("GetNonNullPinnableReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNonNullPinnableReference", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (span))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetReference_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<T>),
                        quest_hook::libil2cpp::ByRefMut<T>,
                        1usize,
                    >("GetReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetReference", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (span))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetReference_Span_1_0<T>(
        span: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Span_1<T>),
                        quest_hook::libil2cpp::ByRefMut<T>,
                        1usize,
                    >("GetReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetReference", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (span))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetArray<T>(
        memory: crate::System::ReadOnlyMemory_1<T>,
        segment: quest_hook::libil2cpp::ByRefMut<crate::System::ArraySegment_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlyMemory_1<T>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::ArraySegment_1<T>,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryGetArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetArray", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (memory, segment))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::MemoryMarshal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
