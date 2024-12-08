#[cfg(feature = "UnityEngine+Timeline+IPropertyCollector")]
#[repr(C)]
#[derive(Debug)]
pub struct IPropertyCollector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyCollector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::IPropertyCollector =>
    "UnityEngine.Timeline"."IPropertyCollector"
);
#[cfg(feature = "UnityEngine+Timeline+IPropertyCollector")]
impl std::ops::Deref for crate::UnityEngine::Timeline::IPropertyCollector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyCollector")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::IPropertyCollector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyCollector")]
impl crate::UnityEngine::Timeline::IPropertyCollector {
    pub fn PopActiveGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopActiveGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushActiveGameObject(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushActiveGameObject", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromClips_IEnumerable_1_0(
        &mut self,
        clips: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::AnimationClip,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromClips", (clips))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromClips_GameObject_IEnumerable_1_1(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        clips: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::AnimationClip,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromClips", (obj, clips))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromComponent(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        component: *mut crate::UnityEngine::Component,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromComponent", (obj, component))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromName_String0<T>(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromName_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromName_GameObject_String2<T>(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromName", (obj, name))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromName_GameObject_String3(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromName", (obj, name))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromName_Component_String4(
        &mut self,
        component: *mut crate::UnityEngine::Component,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromName", (component, name))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromClip_AnimationClip0(
        &mut self,
        clip: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromClip", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn AddFromClip_GameObject_AnimationClip1(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        clip: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFromClip", (obj, clip))?;
        Ok(__cordl_ret)
    }
    pub fn AddObjectProperties(
        &mut self,
        obj: *mut crate::UnityEngine::Object,
        clip: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddObjectProperties", (obj, clip))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyCollector")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::IPropertyCollector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
