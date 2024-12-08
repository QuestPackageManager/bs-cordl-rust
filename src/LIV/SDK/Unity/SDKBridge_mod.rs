#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKBridge {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKBridge => "LIV.SDK.Unity"
    ."SDKBridge"
);
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKBridge {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKBridge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl crate::LIV::SDK::Unity::SDKBridge {
    #[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
    pub type SDKInjection_1<T: quest_hook::libil2cpp::Type> = crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<
        T,
    >;
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKBridge {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKBridge_SDKInjection_1<T: quest_hook::libil2cpp::Type> {
    pub active: bool,
    pub action: *mut crate::System::Action,
    pub data: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKBridge_SDKInjection_1 < T >
    => "LIV.SDK.Unity"."SDKBridge/SDKInjection`1<T>" < T >
);
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {}
