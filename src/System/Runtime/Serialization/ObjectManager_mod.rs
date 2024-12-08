#[cfg(feature = "System+Runtime+Serialization+ObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectManager {
    __cordl_parent: crate::System::Object,
    pub m_onDeserializationHandler: *mut crate::System::Runtime::Serialization::DeserializationEventHandler,
    pub m_onDeserializedHandler: *mut crate::System::Runtime::Serialization::SerializationEventHandler,
    pub m_objects: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Runtime::Serialization::ObjectHolder,
    >,
    pub m_topObject: *mut crate::System::Object,
    pub m_specialFixupObjects: *mut crate::System::Runtime::Serialization::ObjectHolderList,
    pub m_fixupCount: i64,
    pub m_selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    pub m_context: crate::System::Runtime::Serialization::StreamingContext,
}
#[cfg(feature = "System+Runtime+Serialization+ObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Serialization::ObjectManager =>
    "System.Runtime.Serialization"."ObjectManager"
);
#[cfg(feature = "System+Runtime+Serialization+ObjectManager")]
impl std::ops::Deref for crate::System::Runtime::Serialization::ObjectManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectManager")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::ObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectManager")]
impl crate::System::Runtime::Serialization::ObjectManager {
    pub fn AddObjectHolder(
        &mut self,
        holder: *mut crate::System::Runtime::Serialization::ObjectHolder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddObjectHolder", (holder))?;
        Ok(__cordl_ret)
    }
    pub fn AddOnDeserialization(
        &mut self,
        handler: *mut crate::System::Runtime::Serialization::DeserializationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOnDeserialization", (handler))?;
        Ok(__cordl_ret)
    }
    pub fn AddOnDeserialized(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOnDeserialized", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn CanCallGetType(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCallGetType", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteISerializableObject(
        &mut self,
        obj: *mut crate::System::Object,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteISerializableObject", (obj, info, context))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteObject(
        &mut self,
        holder: *mut crate::System::Runtime::Serialization::ObjectHolder,
        bObjectFullyComplete: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteObject", (holder, bObjectFullyComplete))?;
        Ok(__cordl_ret)
    }
    pub fn DoFixups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoFixups", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoNewlyRegisteredObjectFixups(
        &mut self,
        holder: *mut crate::System::Runtime::Serialization::ObjectHolder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoNewlyRegisteredObjectFixups", (holder))?;
        Ok(__cordl_ret)
    }
    pub fn DoValueTypeFixup(
        &mut self,
        memberToFix: *mut crate::System::Reflection::FieldInfo,
        holder: *mut crate::System::Runtime::Serialization::ObjectHolder,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DoValueTypeFixup", (memberToFix, holder, value))?;
        Ok(__cordl_ret)
    }
    pub fn FindObjectHolder(
        &mut self,
        objectID: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::ObjectHolder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::ObjectHolder = __cordl_object
            .invoke("FindObjectHolder", (objectID))?;
        Ok(__cordl_ret)
    }
    pub fn FindOrCreateObjectHolder(
        &mut self,
        objectID: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::ObjectHolder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::ObjectHolder = __cordl_object
            .invoke("FindOrCreateObjectHolder", (objectID))?;
        Ok(__cordl_ret)
    }
    pub fn FixupSpecialObject(
        &mut self,
        holder: *mut crate::System::Runtime::Serialization::ObjectHolder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixupSpecialObject", (holder))?;
        Ok(__cordl_ret)
    }
    pub fn GetCompletionInfo(
        &mut self,
        fixup: *mut crate::System::Runtime::Serialization::FixupHolder,
        holder: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::Serialization::ObjectHolder,
        >,
        member: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
        bThrowIfMissing: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCompletionInfo", (fixup, holder, member, bThrowIfMissing))?;
        Ok(__cordl_ret)
    }
    pub fn GetObject(
        &mut self,
        objectID: i64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetObject", (objectID))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        checkSecurity: bool,
        isCrossAppDomain: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (selector, context, checkSecurity, isCrossAppDomain))?;
        Ok(__cordl_object)
    }
    pub fn RaiseDeserializationEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseDeserializationEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn RaiseOnDeserializedEvent(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseOnDeserializedEvent", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseOnDeserializingEvent(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseOnDeserializingEvent", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn RecordArrayElementFixup(
        &mut self,
        arrayToBeFixed: i64,
        indices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        objectRequired: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecordArrayElementFixup",
                (arrayToBeFixed, indices, objectRequired),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecordDelayedFixup(
        &mut self,
        objectToBeFixed: i64,
        memberName: *mut crate::System::String,
        objectRequired: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecordDelayedFixup",
                (objectToBeFixed, memberName, objectRequired),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecordFixup(
        &mut self,
        objectToBeFixed: i64,
        member: *mut crate::System::Reflection::MemberInfo,
        objectRequired: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordFixup", (objectToBeFixed, member, objectRequired))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterFixup(
        &mut self,
        fixup: *mut crate::System::Runtime::Serialization::FixupHolder,
        objectToBeFixed: i64,
        objectRequired: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterFixup", (fixup, objectToBeFixed, objectRequired))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterObject(
        &mut self,
        obj: *mut crate::System::Object,
        objectID: i64,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        idOfContainingObj: i64,
        member: *mut crate::System::Reflection::MemberInfo,
        arrayIndex: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RegisterObject",
                (obj, objectID, info, idOfContainingObj, member, arrayIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RegisterString(
        &mut self,
        obj: *mut crate::System::String,
        objectID: i64,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        idOfContainingObj: i64,
        member: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterString", (obj, objectID, info, idOfContainingObj, member))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveObjectReference(
        &mut self,
        holder: *mut crate::System::Runtime::Serialization::ObjectHolder,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ResolveObjectReference", (holder))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        checkSecurity: bool,
        isCrossAppDomain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (selector, context, checkSecurity, isCrossAppDomain))?;
        Ok(__cordl_ret)
    }
    pub fn get_SpecialFixupObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::ObjectHolderList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::ObjectHolderList = __cordl_object
            .invoke("get_SpecialFixupObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TopObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_TopObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_TopObject(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TopObject", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::ObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}