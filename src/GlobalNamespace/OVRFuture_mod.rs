#[cfg(feature = "cordl_class_OVRFuture")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRFuture {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRFuture")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRFuture {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRFuture";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OVRFuture")]
impl std::ops::Deref for crate::GlobalNamespace::OVRFuture {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRFuture")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRFuture {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRFuture")]
impl crate::GlobalNamespace::OVRFuture {
    pub fn When(
        future: u64,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRPlugin_Result>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::System::Threading::CancellationToken),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRPlugin_Result,
                        >,
                        2usize,
                    >("When")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "When",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRPlugin_Result,
        > = unsafe { cordl_method_info.invoke_unchecked((), (future, cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn _When_g__CheckCancellationAndThrow_0_1(
        futureToCancel: u64,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("<When>g__CheckCancellationAndThrow|0_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<When>g__CheckCancellationAndThrow|0_1", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (futureToCancel, token))? };
        Ok(__cordl_ret.into())
    }
    pub fn _When_g__LogIfNotSuccess_0_0(
        value: crate::GlobalNamespace::OVRPlugin_Result,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::GlobalNamespace::OVRPlugin_Result, 2usize>(
                        "<When>g__LogIfNotSuccess|0_0",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<When>g__LogIfNotSuccess|0_0",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result =
            unsafe { cordl_method_info.invoke_unchecked((), (value, msg))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRFuture")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRFuture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
