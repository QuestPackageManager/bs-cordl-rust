#[cfg(feature = "ISaberMovementDataProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaberMovementDataProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISaberMovementDataProcessor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISaberMovementDataProcessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISaberMovementDataProcessor";
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
#[cfg(feature = "ISaberMovementDataProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::ISaberMovementDataProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberMovementDataProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISaberMovementDataProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberMovementDataProcessor")]
impl crate::GlobalNamespace::ISaberMovementDataProcessor {
    pub fn ProcessNewData(
        &mut self,
        newData: crate::GlobalNamespace::BladeMovementDataElement,
        prevData: crate::GlobalNamespace::BladeMovementDataElement,
        prevDataAreValid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ISaberMovementDataProcessor as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::BladeMovementDataElement,
                    crate::GlobalNamespace::BladeMovementDataElement,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessNewData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ISaberMovementDataProcessor as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessNewData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newData, prevData, prevDataAreValid))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISaberMovementDataProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISaberMovementDataProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
