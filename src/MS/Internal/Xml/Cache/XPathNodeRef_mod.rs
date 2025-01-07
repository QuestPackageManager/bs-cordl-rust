#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XPathNodeRef {
    pub _page: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::MS::Internal::Xml::Cache::XPathNode>,
    >,
    pub _idx: i32,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.Cache";
    const CLASS_NAME: &'static str = "XPathNodeRef";
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::Return
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
impl crate::MS::Internal::Xml::Cache::XPathNodeRef {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        page: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (page, idx),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Index",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Page(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Page", ())?;
        Ok(__cordl_ret.into())
    }
}
