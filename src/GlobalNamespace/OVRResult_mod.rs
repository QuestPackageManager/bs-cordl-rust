#[cfg(feature = "cordl_class_OVRResult")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRResult")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRResult {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRResult";
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
#[cfg(feature = "OVRResult")]
impl std::ops::Deref for crate::GlobalNamespace::OVRResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRResult")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRResult {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRResult")]
impl crate::GlobalNamespace::OVRResult {
    pub fn From_TResult_TStatus1<TResult, TStatus>(
        result: TResult,
        status: TStatus,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRResult_2<TResult, TStatus>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TStatus: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (TResult, TStatus),
                        crate::GlobalNamespace::OVRResult_2<TResult, TStatus>,
                        2usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRResult_2<TResult, TStatus> = unsafe {
            cordl_method_info.invoke_unchecked((), (result, status))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_TStatus0<TStatus>(
        status: TStatus,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRResult_1<TStatus>>
    where
        TStatus: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (TStatus),
                        crate::GlobalNamespace::OVRResult_1<TStatus>,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRResult_1<TStatus> = unsafe {
            cordl_method_info.invoke_unchecked((), (status))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRResult")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
