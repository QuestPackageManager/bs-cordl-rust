#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct JEnumerable_1<T: quest_hook::libil2cpp::Type> {
    pub _enumerable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JEnumerable_1 < T > =>
    "Newtonsoft.Json.Linq"."JEnumerable`1<T>" < T >
);
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_JEnumerable_1_0(
        &mut self,
        other: crate::Newtonsoft::Json::Linq::JEnumerable_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (enumerable),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::IJEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::IJEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (key))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Newtonsoft::Json::Linq::IJEnumerable_1<T>>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Newtonsoft::Json::Linq::IJEnumerable_1<T>>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IEquatable_1<crate::Newtonsoft::Json::Linq::JEnumerable_1<T>>>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Newtonsoft::Json::Linq::JEnumerable_1<T>> {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IEquatable_1<crate::Newtonsoft::Json::Linq::JEnumerable_1<T>>>
for crate::Newtonsoft::Json::Linq::JEnumerable_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::Newtonsoft::Json::Linq::JEnumerable_1<T>,
    > {
        todo!()
    }
}
