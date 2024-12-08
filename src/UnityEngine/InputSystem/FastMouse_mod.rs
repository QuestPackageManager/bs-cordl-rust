#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
#[repr(C)]
#[derive(Debug)]
pub struct FastMouse {
    __cordl_parent: crate::UnityEngine::InputSystem::Mouse,
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::FastMouse =>
    "UnityEngine.InputSystem"."FastMouse"
);
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::FastMouse {
    type Target = crate::UnityEngine::InputSystem::Mouse;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::FastMouse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl crate::UnityEngine::InputSystem::FastMouse {
    pub const metadata: &'static str = "AutoWindowSpace;Vector2;Delta;Button;Axis;Digital;Integer;Mouse;Pointer";
    pub fn Initialize_ctrlMousebackButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlMousebackButton", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseclickCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl = __cordl_object
            .invoke("Initialize_ctrlMouseclickCount", (kIntegerLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedelta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::DeltaControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::DeltaControl = __cordl_object
            .invoke("Initialize_ctrlMousedelta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedeltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousedeltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedeltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousedeltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedeltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousedeltaright", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedeltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousedeltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedeltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousedeltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedeltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousedeltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousedisplayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl = __cordl_object
            .invoke("Initialize_ctrlMousedisplayIndex", (kIntegerLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseforwardButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlMouseforwardButton", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseleftButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlMouseleftButton", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousemiddleButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlMousemiddleButton", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousepointerId(
        &mut self,
        kDigitalLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl = __cordl_object
            .invoke("Initialize_ctrlMousepointerId", (kDigitalLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseposition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control = __cordl_object
            .invoke("Initialize_ctrlMouseposition", (kVector2Layout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousepositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousepositionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousepositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousepositiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousepress(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlMousepress", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousepressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousepressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseradius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control = __cordl_object
            .invoke("Initialize_ctrlMouseradius", (kVector2Layout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseradiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMouseradiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouseradiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMouseradiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMouserightButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlMouserightButton", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescroll(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::DeltaControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::DeltaControl = __cordl_object
            .invoke("Initialize_ctrlMousescroll", (kDeltaLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescrolldown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousescrolldown", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescrollleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousescrollleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescrollright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousescrollright", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescrollup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousescrollup", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescrollx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousescrollx", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlMousescrolly(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("Initialize_ctrlMousescrolly", (kAxisLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNextUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateEvent", (eventPtr))?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_InputSystem_LowLevel_IEventMerger_MergeForward(
        &mut self,
        currentEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        nextEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IEventMerger.MergeForward",
                (currentEventPtr, nextEventPtr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnNextUpdate",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnStateEvent",
                (eventPtr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::FastMouse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
