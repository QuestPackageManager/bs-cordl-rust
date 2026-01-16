#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FindDrawInstancesJob {
    pub instancesSorted: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub drawInstances: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::DrawInstance,
    >,
    pub outDrawInstanceIndicesWriter: crate::Unity::Collections::NativeList_1_ParallelWriter<
        i32,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "FindDrawInstancesJob";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FindDrawInstancesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+FindDrawInstancesJob")]
impl crate::UnityEngine::Rendering::FindDrawInstancesJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+FindDrawInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+FindDrawInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
for crate::UnityEngine::Rendering::FindDrawInstancesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
