#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct JobProducerTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ProducerType_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "JobProducerTypeAttribute";
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
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl std::ops::Deref for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl std::ops::DerefMut
for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    pub fn New(
        producerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (producerType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        producerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (producerType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
