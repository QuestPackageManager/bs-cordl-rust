#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct FixupHolder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_id: i64,
    pub m_fixupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_fixupType: i32,
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::FixupHolder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "FixupHolder";
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
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl std::ops::Deref for crate::System::Runtime::Serialization::FixupHolder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::FixupHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl crate::System::Runtime::Serialization::FixupHolder {
    pub fn New(
        id: i64,
        fixupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fixupType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, fixupInfo, fixupType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        id: i64,
        fixupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fixupType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (id, fixupInfo, fixupType))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::FixupHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
