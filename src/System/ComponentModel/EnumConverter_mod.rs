#[cfg(feature = "System+ComponentModel+EnumConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
    pub values: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
    >,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+ComponentModel+EnumConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::EnumConverter =>
    "System.ComponentModel"."EnumConverter"
);
#[cfg(feature = "System+ComponentModel+EnumConverter")]
impl std::ops::Deref for crate::System::ComponentModel::EnumConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EnumConverter")]
impl std::ops::DerefMut for crate::System::ComponentModel::EnumConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EnumConverter")]
impl crate::System::ComponentModel::EnumConverter {
    pub fn CanConvertFrom(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        sourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanConvertFrom", (context, sourceType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanConvertTo(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanConvertTo", (context, destinationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertFrom(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ConvertFrom", (context, culture, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTo(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ConvertTo", (context, culture, value, destinationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardValues(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        > = __cordl_object.invoke("GetStandardValues", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardValuesExclusive(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetStandardValuesExclusive", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardValuesSupported(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetStandardValuesSupported", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", (context, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Comparer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IComparer,
        > = __cordl_object.invoke("get_Comparer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnumType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_EnumType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        > = __cordl_object.invoke("get_Values", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Values(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Values", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+EnumConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ComponentModel::EnumConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
