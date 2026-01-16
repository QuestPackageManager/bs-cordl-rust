#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+WatchUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct WatchUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+WatchUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::Manager::WatchUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.Manager";
    const CLASS_NAME: &'static str = "WatchUtils";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+WatchUtils")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Manager::WatchUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+WatchUtils")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Manager::WatchUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+WatchUtils")]
impl crate::Meta::XR::ImmersiveDebugger::Manager::WatchUtils {
    pub const MaxLetterCount: i32 = 64i32;
    pub fn Create(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        attribute: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::DebugMember,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Manager::Watch>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::DebugMember,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Manager::Watch,
                        >,
                        3usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Manager::Watch,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (memberInfo, instanceHandle, attribute))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatFloat(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("FormatFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatFloat", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTexture(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("RegisterTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Register_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("Register")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Register", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Register_Watch_1_ToDisplayStringSignature_i32_0<T>(
        toDisplayString: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Manager::Watch_1_ToDisplayStringSignature<
                T,
            >,
        >,
        numberOfValues: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Manager::Watch_1_ToDisplayStringSignature<
                                    T,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        2usize,
                    >("Register")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Register", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked((), (toDisplayString, numberOfValues))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+WatchUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::Manager::WatchUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
