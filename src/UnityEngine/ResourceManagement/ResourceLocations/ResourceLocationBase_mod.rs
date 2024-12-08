#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationBase")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceLocationBase {
    __cordl_parent: crate::System::Object,
    pub m_Name: *mut crate::System::String,
    pub m_Id: *mut crate::System::String,
    pub m_ProviderId: *mut crate::System::String,
    pub m_Data: *mut crate::System::Object,
    pub m_DependencyHashCode: i32,
    pub m_HashCode: i32,
    pub m_Type: *mut crate::System::Type,
    pub m_Dependencies: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub m_PrimaryKey: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase =>
    "UnityEngine.ResourceManagement.ResourceLocations"."ResourceLocationBase"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationBase")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationBase")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationBase")]
impl crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase {
    pub fn ComputeDependencyHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeDependencyHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn Hash(
        &mut self,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Hash", (t))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        id: *mut crate::System::String,
        providerId: *mut crate::System::String,
        t: *mut crate::System::Type,
        dependencies: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, id, providerId, t, dependencies))?;
        Ok(__cordl_object)
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
        name: *mut crate::System::String,
        id: *mut crate::System::String,
        providerId: *mut crate::System::String,
        t: *mut crate::System::Type,
        dependencies: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, id, providerId, t, dependencies))?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Data", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = __cordl_object.invoke("get_Dependencies", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DependencyHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DependencyHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasDependencies(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDependencies", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InternalId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrimaryKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PrimaryKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProviderId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ProviderId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResourceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ResourceType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Data(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Data", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PrimaryKey(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrimaryKey", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}