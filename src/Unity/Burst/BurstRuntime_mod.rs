#[cfg(feature = "Unity+Burst+BurstRuntime")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstRuntime {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+BurstRuntime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstRuntime => "Unity.Burst"
    ."BurstRuntime"
);
#[cfg(feature = "Unity+Burst+BurstRuntime")]
impl std::ops::Deref for crate::Unity::Burst::BurstRuntime {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime")]
impl crate::Unity::Burst::BurstRuntime {
    #[cfg(feature = "Unity+Burst+BurstRuntime+HashCode64_1")]
    pub type HashCode64_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Burst::BurstRuntime_HashCode64_1<
        T,
    >;
    #[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
    pub type PreserveAttribute = crate::Unity::Burst::BurstRuntime_PreserveAttribute;
    #[cfg(feature = "Unity+Burst+BurstRuntime+HashCode32_1")]
    pub type HashCode32_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Burst::BurstRuntime_HashCode32_1<
        T,
    >;
}
#[cfg(feature = "Unity+Burst+BurstRuntime")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::BurstRuntime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode32_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BurstRuntime_HashCode32_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode32_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstRuntime_HashCode32_1 < T > =>
    "Unity.Burst"."BurstRuntime/HashCode32`1<T>" < T >
);
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode32_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstRuntime_HashCode32_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode32_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Burst::BurstRuntime_HashCode32_1<T> {}
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode64_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BurstRuntime_HashCode64_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode64_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstRuntime_HashCode64_1 < T > =>
    "Unity.Burst"."BurstRuntime/HashCode64`1<T>" < T >
);
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode64_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstRuntime_HashCode64_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime+HashCode64_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Burst::BurstRuntime_HashCode64_1<T> {}
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstRuntime_PreserveAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstRuntime_PreserveAttribute =>
    "Unity.Burst"."BurstRuntime/PreserveAttribute"
);
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
impl std::ops::Deref for crate::Unity::Burst::BurstRuntime_PreserveAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstRuntime_PreserveAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
impl crate::Unity::Burst::BurstRuntime_PreserveAttribute {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstRuntime_PreserveAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
