#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
#[repr(C)]
#[derive(Debug)]
pub struct XRManagementAnalytics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.Management";
    const CLASS_NAME: &'static str = "XRManagementAnalytics";
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
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl std::ops::Deref for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl std::ops::DerefMut for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl crate::UnityEngine::XR::Management::XRManagementAnalytics {
    pub const kEventBuild: &'static str = "xrmanagment_build";
    pub const kMaxEventsPerHour: i32 = 1000i32;
    pub const kMaxNumberOfElements: i32 = 1000i32;
    pub const kVendorKey: &'static str = "unity.xrmanagement";
    #[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
    pub type BuildEvent = crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent;
    pub fn Initialize() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XRManagementAnalytics_BuildEvent {
    pub buildGuid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub buildTarget: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub buildTargetGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub assigned_loaders: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.Management";
    const CLASS_NAME: &'static str = "XRManagementAnalytics/BuildEvent";
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
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
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
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
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
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
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
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
impl crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {}
