#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectDispatcher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
    pub m_DispatchAllocator: crate::Unity::Collections::Allocator,
    pub m_TypeDispatchData: crate::UnityEngine::TypeDispatchData,
    pub m_TransformDispatchData: crate::UnityEngine::TransformDispatchData,
    pub m_TransformedComponents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        >,
    >,
    pub m_TypeDataCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::UnityEngine::TypeDispatchData>,
    >,
    pub m_TransformDataCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::UnityEngine::TransformDispatchData>,
    >,
    pub m_TransformComponentCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ObjectDispatcher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ObjectDispatcher";
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
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl std::ops::Deref for crate::UnityEngine::ObjectDispatcher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl std::ops::DerefMut for crate::UnityEngine::ObjectDispatcher {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl crate::UnityEngine::ObjectDispatcher {
    #[cfg(feature = "UnityEngine+ObjectDispatcher+TransformTrackingType")]
    pub type TransformTrackingType = crate::UnityEngine::ObjectDispatcher_TransformTrackingType;
    #[cfg(feature = "UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
    pub type TypeTrackingFlags = crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags;
    pub fn CreateDispatchSystemHandle() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::IntPtr,
                        0usize,
                    >("CreateDispatchSystemHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDispatchSystemHandle", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyDispatchSystemHandle(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DestroyDispatchSystemHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyDispatchSystemHandle", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (ptr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCallback_Il2CppArray2(
        &mut self,
        components: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DispatchCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (components))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCallback_TransformDispatchData1(
        &mut self,
        data: crate::UnityEngine::TransformDispatchData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::TransformDispatchData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DispatchCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCallback_TypeDispatchData0(
        &mut self,
        data: crate::UnityEngine::TypeDispatchData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::TypeDispatchData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DispatchCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchTransformChangesAndClear(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::TransformDispatchData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::TransformDispatchData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DispatchTransformChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchTransformChangesAndClear", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (_cordl_type, trackingType, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchTransformDataChangesAndClear(
        ptr: crate::System::IntPtr,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_8<
                crate::System::IntPtr,
                crate::System::IntPtr,
                crate::System::IntPtr,
                crate::System::IntPtr,
                crate::System::IntPtr,
                crate::System::IntPtr,
                i32,
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::UnityEngine::TransformDispatchData>,
                >,
            >,
        >,
        param: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::TransformDispatchData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_8<
                                    crate::System::IntPtr,
                                    crate::System::IntPtr,
                                    crate::System::IntPtr,
                                    crate::System::IntPtr,
                                    crate::System::IntPtr,
                                    crate::System::IntPtr,
                                    i32,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Action_1<
                                            crate::UnityEngine::TransformDispatchData,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::TransformDispatchData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DispatchTransformDataChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchTransformDataChangesAndClear", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (ptr, _cordl_type, trackingType, callback, param))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchTypeChangesAndClear_IntPtr_Type_Action_6__cordl_bool_Action_1_1(
        ptr: crate::System::IntPtr,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_6<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >,
                crate::System::IntPtr,
                crate::System::IntPtr,
                i32,
                i32,
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::UnityEngine::TypeDispatchData>,
                >,
            >,
        >,
        sortByInstanceID: bool,
        noScriptingArray: bool,
        param: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::TypeDispatchData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_6<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppArray<
                                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                                        >,
                                    >,
                                    crate::System::IntPtr,
                                    crate::System::IntPtr,
                                    i32,
                                    i32,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Action_1<
                                            crate::UnityEngine::TypeDispatchData,
                                        >,
                                    >,
                                >,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::TypeDispatchData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DispatchTypeChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchTypeChangesAndClear", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        ptr,
                        _cordl_type,
                        callback,
                        sortByInstanceID,
                        noScriptingArray,
                        param,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchTypeChangesAndClear_Type_Action_1__cordl_bool0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::TypeDispatchData>,
        >,
        sortByInstanceID: bool,
        noScriptingArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::TypeDispatchData,
                                >,
                            >,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DispatchTypeChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchTypeChangesAndClear", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_type, callback, sortByInstanceID, noScriptingArray),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (disposing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableTransformTracking_IntPtr_Type_ObjectDispatcher_TransformTrackingType2(
        ptr: crate::System::IntPtr,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("EnableTransformTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableTransformTracking", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (ptr, _cordl_type, trackingType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableTransformTracking_ObjectDispatcher_TransformTrackingType1<T>(
        &mut self,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::ObjectDispatcher_TransformTrackingType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableTransformTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableTransformTracking", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (trackingType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableTransformTracking_ObjectDispatcher_TransformTrackingType_Il2CppArray0(
        &mut self,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("EnableTransformTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableTransformTracking", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (trackingType, types))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableTypeTracking_IntPtr_Type_ObjectDispatcher_TypeTrackingFlags2(
        ptr: crate::System::IntPtr,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeTrackingMask: crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("EnableTypeTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableTypeTracking", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (ptr, _cordl_type, typeTrackingMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableTypeTracking_ObjectDispatcher_TypeTrackingFlags1<T>(
        &mut self,
        typeTrackingMask: crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableTypeTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableTypeTracking", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (typeTrackingMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableTypeTracking_ObjectDispatcher_TypeTrackingFlags_Il2CppArray0(
        &mut self,
        typeTrackingMask: crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("EnableTypeTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableTypeTracking", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (typeTrackingMask, types))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Finalize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformChangesAndClear_ObjectDispatcher_TransformTrackingType_Allocator1<
        T,
    >(
        &mut self,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TransformDispatchData>
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
                            crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
                            crate::Unity::Collections::Allocator,
                        ),
                        crate::UnityEngine::TransformDispatchData,
                        2usize,
                    >("GetTransformChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTransformChangesAndClear", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TransformDispatchData = unsafe {
            cordl_method_info.invoke_unchecked(self, (trackingType, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformChangesAndClear_Type_ObjectDispatcher_TransformTrackingType_Allocator0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        trackingType: crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TransformDispatchData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::UnityEngine::ObjectDispatcher_TransformTrackingType,
                            crate::Unity::Collections::Allocator,
                        ),
                        crate::UnityEngine::TransformDispatchData,
                        3usize,
                    >("GetTransformChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTransformChangesAndClear", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TransformDispatchData = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (_cordl_type, trackingType, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeChangesAndClear_Allocator__cordl_bool1<T>(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
        sortByInstanceID: bool,
        noScriptingArray: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TypeDispatchData>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::Allocator, bool, bool),
                        crate::UnityEngine::TypeDispatchData,
                        3usize,
                    >("GetTypeChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypeChangesAndClear", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TypeDispatchData = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (allocator, sortByInstanceID, noScriptingArray))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeChangesAndClear_Type_Allocator__cordl_bool0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        allocator: crate::Unity::Collections::Allocator,
        sortByInstanceID: bool,
        noScriptingArray: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TypeDispatchData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::Unity::Collections::Allocator,
                            bool,
                            bool,
                        ),
                        crate::UnityEngine::TypeDispatchData,
                        4usize,
                    >("GetTypeChangesAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypeChangesAndClear", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TypeDispatchData = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_type, allocator, sortByInstanceID, noScriptingArray),
                )?
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
    pub fn ValidateComponentTypeAndThrow(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateComponentTypeAndThrow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateComponentTypeAndThrow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateSystemHandleAndThrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ValidateSystemHandleAndThrow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateSystemHandleAndThrow", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateTypeAndThrow(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateTypeAndThrow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateTypeAndThrow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
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
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_valid", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ObjectDispatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::ObjectDispatcher {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::ObjectDispatcher {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TransformTrackingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectDispatcher_TransformTrackingType {
    #[default]
    GlobalTRS = 0i32,
    Hierarchy = 2i32,
    LocalTRS = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TransformTrackingType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ObjectDispatcher_TransformTrackingType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ObjectDispatcher/TransformTrackingType";
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TransformTrackingType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ObjectDispatcher_TransformTrackingType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TransformTrackingType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ObjectDispatcher_TransformTrackingType {
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TransformTrackingType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ObjectDispatcher_TransformTrackingType {
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TransformTrackingType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ObjectDispatcher_TransformTrackingType {
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectDispatcher_TypeTrackingFlags {
    #[default]
    All = 7i32,
    Assets = 2i32,
    Default = 3i32,
    EditorOnlyObjects = 4i32,
    SceneObjects = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ObjectDispatcher/TypeTrackingFlags";
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags {
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags {
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
#[cfg(feature = "cordl_class_UnityEngine+ObjectDispatcher+TypeTrackingFlags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ObjectDispatcher_TypeTrackingFlags {
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
