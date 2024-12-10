#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollViewItemsVisibilityController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _viewport: *mut crate::UnityEngine::RectTransform,
    pub _contentRectTransform: *mut crate::UnityEngine::RectTransform,
    pub _items: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::HMUI::ScrollViewItemForVisibilityController,
    >,
    pub _lastContentAnchoredPositionY: f32,
    pub _viewportWorldCorners: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub _upperItemsCornes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Tuple_2<
            *mut crate::HMUI::ScrollViewItemForVisibilityController,
            f32,
        >,
    >,
    pub _lowerItemsCornes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Tuple_2<
            *mut crate::HMUI::ScrollViewItemForVisibilityController,
            f32,
        >,
    >,
    pub _lowerLastVisibleIndex: i32,
    pub _upperLastVisibleIndex: i32,
    pub _contentMaxY: f32,
    pub _contentMinY: f32,
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScrollViewItemsVisibilityController =>
    "HMUI"."ScrollViewItemsVisibilityController"
);
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl std::ops::Deref for crate::HMUI::ScrollViewItemsVisibilityController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl std::ops::DerefMut for crate::HMUI::ScrollViewItemsVisibilityController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl crate::HMUI::ScrollViewItemsVisibilityController {
    #[cfg(feature = "HMUI+ScrollViewItemsVisibilityController+__c")]
    pub type __c = crate::HMUI::ScrollViewItemsVisibilityController___c;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibilityDownDirection(
        &mut self,
        newContentAnchoredPositionY: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibilityDownDirection", (newContentAnchoredPositionY))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibilityUpDirection(
        &mut self,
        newContentAnchoredPositionY: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibilityUpDirection", (newContentAnchoredPositionY))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::ScrollViewItemsVisibilityController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
