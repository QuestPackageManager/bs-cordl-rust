#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EventAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _EventId_k__BackingField: i32,
    pub _Level_k__BackingField: crate::System::Diagnostics::Tracing::EventLevel,
    pub _Keywords_k__BackingField: crate::System::Diagnostics::Tracing::EventKeywords,
    pub _Message_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Diagnostics::Tracing::EventAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics.Tracing";
    const CLASS_NAME: &'static str = "EventAttribute";
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
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::Tracing::EventAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::Tracing::EventAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl crate::System::Diagnostics::Tracing::EventAttribute {
    pub fn New(
        eventId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        eventId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::Tracing::EventAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::Tracing::EventAttribute as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_EventId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::Tracing::EventAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_EventId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::Tracing::EventAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "set_EventId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Keywords(
        &mut self,
        value: crate::System::Diagnostics::Tracing::EventKeywords,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::Tracing::EventAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Diagnostics::Tracing::EventKeywords),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Keywords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::Tracing::EventAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "set_Keywords", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Level(
        &mut self,
        value: crate::System::Diagnostics::Tracing::EventLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::Tracing::EventAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Diagnostics::Tracing::EventLevel),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Level")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::Tracing::EventAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "set_Level", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Message(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::Tracing::EventAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Message")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Diagnostics::Tracing::EventAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "set_Message", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::Tracing::EventAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
