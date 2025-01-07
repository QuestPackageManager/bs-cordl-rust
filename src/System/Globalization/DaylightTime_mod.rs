#[cfg(feature = "System+Globalization+DaylightTime")]
#[repr(C)]
#[derive(Debug)]
pub struct DaylightTime {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _start: crate::System::DateTime,
    pub _end: crate::System::DateTime,
    pub _delta: crate::System::TimeSpan,
}
#[cfg(feature = "System+Globalization+DaylightTime")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::DaylightTime {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "DaylightTime";
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
#[cfg(feature = "System+Globalization+DaylightTime")]
impl std::ops::Deref for crate::System::Globalization::DaylightTime {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DaylightTime")]
impl std::ops::DerefMut for crate::System::Globalization::DaylightTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DaylightTime")]
impl crate::System::Globalization::DaylightTime {
    pub fn New(
        start: crate::System::DateTime,
        end: crate::System::DateTime,
        delta: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start, end, delta))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        start: crate::System::DateTime,
        end: crate::System::DateTime,
        delta: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start, end, delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Delta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_Delta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_End(&mut self) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object.invoke("get_End", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_Start", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+DaylightTime")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::DaylightTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
