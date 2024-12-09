#[cfg(feature = "ArgumentBase")]
#[repr(C)]
#[derive(Debug)]
pub struct ArgumentBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub description: *mut quest_hook::libil2cpp::Il2CppString,
    pub valueType: *mut crate::System::Type,
}
#[cfg(feature = "ArgumentBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ArgumentBase => ""
    ."ArgumentBase"
);
#[cfg(feature = "ArgumentBase")]
impl std::ops::Deref for crate::GlobalNamespace::ArgumentBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ArgumentBase")]
impl std::ops::DerefMut for crate::GlobalNamespace::ArgumentBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ArgumentBase")]
impl crate::GlobalNamespace::ArgumentBase {
    pub fn New(
        name: *mut quest_hook::libil2cpp::Il2CppString,
        description: *mut quest_hook::libil2cpp::Il2CppString,
        valueType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, description, valueType))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryParseWithValue(
        &mut self,
        inValue: *mut quest_hook::libil2cpp::Il2CppString,
        outError: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryParseWithValue", (inValue, outError))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        description: *mut quest_hook::libil2cpp::Il2CppString,
        valueType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, description, valueType))?;
        Ok(__cordl_ret)
    }
    pub fn get_isOptional(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOptional", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ArgumentBase")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ArgumentBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
