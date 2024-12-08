#[cfg(feature = "System+ArraySpec")]
#[repr(C)]
#[derive(Debug)]
pub struct ArraySpec {
    __cordl_parent: crate::System::Object,
    pub dimensions: i32,
    pub bound: bool,
}
#[cfg(feature = "System+ArraySpec")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ArraySpec => "System"."ArraySpec"
);
#[cfg(feature = "System+ArraySpec")]
impl std::ops::Deref for crate::System::ArraySpec {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ArraySpec")]
impl std::ops::DerefMut for crate::System::ArraySpec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ArraySpec")]
impl crate::System::ArraySpec {
    pub fn Resolve(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("Resolve", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        dimensions: i32,
        bound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dimensions, bound))?;
        Ok(__cordl_ret)
    }
    pub fn Append(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        dimensions: i32,
        bound: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dimensions, bound))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ArraySpec")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ArraySpec {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
