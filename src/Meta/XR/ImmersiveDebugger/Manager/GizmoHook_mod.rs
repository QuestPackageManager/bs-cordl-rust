#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+GizmoHook")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct GizmoHook {
    __cordl_parent: crate::Meta::XR::ImmersiveDebugger::Manager::Hook,
    pub _SetState_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    pub _GetState_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+GizmoHook")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoHook {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.Manager";
    const CLASS_NAME: &'static str = "GizmoHook";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoHook")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoHook {
    type Target = crate::Meta::XR::ImmersiveDebugger::Manager::Hook;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoHook")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoHook {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoHook")]
impl crate::Meta::XR::ImmersiveDebugger::Manager::GizmoHook {
    pub fn New(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        attribute: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
        setState: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
        getState: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (memberInfo, instanceHandle, attribute, setState, getState),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        attribute: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
        setState: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
        getState: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
                        quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
                        quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                    ), quest_hook::libil2cpp::Void, 5usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (memberInfo, instanceHandle, attribute, setState, getState),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                        0usize,
                    >("get_GetState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetState", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SetState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
                        0usize,
                    >("get_SetState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SetState", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+GizmoHook")]
impl quest_hook::libil2cpp::ObjectType for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoHook {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
