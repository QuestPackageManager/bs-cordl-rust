#[cfg(feature = "cordl_class_RenderGraphCompilationCache")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphCompilationCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_HashEntries: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::DynamicArray_1<
            crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
                >,
            >,
        >,
    >,
    pub m_NativeHashEntries: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::DynamicArray_1<
            crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<Blacklisted>,
        >,
    >,
    pub m_CompiledGraphPool: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
            >,
        >,
    >,
    pub m_NativeCompiledGraphPool: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<Blacklisted>,
    >,
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RenderGraphCompilationCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RenderGraphCompilationCache";
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
#[cfg(feature = "RenderGraphCompilationCache")]
impl std::ops::Deref for crate::GlobalNamespace::RenderGraphCompilationCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RenderGraphCompilationCache")]
impl std::ops::DerefMut for crate::GlobalNamespace::RenderGraphCompilationCache {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RenderGraphCompilationCache")]
impl crate::GlobalNamespace::RenderGraphCompilationCache {
    pub const k_CachedGraphCount: i32 = 20i32;
    #[cfg(feature = "RenderGraphCompilationCache+HashEntry_1")]
    pub type HashEntry_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<
        T,
    >;
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompilationCache_DynamicArray_1_Stack_1_DynamicArray_1_SortComparer0<T>(
        &mut self,
        hash: i32,
        frameIndex: i32,
        outGraph: quest_hook::libil2cpp::ByRefMut<T>,
        hashEntries: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1<
                crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T>,
            >,
        >,
        pool: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::Stack_1<T>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<
                crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DynamicArray_1<
                                    crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<
                                        T,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Stack_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<
                                    crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<
                                        T,
                                    >,
                                >,
                            >,
                        ),
                        bool,
                        6usize,
                    >("GetCompilationCache")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCompilationCache", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (hash, frameIndex, outGraph, hashEntries, pool, comparer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompilationCache_i32_i32_ByRefMut1(
        &mut self,
        hash: i32,
        frameIndex: i32,
        outGraph: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("GetCompilationCache")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCompilationCache", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (hash, frameIndex, outGraph))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompilationCache_i32_i32_ByRefMut2(
        &mut self,
        hash: i32,
        frameIndex: i32,
        outGraph: quest_hook::libil2cpp::ByRefMut<Blacklisted>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, quest_hook::libil2cpp::ByRefMut<Blacklisted>),
                        bool,
                        3usize,
                    >("GetCompilationCache")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCompilationCache", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (hash, frameIndex, outGraph))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HashEntryComparer<T>(
        a: crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T>,
        b: crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<
                                T,
                            >,
                            crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<
                                T,
                            >,
                        ),
                        i32,
                        2usize,
                    >("HashEntryComparer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HashEntryComparer", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RenderGraphCompilationCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderGraphCompilationCache_HashEntry_1<T: quest_hook::libil2cpp::Type> {
    pub hash: i32,
    pub lastFrameUsed: i32,
    pub compiledGraph: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RenderGraphCompilationCache/HashEntry`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "",
                        "RenderGraphCompilationCache/HashEntry`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_RenderGraphCompilationCache+HashEntry_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "RenderGraphCompilationCache+HashEntry_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::RenderGraphCompilationCache_HashEntry_1<T> {}
