#[cfg(feature = "HoudiniEngineUnity+JSON")]
#[repr(C)]
#[derive(Debug)]
pub struct JSON {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSON => "HoudiniEngineUnity"
    ."JSON"
);
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl std::ops::Deref for crate::HoudiniEngineUnity::JSON {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::JSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl crate::HoudiniEngineUnity::JSON {
    pub fn Parse(
        aJSON: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Parse", (aJSON))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::JSON {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
