#[cfg(feature = "Unity+Properties+PropertyMember")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PropertyMember {
    pub m_PropertyInfo: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::PropertyInfo,
    >,
    pub _Name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "Unity+Properties+PropertyMember")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::PropertyMember =>
    "Unity.Properties"."PropertyMember"
);
#[cfg(feature = "Unity+Properties+PropertyMember")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::PropertyMember {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+PropertyMember")]
impl crate::Unity::Properties::PropertyMember {
    pub fn GetCustomAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Attribute,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Attribute,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCustomAttributes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (propertyInfo),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsReadOnly",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ValueType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+PropertyMember")]
impl AsRef<crate::Unity::Properties::IMemberInfo>
for crate::Unity::Properties::PropertyMember {
    fn as_ref(&self) -> &crate::Unity::Properties::IMemberInfo {
        todo!()
    }
}
#[cfg(feature = "Unity+Properties+PropertyMember")]
impl AsMut<crate::Unity::Properties::IMemberInfo>
for crate::Unity::Properties::PropertyMember {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IMemberInfo {
        todo!()
    }
}
