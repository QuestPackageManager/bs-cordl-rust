#[cfg(feature = "System+Threading+Volatile")]
#[repr(C)]
#[derive(Debug)]
pub struct Volatile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Volatile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Volatile =>
    "System.Threading"."Volatile"
);
#[cfg(feature = "System+Threading+Volatile")]
impl std::ops::Deref for crate::System::Threading::Volatile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Volatile")]
impl std::ops::DerefMut for crate::System::Threading::Volatile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Volatile")]
impl crate::System::Threading::Volatile {
    #[cfg(feature = "System+Threading+Volatile+VolatileBoolean")]
    pub type VolatileBoolean = crate::System::Threading::Volatile_VolatileBoolean;
    #[cfg(feature = "System+Threading+Volatile+VolatileInt32")]
    pub type VolatileInt32 = crate::System::Threading::Volatile_VolatileInt32;
    #[cfg(feature = "System+Threading+Volatile+VolatileObject")]
    pub type VolatileObject = crate::System::Threading::Volatile_VolatileObject;
}
#[cfg(feature = "System+Threading+Volatile")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Volatile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Volatile+VolatileBoolean")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Volatile_VolatileBoolean {
    pub Value: bool,
}
#[cfg(feature = "System+Threading+Volatile+VolatileBoolean")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Volatile_VolatileBoolean =>
    "System.Threading"."Volatile/VolatileBoolean"
);
#[cfg(feature = "System+Threading+Volatile+VolatileBoolean")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Volatile_VolatileBoolean {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Volatile+VolatileBoolean")]
impl crate::System::Threading::Volatile_VolatileBoolean {}
#[cfg(feature = "System+Threading+Volatile+VolatileInt32")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Volatile_VolatileInt32 {
    pub Value: i32,
}
#[cfg(feature = "System+Threading+Volatile+VolatileInt32")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Volatile_VolatileInt32 =>
    "System.Threading"."Volatile/VolatileInt32"
);
#[cfg(feature = "System+Threading+Volatile+VolatileInt32")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Volatile_VolatileInt32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Volatile+VolatileInt32")]
impl crate::System::Threading::Volatile_VolatileInt32 {}
#[cfg(feature = "System+Threading+Volatile+VolatileObject")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Volatile_VolatileObject {
    pub Value: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Volatile+VolatileObject")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Volatile_VolatileObject =>
    "System.Threading"."Volatile/VolatileObject"
);
#[cfg(feature = "System+Threading+Volatile+VolatileObject")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Volatile_VolatileObject {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Volatile+VolatileObject")]
impl crate::System::Threading::Volatile_VolatileObject {}
