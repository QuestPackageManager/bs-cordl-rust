#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ISonyOnlineServicesAvailabilityModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyOnlineServicesAvailabilityModel";
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
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl std::ops::Deref for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel {
    #[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
    pub type OnlineServicesAvailability = crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability;
    pub fn GetOnlineServicesAvailabilityAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability,
                            >,
                        >,
                        0usize,
                    >("GetOnlineServicesAvailabilityAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOnlineServicesAvailabilityAsync", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
    #[default]
    Available = 0i32,
    Unavailable = 1i32,
    UnknownError = 2i32,
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyOnlineServicesAvailabilityModel/OnlineServicesAvailability";
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
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
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
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
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
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
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
