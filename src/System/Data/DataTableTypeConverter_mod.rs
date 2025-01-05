#[cfg(feature = "System+Data+DataTableTypeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct DataTableTypeConverter {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::ReferenceConverter,
    >,
}
#[cfg(feature = "System+Data+DataTableTypeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataTableTypeConverter =>
    "System.Data"."DataTableTypeConverter"
);
#[cfg(feature = "System+Data+DataTableTypeConverter")]
impl std::ops::Deref for crate::System::Data::DataTableTypeConverter {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::ReferenceConverter,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTableTypeConverter")]
impl std::ops::DerefMut for crate::System::Data::DataTableTypeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTableTypeConverter")]
impl crate::System::Data::DataTableTypeConverter {
    pub fn GetPropertiesSupported(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPropertiesSupported", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataTableTypeConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataTableTypeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
