#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StructMultiKey_2<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> {
    pub Value1: T1,
    pub Value2: T2,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::StructMultiKey_2 <
    T1, T2 > => "Newtonsoft.Json.Utilities"."StructMultiKey`2<T1,T2>" < T1, T2 >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_StructMultiKey_2_1(
        &mut self,
        other: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        v1: T1,
        v2: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v1, v2),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::IEquatable_1<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2>,
    >,
> for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2>,
    > {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::IEquatable_1<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2>,
    >,
> for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2>,
    > {
        todo!()
    }
}
