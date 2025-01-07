#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
#[repr(C)]
#[derive(Debug)]
pub struct LocationCacheKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Location: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub m_DesiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "LocationCacheKey";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl std::ops::Deref for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    pub fn Equals_IOperationCacheKey1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_LocationCacheKey2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::LocationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        desiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (location, desiredType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        desiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (location, desiredType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    >,
> for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    >,
> for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl AsRef<crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey>
for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationCacheKey")]
impl AsMut<crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey>
for crate::UnityEngine::ResourceManagement::Util::LocationCacheKey {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey {
        unsafe { std::mem::transmute(self) }
    }
}
