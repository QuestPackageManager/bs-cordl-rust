#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstLike {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::LowLevel::Unsafe::BurstLike
    => "Unity.Collections.LowLevel.Unsafe"."BurstLike"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::BurstLike {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike")]
impl std::ops::DerefMut for crate::Unity::Collections::LowLevel::Unsafe::BurstLike {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike")]
impl crate::Unity::Collections::LowLevel::Unsafe::BurstLike {
    #[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
    pub type SharedStatic = crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic;
    #[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic_1")]
    pub type SharedStatic_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic_1<
        T,
    >;
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::BurstLike {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstLike_SharedStatic {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic =>
    "Unity.Collections.LowLevel.Unsafe"."BurstLike/SharedStatic"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
impl crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic {
    pub fn GetOrCreateSharedStaticInternal(
        getHashCode64: i64,
        getSubHashCode64: i64,
        sizeOf: u32,
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetOrCreateSharedStaticInternal",
                (getHashCode64, getSubHashCode64, sizeOf, alignment),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BurstLike_SharedStatic_1<T: quest_hook::libil2cpp::Type> {
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic_1 < T > =>
    "Unity.Collections.LowLevel.Unsafe"."BurstLike/SharedStatic`1<T>" < T >
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstLike+SharedStatic_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic_1<T> {
    pub fn GetOrCreate<TContext>(
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContext: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::BurstLike_SharedStatic_1<
            T,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreate", (alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buffer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
