#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+InspectedDataRegistry")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct InspectedDataRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+InspectedDataRegistry")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::InspectedDataRegistry
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "InspectedDataRegistry";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+InspectedDataRegistry")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::InspectedDataRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+InspectedDataRegistry")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::InspectedDataRegistry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+InspectedDataRegistry")]
impl crate::Meta::XR::ImmersiveDebugger::InspectedDataRegistry {
    pub fn Add(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inspectedMember: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::InspectedMember,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::InspectedMember,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Add")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Add",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type, inspectedMember))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMembersForType<T>(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        filterCallback: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                T,
                quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::ValueTuple_2<
                    T,
                    quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
                >,
            >,
        >,
    >
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_3<
                                T,
                                quest_hook::libil2cpp::Gc<
                                    crate::Meta::XR::ImmersiveDebugger::DebugMember,
                                >,
                                bool,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::System::ValueTuple_2<
                                T,
                                quest_hook::libil2cpp::Gc<
                                    crate::Meta::XR::ImmersiveDebugger::DebugMember,
                                >,
                            >,
                        >,
                    >, 2usize>("GetMembersForType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMembersForType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::ValueTuple_2<
                    T,
                    quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type, filterCallback))? };
        Ok(__cordl_ret.into())
    }
    pub fn Reset() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+InspectedDataRegistry")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::ImmersiveDebugger::InspectedDataRegistry
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
