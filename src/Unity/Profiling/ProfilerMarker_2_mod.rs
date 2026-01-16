#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerMarker_2<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> {
    __cordl_phantom_TP1: std::marker::PhantomData<TP1>,
    __cordl_phantom_TP2: std::marker::PhantomData<TP2>,
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type for crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling";
    const CLASS_NAME: &'static str = "ProfilerMarker`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Profiling",
                        "ProfilerMarker`2",
                    )
                    .unwrap()
                    .make_generic::<(TP1, TP2)>()
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Return for crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker_2")]
impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2> {
    #[cfg(feature = "Unity+Profiling+ProfilerMarker_2+AutoScope")]
    pub type AutoScope = crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2>;
    pub fn Auto(
        &mut self,
        p1: TP1,
        p2: TP2,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2>,
    >
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TP1, TP2),
                        crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2>,
                        2usize,
                    >("Auto")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Auto",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> = unsafe {
            cordl_method_info.invoke_unchecked(self, (p1, p2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Begin(
        &mut self,
        p1: TP1,
        p2: TP2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TP1, TP2),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Begin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Begin",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (p1, p2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn End(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("End")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "End",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param1Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param2Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
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
            cordl_method_info.invoke_unchecked(self, (name, param1Name, param2Name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ProfilerCategory_Il2CppString1(
        &mut self,
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param1Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param2Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Profiling::ProfilerCategory,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (category, name, param1Name, param2Name))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerMarker_2_AutoScope<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> {
    __cordl_phantom_TP1: std::marker::PhantomData<TP1>,
    __cordl_phantom_TP2: std::marker::PhantomData<TP2>,
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling";
    const CLASS_NAME: &'static str = "ProfilerMarker`2/AutoScope";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Profiling",
                        "ProfilerMarker`2/AutoScope",
                    )
                    .unwrap()
                    .make_generic::<(TP1, TP2)>()
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Return
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerMarker_2+AutoScope")]
unsafe impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker_2+AutoScope")]
impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
    pub fn _ctor(
        &mut self,
        marker: crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2>,
        p1: TP1,
        p2: TP2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TP1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TP2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Profiling::ProfilerMarker_2<TP1, TP2>, TP1, TP2),
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
            cordl_method_info.invoke_unchecked(self, (marker, p1, p2))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker_2+AutoScope")]
impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IDisposable>
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker_2+AutoScope")]
impl<
    TP1: quest_hook::libil2cpp::Type,
    TP2: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IDisposable>
for crate::Unity::Profiling::ProfilerMarker_2_AutoScope<TP1, TP2> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
