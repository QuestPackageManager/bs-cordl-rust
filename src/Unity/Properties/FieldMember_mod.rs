#[cfg(feature = "Unity+Properties+FieldMember")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FieldMember {
    pub m_FieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    pub _Name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "Unity+Properties+FieldMember")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::FieldMember =>
    "Unity.Properties"."FieldMember"
);
#[cfg(feature = "Unity+Properties+FieldMember")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::FieldMember {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+FieldMember")]
impl crate::Unity::Properties::FieldMember {
    pub fn GetCustomAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
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
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (fieldInfo),
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
#[cfg(feature = "Unity+Properties+FieldMember")]
impl AsRef<crate::Unity::Properties::IMemberInfo>
for crate::Unity::Properties::FieldMember {
    fn as_ref(&self) -> &crate::Unity::Properties::IMemberInfo {
        todo!()
    }
}
#[cfg(feature = "Unity+Properties+FieldMember")]
impl AsMut<crate::Unity::Properties::IMemberInfo>
for crate::Unity::Properties::FieldMember {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IMemberInfo {
        todo!()
    }
}
