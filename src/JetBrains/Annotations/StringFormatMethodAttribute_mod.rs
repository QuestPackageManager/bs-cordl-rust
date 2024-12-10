#[cfg(feature = "JetBrains+Annotations+StringFormatMethodAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct StringFormatMethodAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _FormatParameterName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "JetBrains+Annotations+StringFormatMethodAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::StringFormatMethodAttribute => "JetBrains.Annotations"
    ."StringFormatMethodAttribute"
);
#[cfg(feature = "JetBrains+Annotations+StringFormatMethodAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::StringFormatMethodAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+StringFormatMethodAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::StringFormatMethodAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+StringFormatMethodAttribute")]
impl crate::JetBrains::Annotations::StringFormatMethodAttribute {
    pub fn New(
        formatParameterName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (formatParameterName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        formatParameterName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (formatParameterName))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FormatParameterName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_FormatParameterName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FormatParameterName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FormatParameterName", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "JetBrains+Annotations+StringFormatMethodAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::StringFormatMethodAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
