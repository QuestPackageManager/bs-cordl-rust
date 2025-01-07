#[cfg(feature = "Newtonsoft+Json+Utilities+StructMultiKey_2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "StructMultiKey`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Newtonsoft.Json.Utilities",
                        "StructMultiKey`2",
                    )
                    .unwrap()
                    .make_generic::<(T1, T2)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::Utilities::StructMultiKey_2<T1, T2> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
