#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+TypeInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeInformation {
    __cordl_parent: crate::System::Object,
    pub fullTypeName: *mut crate::System::String,
    pub assemblyString: *mut crate::System::String,
    pub hasTypeForwardedFrom: bool,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+TypeInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::TypeInformation =>
    "System.Runtime.Serialization.Formatters.Binary"."TypeInformation"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+TypeInformation")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::TypeInformation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+TypeInformation")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::TypeInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+TypeInformation")]
impl crate::System::Runtime::Serialization::Formatters::Binary::TypeInformation {
    pub fn New(
        fullTypeName: *mut crate::System::String,
        assemblyString: *mut crate::System::String,
        hasTypeForwardedFrom: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fullTypeName, assemblyString, hasTypeForwardedFrom))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        fullTypeName: *mut crate::System::String,
        assemblyString: *mut crate::System::String,
        hasTypeForwardedFrom: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fullTypeName, assemblyString, hasTypeForwardedFrom))?;
        Ok(__cordl_ret)
    }
    pub fn get_AssemblyString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssemblyString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FullTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasTypeForwardedFrom(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasTypeForwardedFrom", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+TypeInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::TypeInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
