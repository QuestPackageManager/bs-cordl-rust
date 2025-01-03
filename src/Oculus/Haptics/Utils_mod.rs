#[cfg(feature = "Oculus+Haptics+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Haptics+Utils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Utils => "Oculus.Haptics"
    ."Utils"
);
#[cfg(feature = "Oculus+Haptics+Utils")]
impl std::ops::Deref for crate::Oculus::Haptics::Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Utils")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Utils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Utils")]
impl crate::Oculus::Haptics::Utils {
    pub fn ControllerToFfiController(
        controller: crate::Oculus::Haptics::Controller,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Controller> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Controller = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ControllerToFfiController", (controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn Map(
        input: i32,
        inMin: i32,
        inMax: i32,
        outMin: i32,
        outMax: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Map", (input, inMin, inMax, outMin, outMax))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Haptics+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
