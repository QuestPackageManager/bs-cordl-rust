#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HierarchyFlattenedNodeChildren {
    pub m_HierarchyFlattened: quest_hook::libil2cpp::Gc<
        crate::Unity::Hierarchy::HierarchyFlattened,
    >,
    pub m_Node: crate::Unity::Hierarchy::HierarchyNode,
    pub m_Version: i32,
    pub m_Count: i32,
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Hierarchy";
    const CLASS_NAME: &'static str = "HierarchyFlattenedNodeChildren";
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Hierarchy+HierarchyFlattenedNodeChildren")]
impl crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren {
    #[cfg(feature = "Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
    pub type Enumerator = crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator,
                        0usize,
                    >("GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfVersionChanged(
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
                    >("ThrowIfVersionChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowIfVersionChanged", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        hierarchyFlattened: quest_hook::libil2cpp::Gc<
            crate::Unity::Hierarchy::HierarchyFlattened,
        >,
        node: quest_hook::libil2cpp::ByRefMut<crate::Unity::Hierarchy::HierarchyNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Hierarchy::HierarchyFlattened,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Hierarchy::HierarchyNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (hierarchyFlattened, node))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HierarchyFlattenedNodeChildren_Enumerator {
    pub m_Enumerable: crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren,
    pub m_HierarchyFlattened: quest_hook::libil2cpp::Gc<
        crate::Unity::Hierarchy::HierarchyFlattened,
    >,
    pub m_Node: crate::Unity::Hierarchy::HierarchyNode,
    pub m_CurrentIndex: i32,
    pub m_ChildrenIndex: i32,
    pub m_ChildrenCount: i32,
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Hierarchy";
    const CLASS_NAME: &'static str = "HierarchyFlattenedNodeChildren/Enumerator";
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
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
#[cfg(feature = "cordl_class_Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Hierarchy+HierarchyFlattenedNodeChildren+Enumerator")]
impl crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren_Enumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        enumerable: crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren,
        node: crate::Unity::Hierarchy::HierarchyNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Hierarchy::HierarchyFlattenedNodeChildren,
                            crate::Unity::Hierarchy::HierarchyNode,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (enumerable, node))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Hierarchy::HierarchyNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Hierarchy::HierarchyNode,
                        >,
                        0usize,
                    >("get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Hierarchy::HierarchyNode,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
