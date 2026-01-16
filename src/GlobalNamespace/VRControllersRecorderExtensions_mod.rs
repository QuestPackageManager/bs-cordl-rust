#[cfg(feature = "cordl_class_VRControllersRecorderExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorderExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_VRControllersRecorderExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorderExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VRControllersRecorderExtensions";
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
#[cfg(feature = "VRControllersRecorderExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorderExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorderExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRControllersRecorderExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorderExtensions")]
impl crate::GlobalNamespace::VRControllersRecorderExtensions {
    pub fn ToVRControllersRecorderInitData(
        gameplayAdditionalInformation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayAdditionalInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_InitData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayAdditionalInformation,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::VRControllersRecorder_InitData,
                        >,
                        1usize,
                    >("ToVRControllersRecorderInitData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToVRControllersRecorderInitData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_InitData,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (gameplayAdditionalInformation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToVRControllersRecorderMode(
        playmodeOptions: crate::GlobalNamespace::PlaymodeOptions,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::VRControllersRecorder_Mode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::PlaymodeOptions),
                        crate::GlobalNamespace::VRControllersRecorder_Mode,
                        1usize,
                    >("ToVRControllersRecorderMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToVRControllersRecorderMode", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::VRControllersRecorder_Mode = unsafe {
            cordl_method_info.invoke_unchecked((), (playmodeOptions))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_VRControllersRecorderExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllersRecorderExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
