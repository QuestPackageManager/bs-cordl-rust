#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+RunTimeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct RunTimeUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+RunTimeUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::BuildingBlocks::RunTimeUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "RunTimeUtils";
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
#[cfg(feature = "Meta+XR+BuildingBlocks+RunTimeUtils")]
impl std::ops::Deref for crate::Meta::XR::BuildingBlocks::RunTimeUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+RunTimeUtils")]
impl std::ops::DerefMut for crate::Meta::XR::BuildingBlocks::RunTimeUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+RunTimeUtils")]
impl crate::Meta::XR::BuildingBlocks::RunTimeUtils {
    pub fn GenerateRandomString(
        _cordl_size: i32,
        includeLowercase: bool,
        includeUppercase: bool,
        includeNumeric: bool,
        includeSpecial: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, bool, bool, bool, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        5usize,
                    >("GenerateRandomString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateRandomString", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _cordl_size,
                    includeLowercase,
                    includeUppercase,
                    includeNumeric,
                    includeSpecial,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInterfaceComponent<T>(
        monoBehaviour: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>),
                        T,
                        1usize,
                    >("GetInterfaceComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInterfaceComponent", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (monoBehaviour))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+RunTimeUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::Meta::XR::BuildingBlocks::RunTimeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
