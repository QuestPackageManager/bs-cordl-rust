#[cfg(feature = "System+Globalization+DaylightTime")]
#[repr(C)]
#[derive(Debug)]
pub struct DaylightTime {
    __cordl_parent: crate::System::Object,
    pub _start: crate::System::DateTime,
    pub _end: crate::System::DateTime,
    pub _delta: crate::System::TimeSpan,
}
#[cfg(feature = "System+Globalization+DaylightTime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::DaylightTime =>
    "System.Globalization"."DaylightTime"
);
#[cfg(feature = "System+Globalization+DaylightTime")]
impl std::ops::Deref for crate::System::Globalization::DaylightTime {
    type Target = crate::System::Object;
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
    pub fn get_End(&mut self) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object.invoke("get_End", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Delta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_Delta", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        start: crate::System::DateTime,
        end: crate::System::DateTime,
        delta: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start, end, delta))?;
        Ok(__cordl_object)
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
