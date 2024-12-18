#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DoesNotReturnIfAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ParameterValue_k__BackingField: bool,
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute =>
    "System.Diagnostics.CodeAnalysis"."DoesNotReturnIfAttribute"
);
#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
impl std::ops::Deref
for crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
impl std::ops::DerefMut
for crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
impl crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute {
    pub fn New(
        parameterValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameterValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        parameterValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameterValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParameterValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ParameterValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
