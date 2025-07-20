#[cfg(feature = "System+Runtime+ConstrainedExecution+ReliabilityContractAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ReliabilityContractAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ConsistencyGuarantee_k__BackingField: crate::System::Runtime::ConstrainedExecution::Consistency,
    pub _Cer_k__BackingField: crate::System::Runtime::ConstrainedExecution::Cer,
}
#[cfg(feature = "System+Runtime+ConstrainedExecution+ReliabilityContractAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::ConstrainedExecution::ReliabilityContractAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.ConstrainedExecution";
    const CLASS_NAME: &'static str = "ReliabilityContractAttribute";
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
#[cfg(feature = "System+Runtime+ConstrainedExecution+ReliabilityContractAttribute")]
impl std::ops::Deref
for crate::System::Runtime::ConstrainedExecution::ReliabilityContractAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ConstrainedExecution+ReliabilityContractAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::ConstrainedExecution::ReliabilityContractAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ConstrainedExecution+ReliabilityContractAttribute")]
impl crate::System::Runtime::ConstrainedExecution::ReliabilityContractAttribute {
    pub fn New(
        consistencyGuarantee: crate::System::Runtime::ConstrainedExecution::Consistency,
        cer: crate::System::Runtime::ConstrainedExecution::Cer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (consistencyGuarantee, cer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        consistencyGuarantee: crate::System::Runtime::ConstrainedExecution::Consistency,
        cer: crate::System::Runtime::ConstrainedExecution::Cer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::System::Runtime::ConstrainedExecution::Consistency,
                            crate::System::Runtime::ConstrainedExecution::Cer,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (consistencyGuarantee, cer))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+ConstrainedExecution+ReliabilityContractAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::ConstrainedExecution::ReliabilityContractAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
