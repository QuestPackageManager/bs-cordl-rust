#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct SortJobDefer_2<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> {
    pub Data: crate::Unity::Collections::NativeList_1<T>,
    pub Comp: U,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_U: std::marker::PhantomData<U>,
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type for crate::Unity::Collections::SortJobDefer_2<T, U>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "SortJobDefer`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Unity.Collections", "SortJobDefer`2")
                .unwrap()
                .make_generic::<(T, U)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Argument for crate::Unity::Collections::SortJobDefer_2<T, U>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Parameter for crate::Unity::Collections::SortJobDefer_2<T, U>
{
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
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Returned for crate::Unity::Collections::SortJobDefer_2<T, U>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Return for crate::Unity::Collections::SortJobDefer_2<T, U>
{
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
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ThisArgument for crate::Unity::Collections::SortJobDefer_2<T, U>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    crate::Unity::Collections::SortJobDefer_2<T, U>
{
    #[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSort")]
    pub type SegmentSort = crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>;
    #[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
    pub type SegmentSortMerge = crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>;
    pub fn Schedule(
        &mut self,
        inputDeps: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Jobs::JobHandle),
                        crate::Unity::Jobs::JobHandle,
                        1usize,
                    >("Schedule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Schedule", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (inputDeps))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct SortJobDefer_2_SegmentSort<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> {
    pub DataRO: crate::Unity::Collections::NativeList_1<T>,
    pub Data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Comp: U,
    pub SegmentWidth: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_U: std::marker::PhantomData<U>,
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "SortJobDefer`2/SegmentSort";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Collections",
                "SortJobDefer`2/SegmentSort",
            )
            .unwrap()
            .make_generic::<(T, U)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
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
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Return for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
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
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSort")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSort")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSort")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Jobs::IJobParallelForDefer>
    for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForDefer {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSort")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Jobs::IJobParallelForDefer>
    for crate::Unity::Collections::SortJobDefer_2_SegmentSort<T, U>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForDefer {
        todo!()
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct SortJobDefer_2_SegmentSortMerge<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> {
    pub Data: crate::Unity::Collections::NativeList_1<T>,
    pub Comp: U,
    pub SegmentWidth: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_U: std::marker::PhantomData<U>,
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "SortJobDefer`2/SegmentSortMerge";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Collections",
                "SortJobDefer`2/SegmentSortMerge",
            )
            .unwrap()
            .make_generic::<(T, U)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
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
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Return
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
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
#[cfg(feature = "cordl_class_Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
unsafe impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type>
    crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    pub fn Execute(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> AsRef<crate::Unity::Jobs::IJob>
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+SortJobDefer_2+SegmentSortMerge")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> AsMut<crate::Unity::Jobs::IJob>
    for crate::Unity::Collections::SortJobDefer_2_SegmentSortMerge<T, U>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
