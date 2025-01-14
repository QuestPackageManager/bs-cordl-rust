#[cfg(feature = "Unity+Jobs+IJobExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IJobExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Jobs::IJobExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobExtensions";
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
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl std::ops::Deref for crate::Unity::Jobs::IJobExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl std::ops::DerefMut for crate::Unity::Jobs::IJobExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl crate::Unity::Jobs::IJobExtensions {
    #[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
    pub type JobStruct_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Jobs::IJobExtensions_JobStruct_1<
        T,
    >;
    pub fn EarlyJobInit<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("EarlyJobInit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EarlyJobInit", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionData<T>() -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::IntPtr, 0usize>("GetReflectionData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetReflectionData", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Schedule<T>(
        jobData: T,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (T, crate::Unity::Jobs::JobHandle),
                crate::Unity::Jobs::JobHandle,
                2usize,
            >("Schedule")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Schedule", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method.invoke_unchecked((), (jobData, dependsOn))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Jobs::IJobExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IJobExtensions_JobStruct_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobExtensions/JobStruct`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Jobs",
                        "IJobExtensions/JobStruct`1",
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
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
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
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    #[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
    pub type ExecuteJobFunction = crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<
        T,
    >;
    pub fn Execute(
        data: quest_hook::libil2cpp::ByRefMut<T>,
        additionalPtr: crate::System::IntPtr,
        bufferRangePatchData: crate::System::IntPtr,
        ranges: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
        >,
        jobIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<T>,
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Execute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Execute", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (data, additionalPtr, bufferRangePatchData, ranges, jobIndex),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct JobStruct_1_IJobExtensions_ExecuteJobFunction<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobExtensions/JobStruct`1/ExecuteJobFunction";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Jobs",
                        "IJobExtensions/JobStruct`1/ExecuteJobFunction",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    pub fn Invoke(
        &mut self,
        data: quest_hook::libil2cpp::ByRefMut<T>,
        additionalPtr: crate::System::IntPtr,
        bufferRangePatchData: crate::System::IntPtr,
        ranges: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
        >,
        jobIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<T>,
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Invoke", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (data, additionalPtr, bufferRangePatchData, ranges, jobIndex),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
