#[cfg(feature = "Unity+Burst+BurstRuntime")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstRuntime {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Burst+BurstRuntime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstRuntime => "Unity.Burst"
    ."BurstRuntime"
);
#[cfg(feature = "Unity+Burst+BurstRuntime")]
impl std::ops::Deref for crate::Unity::Burst::BurstRuntime {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    #[cfg(feature = "Unity+Burst+BurstRuntime+HashCode32_1")]
    pub type HashCode32_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Burst::BurstRuntime_HashCode32_1<
        T,
    >;
    #[cfg(feature = "Unity+Burst+BurstRuntime+HashCode64_1")]
    pub type HashCode64_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Burst::BurstRuntime_HashCode64_1<
        T,
    >;
    #[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
    pub type PreserveAttribute = crate::Unity::Burst::BurstRuntime_PreserveAttribute;
    pub fn GetHashCode32_0<T>() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode32", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode32_Gc1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode32", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode64_0<T>() -> quest_hook::libil2cpp::Result<i64>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode64", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode64_Gc1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode64", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF8LiteralPointer(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        byteCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUTF8LiteralPointer", (str, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashStringWithFNV1A32(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashStringWithFNV1A32", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashStringWithFNV1A64(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashStringWithFNV1A64", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAdditionalLibrary(
        pathToLibBurstGenerated: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAdditionalLibrary", (pathToLibBurstGenerated))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAdditionalLibraryInternal(
        pathToLibBurstGenerated: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAdditionalLibraryInternal", (pathToLibBurstGenerated))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        logType: i32,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        lineNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message, logType, fileName, lineNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreventRequiredAttributeStrip() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreventRequiredAttributeStrip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RuntimeLog(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        logType: i32,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        lineNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RuntimeLog", (message, logType, fileName, lineNumber))?;
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
}
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstRuntime_PreserveAttribute =>
    "Unity.Burst"."BurstRuntime/PreserveAttribute"
);
#[cfg(feature = "Unity+Burst+BurstRuntime+PreserveAttribute")]
impl std::ops::Deref for crate::Unity::Burst::BurstRuntime_PreserveAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
