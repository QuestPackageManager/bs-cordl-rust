#[cfg(feature = "HMUI+ViewControllerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewControllerBase {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub didActivateEvent: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewControllerBase_DidActivateDelegate,
    >,
    pub didDeactivateEvent: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewControllerBase_DidDeactivateDelegate,
    >,
}
#[cfg(feature = "HMUI+ViewControllerBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewControllerBase => "HMUI"
    ."ViewControllerBase"
);
#[cfg(feature = "HMUI+ViewControllerBase")]
impl std::ops::Deref for crate::HMUI::ViewControllerBase {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerBase")]
impl std::ops::DerefMut for crate::HMUI::ViewControllerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerBase")]
impl crate::HMUI::ViewControllerBase {
    #[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
    pub type DidActivateDelegate = crate::HMUI::ViewControllerBase_DidActivateDelegate;
    #[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
    pub type DidDeactivateDelegate = crate::HMUI::ViewControllerBase_DidDeactivateDelegate;
    pub fn CallDidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CallDidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CallDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallDidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn add_didActivateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::HMUI::ViewControllerBase_DidActivateDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didActivateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didDeactivateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::HMUI::ViewControllerBase_DidDeactivateDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didDeactivateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didActivateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::HMUI::ViewControllerBase_DidActivateDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didActivateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didDeactivateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::HMUI::ViewControllerBase_DidDeactivateDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didDeactivateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ViewControllerBase")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ViewControllerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewControllerBase_DidActivateDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewControllerBase_DidActivateDelegate =>
    "HMUI"."ViewControllerBase/DidActivateDelegate"
);
#[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
impl std::ops::Deref for crate::HMUI::ViewControllerBase_DidActivateDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
impl std::ops::DerefMut for crate::HMUI::ViewControllerBase_DidActivateDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
impl crate::HMUI::ViewControllerBase_DidActivateDelegate {
    pub fn BeginInvoke(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    firstActivation,
                    addedToHierarchy,
                    screenSystemEnabling,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidActivateDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::ViewControllerBase_DidActivateDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewControllerBase_DidDeactivateDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewControllerBase_DidDeactivateDelegate
    => "HMUI"."ViewControllerBase/DidDeactivateDelegate"
);
#[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
impl std::ops::Deref for crate::HMUI::ViewControllerBase_DidDeactivateDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
impl std::ops::DerefMut for crate::HMUI::ViewControllerBase_DidDeactivateDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
impl crate::HMUI::ViewControllerBase_DidDeactivateDelegate {
    pub fn BeginInvoke(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (removedFromHierarchy, screenSystemDisabling, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ViewControllerBase+DidDeactivateDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::ViewControllerBase_DidDeactivateDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
