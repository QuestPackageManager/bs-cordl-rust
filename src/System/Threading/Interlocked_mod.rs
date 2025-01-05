#[cfg(feature = "System+Threading+Interlocked")]
#[repr(C)]
#[derive(Debug)]
pub struct Interlocked {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Threading+Interlocked")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Interlocked =>
    "System.Threading"."Interlocked"
);
#[cfg(feature = "System+Threading+Interlocked")]
impl std::ops::Deref for crate::System::Threading::Interlocked {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Interlocked")]
impl std::ops::DerefMut for crate::System::Threading::Interlocked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Interlocked")]
impl crate::System::Threading::Interlocked {
    pub fn Add_i32_0(
        location1: quest_hook::libil2cpp::ByRefMut<i32>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_i64_1(
        location1: quest_hook::libil2cpp::ByRefMut<i64>,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_ByRefMut_ByRefMut_ByRefMut2(
        location1: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        comparand: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_Gc_Gc3(
        location1: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_IntPtr_IntPtr6(
        location1: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        value: crate::System::IntPtr,
        comparand: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_T_T8<T>(
        location1: quest_hook::libil2cpp::ByRefMut<T>,
        value: T,
        comparand: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_f32_f32_4(
        location1: quest_hook::libil2cpp::ByRefMut<f32>,
        value: f32,
        comparand: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_f64_f64_7(
        location1: quest_hook::libil2cpp::ByRefMut<f64>,
        value: f64,
        comparand: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_i32_i32_0(
        location1: quest_hook::libil2cpp::ByRefMut<i32>,
        value: i32,
        comparand: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_i32_i32_ByRefMut1(
        location1: quest_hook::libil2cpp::ByRefMut<i32>,
        value: i32,
        comparand: i32,
        succeeded: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand, succeeded))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareExchange_i64_i64_5(
        location1: quest_hook::libil2cpp::ByRefMut<i64>,
        value: i64,
        comparand: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareExchange", (location1, value, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decrement_ByRefMut0(
        location: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decrement", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decrement_ByRefMut1(
        location: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decrement", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_ByRefMut_ByRefMut1(
        location1: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_Gc2(
        location1: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_IntPtr5(
        location1: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_T7<T>(
        location1: quest_hook::libil2cpp::ByRefMut<T>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_f32_3(
        location1: quest_hook::libil2cpp::ByRefMut<f32>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_f64_6(
        location1: quest_hook::libil2cpp::ByRefMut<f64>,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_i32_0(
        location1: quest_hook::libil2cpp::ByRefMut<i32>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exchange_i64_4(
        location1: quest_hook::libil2cpp::ByRefMut<i64>,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exchange", (location1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Increment_ByRefMut0(
        location: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Increment", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Increment_ByRefMut1(
        location: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Increment", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemoryBarrier() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemoryBarrier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Read(
        location: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (location))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Interlocked")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Interlocked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
