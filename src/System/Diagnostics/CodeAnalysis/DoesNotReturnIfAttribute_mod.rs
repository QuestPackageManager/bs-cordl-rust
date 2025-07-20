#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DoesNotReturnIfAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ParameterValue_k__BackingField: bool,
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+DoesNotReturnIfAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics.CodeAnalysis";
    const CLASS_NAME: &'static str = "DoesNotReturnIfAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parameterValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ParameterValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ParameterValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::CodeAnalysis::DoesNotReturnIfAttribute
                    as quest_hook::libil2cpp::Type > ::class(), "get_ParameterValue",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
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
