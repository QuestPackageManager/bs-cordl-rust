#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutDataAccess {
    pub m_Manager: i32,
    pub m_Nodes: crate::UnityEngine::UIElements::Layout::LayoutDataStore,
    pub m_Configs: crate::UnityEngine::UIElements::Layout::LayoutDataStore,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutDataAccess";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDataAccess")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutDataAccess")]
impl crate::UnityEngine::UIElements::Layout::LayoutDataAccess {
    pub fn GetBaselineFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction,
                        >,
                        1usize,
                    >("GetBaselineFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBaselineFunction", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComputedData(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutComputedData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutComputedData,
                        >,
                        1usize,
                    >("GetComputedData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetComputedData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutComputedData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConfigData(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutConfigData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutConfigData,
                        >,
                        1usize,
                    >("GetConfigData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetConfigData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutConfigData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMeasureFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
                        >,
                        1usize,
                    >("GetMeasureFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMeasureFunction", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeData(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutNodeData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutNodeData,
                        >,
                        1usize,
                    >("GetNodeData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNodeData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutNodeData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOwner(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >,
                        1usize,
                    >("GetOwner")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOwner", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleData(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutStyleData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutStyleData,
                        >,
                        1usize,
                    >("GetStyleData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStyleData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutStyleData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypedConfigDataRef<T>(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
        _cordl_type: crate::UnityEngine::UIElements::Layout::LayoutConfigDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
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
                            crate::UnityEngine::UIElements::Layout::LayoutHandle,
                            crate::UnityEngine::UIElements::Layout::LayoutConfigDataType,
                        ),
                        quest_hook::libil2cpp::ByRefMut<T>,
                        2usize,
                    >("GetTypedConfigDataRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypedConfigDataRef", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            cordl_method_info.invoke_unchecked(self, (handle, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypedNodeDataRef<T>(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
        _cordl_type: crate::UnityEngine::UIElements::Layout::LayoutNodeDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
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
                            crate::UnityEngine::UIElements::Layout::LayoutHandle,
                            crate::UnityEngine::UIElements::Layout::LayoutNodeDataType,
                        ),
                        quest_hook::libil2cpp::ByRefMut<T>,
                        2usize,
                    >("GetTypedNodeDataRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypedNodeDataRef", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            cordl_method_info.invoke_unchecked(self, (handle, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMeasureFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::Layout::LayoutHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetMeasureFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetMeasureFunction", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (handle, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOwner(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::Layout::LayoutHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetOwner")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOwner", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (handle, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        manager: i32,
        nodes: crate::UnityEngine::UIElements::Layout::LayoutDataStore,
        configs: crate::UnityEngine::UIElements::Layout::LayoutDataStore,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            crate::UnityEngine::UIElements::Layout::LayoutDataStore,
                            crate::UnityEngine::UIElements::Layout::LayoutDataStore,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (manager, nodes, configs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsValid", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
