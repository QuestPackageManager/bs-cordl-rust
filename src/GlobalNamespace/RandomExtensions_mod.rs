#[cfg(feature = "RandomExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RandomExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::RandomExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RandomExtensions";
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
#[cfg(feature = "RandomExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::RandomExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RandomExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::RandomExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RandomExtensions")]
impl crate::GlobalNamespace::RandomExtensions {
    pub fn GetBool(
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Random>),
                        bool,
                        1usize,
                    >("GetBool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GetBool",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (random))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRange(
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Random>, f32, f32),
                        f32,
                        3usize,
                    >("GetRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRange", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (random, min, max))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RandomExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RandomExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
