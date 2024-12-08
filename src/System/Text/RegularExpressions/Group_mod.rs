#[cfg(feature = "System+Text+RegularExpressions+Group")]
#[repr(C)]
#[derive(Debug)]
pub struct Group {
    __cordl_parent: crate::System::Text::RegularExpressions::Capture,
    pub _caps: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _capcount: i32,
    pub _capcoll: *mut crate::System::Text::RegularExpressions::CaptureCollection,
    pub _Name_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Text+RegularExpressions+Group")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::Group =>
    "System.Text.RegularExpressions"."Group"
);
#[cfg(feature = "System+Text+RegularExpressions+Group")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::Group {
    type Target = crate::System::Text::RegularExpressions::Capture;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Group")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Group")]
impl crate::System::Text::RegularExpressions::Group {
    pub fn _ctor_String_Il2CppArray_i32_String0(
        &mut self,
        text: *mut crate::System::String,
        caps: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        capcount: i32,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text, caps, capcount, name))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Success(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Success", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String_Il2CppArray_i32_String0(
        text: *mut crate::System::String,
        caps: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        capcount: i32,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text, caps, capcount, name))?;
        Ok(__cordl_object)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Group")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::Group {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
