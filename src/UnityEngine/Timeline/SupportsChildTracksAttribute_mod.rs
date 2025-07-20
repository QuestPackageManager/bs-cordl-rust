#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SupportsChildTracksAttribute {
    __cordl_parent: crate::System::Attribute,
    pub childType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub levels: i32,
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "SupportsChildTracksAttribute";
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
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    pub fn New(
        childType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        levels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (childType, levels))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        childType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        levels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::SupportsChildTracksAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::SupportsChildTracksAttribute as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (childType, levels))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
