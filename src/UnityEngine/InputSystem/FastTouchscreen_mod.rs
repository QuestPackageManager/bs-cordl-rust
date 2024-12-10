#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
#[repr(C)]
#[derive(Debug)]
pub struct FastTouchscreen {
    __cordl_parent: crate::UnityEngine::InputSystem::Touchscreen,
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::FastTouchscreen =>
    "UnityEngine.InputSystem"."FastTouchscreen"
);
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::FastTouchscreen {
    type Target = crate::UnityEngine::InputSystem::Touchscreen;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::FastTouchscreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl crate::UnityEngine::InputSystem::FastTouchscreen {
    pub const metadata: &'static str = "AutoWindowSpace;Touch;Vector2;Delta;Analog;TouchPress;Button;Axis;Integer;TouchPhase;Double;Touchscreen;Pointer";
    pub fn Initialize_ctrlTouchscreendelta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendelta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendeltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendeltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendeltaright", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendeltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendeltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendeltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendisplayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreendisplayIndex", (kIntegerLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenposition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenposition", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenpositionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenpositiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpress(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenpress", (kTouchPressLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpressure(
        &mut self,
        kAnalogLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenpressure", (kAnalogLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouch(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenprimaryTouch", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdelta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdelta",
                (kDeltaLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdeltadown",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdeltaleft",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdeltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdeltaup",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdeltax",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdeltay",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdisplayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchdisplayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchindirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchindirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchphase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchphase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchposition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchposition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchpositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchpositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpress(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchpress",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchpressure",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchradius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchradius",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchradiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchradiusx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchradiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchradiusy",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchstartPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchstartPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchstartPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchstartTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchtap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchtap",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchtapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchtapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchtouchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreenprimaryTouchtouchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenradius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenradius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenradiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenradiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenradiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreenradiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch0tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch0touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch1tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch1touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch2tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch2touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch3tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch3touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch4tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch4touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch5tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch5touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch6tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch6touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch7tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch7touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch8tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch8touchId",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9", (kTouchLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9delta", (kDeltaLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9deltadown", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9deltaleft", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9deltaright",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9deltaup", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9deltax", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9deltay", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9displayIndex",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9indirectTouch",
                (kButtonLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9phase",
                (kTouchPhaseLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9position",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9positionx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9positiony", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9press",
                (kTouchPressLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9pressure", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9radius", (kVector2Layout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9radiusx", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9radiusy", (kAxisLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9startPosition",
                (kVector2Layout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9startPositionx",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9startPositiony",
                (kAxisLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9startTime",
                (kDoubleLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object
            .invoke("Initialize_ctrlTouchscreentouch9tap", (kButtonLayout, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9tapCount",
                (kIntegerLayout, parent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object
            .invoke(
                "Initialize_ctrlTouchscreentouch9touchId",
                (kIntegerLayout, parent),
            )?;
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
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::FastTouchscreen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
