#[cfg(feature = "Mono+RuntimeStructs")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeStructs {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+RuntimeStructs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeStructs => "Mono"."RuntimeStructs"
);
#[cfg(feature = "Mono+RuntimeStructs")]
impl std::ops::Deref for crate::Mono::RuntimeStructs {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeStructs")]
impl std::ops::DerefMut for crate::Mono::RuntimeStructs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeStructs")]
impl crate::Mono::RuntimeStructs {
    #[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
    pub type GPtrArray = crate::Mono::RuntimeStructs_GPtrArray;
    #[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
    pub type GenericParamInfo = crate::Mono::RuntimeStructs_GenericParamInfo;
    #[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
    pub type MonoClass = crate::Mono::RuntimeStructs_MonoClass;
    #[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
    pub type RemoteClass = crate::Mono::RuntimeStructs_RemoteClass;
}
#[cfg(feature = "Mono+RuntimeStructs")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::RuntimeStructs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RuntimeStructs_GPtrArray {
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub len: i32,
}
#[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeStructs_GPtrArray => "Mono"
    ."RuntimeStructs/GPtrArray"
);
#[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::RuntimeStructs_GPtrArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
impl crate::Mono::RuntimeStructs_GPtrArray {}
#[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RuntimeStructs_GenericParamInfo {
    pub pklass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub name: crate::System::IntPtr,
    pub flags: u16,
    pub token: u32,
    pub constraints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeStructs_GenericParamInfo => "Mono"
    ."RuntimeStructs/GenericParamInfo"
);
#[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::RuntimeStructs_GenericParamInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
impl crate::Mono::RuntimeStructs_GenericParamInfo {}
#[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RuntimeStructs_MonoClass {}
#[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeStructs_MonoClass => "Mono"
    ."RuntimeStructs/MonoClass"
);
#[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::RuntimeStructs_MonoClass {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
impl crate::Mono::RuntimeStructs_MonoClass {}
#[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RuntimeStructs_RemoteClass {
    pub default_vtable: crate::System::IntPtr,
    pub xdomain_vtable: crate::System::IntPtr,
    pub proxy_class: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub proxy_class_name: crate::System::IntPtr,
    pub interface_count: u32,
}
#[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeStructs_RemoteClass => "Mono"
    ."RuntimeStructs/RemoteClass"
);
#[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::RuntimeStructs_RemoteClass {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
impl crate::Mono::RuntimeStructs_RemoteClass {}
