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
    pub fn Read_ByRefMut0(
        location: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read_ByRefMut1(
        location: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read_ByRefMut2<T>(
        location: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_T2<T>(
        location: quest_hook::libil2cpp::ByRefMut<T>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (location, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write__cordl_bool0(
        location: quest_hook::libil2cpp::ByRefMut<bool>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (location, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_i32_1(
        location: quest_hook::libil2cpp::ByRefMut<i32>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (location, value))?;
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
pub struct Volatile_VolatileObject {
    pub Value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
