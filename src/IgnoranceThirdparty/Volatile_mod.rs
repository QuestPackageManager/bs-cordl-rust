#[cfg(feature = "IgnoranceThirdparty+Volatile")]
#[repr(C)]
#[derive(Debug)]
pub struct Volatile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IgnoranceThirdparty+Volatile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceThirdparty::Volatile =>
    "IgnoranceThirdparty"."Volatile"
);
#[cfg(feature = "IgnoranceThirdparty+Volatile")]
impl std::ops::Deref for crate::IgnoranceThirdparty::Volatile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceThirdparty+Volatile")]
impl std::ops::DerefMut for crate::IgnoranceThirdparty::Volatile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceThirdparty+Volatile")]
impl crate::IgnoranceThirdparty::Volatile {
    pub const CacheLineSize: i32 = 64i32;
    #[cfg(feature = "IgnoranceThirdparty+Volatile+PaddedLong")]
    pub type PaddedLong = crate::IgnoranceThirdparty::Volatile_PaddedLong;
}
#[cfg(feature = "IgnoranceThirdparty+Volatile")]
impl quest_hook::libil2cpp::ObjectType for crate::IgnoranceThirdparty::Volatile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceThirdparty+Volatile+PaddedLong")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Volatile_PaddedLong {
    padding: [u8; 72usize],
}
#[cfg(feature = "IgnoranceThirdparty+Volatile+PaddedLong")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceThirdparty::Volatile_PaddedLong =>
    "IgnoranceThirdparty"."Volatile/PaddedLong"
);
#[cfg(feature = "IgnoranceThirdparty+Volatile+PaddedLong")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceThirdparty::Volatile_PaddedLong {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceThirdparty+Volatile+PaddedLong")]
impl crate::IgnoranceThirdparty::Volatile_PaddedLong {
    pub fn AtomicAddAndGet(&mut self, delta: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AtomicAddAndGet",
            (delta),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtomicCompareExchange(
        &mut self,
        newValue: i64,
        comparand: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AtomicCompareExchange",
            (newValue, comparand),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtomicDecrementAndGet(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AtomicDecrementAndGet",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtomicExchange(
        &mut self,
        newValue: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AtomicExchange",
            (newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtomicIncrementAndGet(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AtomicIncrementAndGet",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAcquireFence(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadAcquireFence",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCompilerOnlyFence(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadCompilerOnlyFence",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFullFence(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadFullFence",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnfenced(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadUnfenced",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCompilerOnlyFence(
        &mut self,
        newValue: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteCompilerOnlyFence",
            (newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFullFence(
        &mut self,
        newValue: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteFullFence",
            (newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteReleaseFence(
        &mut self,
        newValue: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteReleaseFence",
            (newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnfenced(
        &mut self,
        newValue: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteUnfenced",
            (newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
