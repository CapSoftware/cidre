use crate::{arc, cg, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSWindowStyleMask")]
    pub StyleMask(usize)
);

impl StyleMask {
    pub const BORDERLESS: Self = Self(0);
    pub const TITLED: Self = Self(1 << 0);
    pub const CLOSABLE: Self = Self(1 << 1);
    pub const MINIATURIZABLE: Self = Self(1 << 2);
    pub const RESIZABLE: Self = Self(1 << 3);

    /// Specifies a window whose titlebar and toolbar have a unified look - that is,
    /// a continuous background. Under the titlebar and toolbar a horizontal separator
    /// line will appear.
    pub const UNIFIED_TITLE_AND_TOOLBAR: Self = Self(1 << 12);

    /// When present, the window will appear full screen. This mask is automatically toggled
    /// when \c -toggleFullScreen: is called.
    pub const FULL_SCREEN: Self = Self(1 << 14);

    /// If set, the \c contentView will consume the full size of the window; it can be combined
    /// with other window style masks, but is only respected for windows with a titlebar.
    /// Utilizing this mask opts-in to layer-backing. Utilize the \c contentLayoutRect or
    /// auto-layout \c contentLayoutGuide to layout views underneath the titlebar/toolbar area.
    pub const FULL_SIZE_CONTENT_VIEW: Self = Self(1 << 15);

    /// Only applicable for \c NSPanel (or a subclass thereof).
    pub const UTILITY_WINDOW: Self = Self(1 << 4);

    /// Only applicable for \c NSPanel (or a subclass thereof).
    pub const DOC_MODAL_WINDOW: Self = Self(1 << 6);

    /// Specifies that a panel that does not activate the owning application.
    /// Only applicable for \c NSPanel (or a subclass thereof).
    pub const NONACTIVATING_PANEL: Self = Self(1 << 7);

    /// Specifies a heads up display panel. Only applicable for \c NSPanel (or a subclass thereof).
    pub const HUD_WINDOW: Self = Self(1 << 13);
}

define_opts!(
    #[doc(alias = "NSWindowOcclusionState")]
    pub WindowOcclusionState(usize)
);

impl WindowOcclusionState {
    #[doc(alias = "NSWindowOcclusionStateVisible")]
    pub const VISIBLE: Self = Self(1 << 1);
}

define_opts!(
    #[doc(alias = "NSWindowCollectionBehavior")]
    pub CollectionBehavior(usize)
);

impl CollectionBehavior {
    pub const DEFAULT: Self = Self(0);
    pub const CAN_JOIN_ALL_SPACES: Self = Self(1 << 0);
    pub const MOVE_TO_ACTIVE_SPACE: Self = Self(1 << 1);
    pub const MANAGED: Self = Self(1 << 2);
    pub const TRANSIENT: Self = Self(1 << 3);
    pub const STATIONARY: Self = Self(1 << 4);
    pub const PARTICIPATES_IN_CYCLE: Self = Self(1 << 5);
    pub const IGNORES_CYCLE: Self = Self(1 << 6);
    pub const FULL_SCREEN_PRIMARY: Self = Self(1 << 7);
    pub const FULL_SCREEN_AUXILIARY: Self = Self(1 << 8);
    pub const FULL_SCREEN_NONE: Self = Self(1 << 9);
    pub const FULL_SCREEN_ALLOWS_TILING: Self = Self(1 << 11);
    pub const FULL_SCREEN_DISALLOWS_TILING: Self = Self(1 << 12);
    pub const PRIMARY: Self = Self(1 << 16);
    pub const AUXILIARY: Self = Self(1 << 17);
    pub const CAN_JOIN_ALL_APPLICATIONS: Self = Self(1 << 18);
}

#[doc(alias = "NSWindowTitleVisibility")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum TitleVisibility {
    Visible,
    Hidden,
}

#[doc(alias = "NSWindowToolbarStyle")]
#[repr(isize)]
pub enum ToolbarStyle {
    Automatic,
    Expanded,
    Preference,
    Unified,
    UnifiedCompact,
}

#[doc(alias = "NSWindowLevel")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct WindowLevel(pub isize);

impl WindowLevel {
    #[inline]
    pub const fn with_cg(level: cg::WindowLevel) -> Self {
        Self(level.0 as _)
    }
    pub const NORMAL: Self = Self::with_cg(cg::WindowLevel::NORMAL);
    pub const FLOATING: Self = Self::with_cg(cg::WindowLevel::FLOATING);
    pub const SUBMENU: Self = Self::with_cg(cg::WindowLevel::TORN_OFF_MENU);
    pub const TORN_OFF_MENU: Self = Self::with_cg(cg::WindowLevel::TORN_OFF_MENU);
    pub const MAIN_MENU: Self = Self::with_cg(cg::WindowLevel::MAIN_MENU);
    pub const STATUS: Self = Self::with_cg(cg::WindowLevel::STATUS);
    pub const MODAL_PANEL: Self = Self::with_cg(cg::WindowLevel::MODAL_PANEL);
    pub const POP_UP_MENU: Self = Self::with_cg(cg::WindowLevel::POP_UP_MENU);
    pub const SCREEN_SAVER: Self = Self::with_cg(cg::WindowLevel::SCREEN_SAVER);
}

#[doc(alias = "NSTitlebarSeparatorStyle")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum TitlebarSeparatorStyle {
    Automatic,
    None,
    Line,
    Shadow,
}

define_obj_type!(
   #[doc(alias = "NSWindow")]
   pub Window(ns::Responder),
   NS_WINDOW
);

impl Window {
    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(subtitle)]
    pub fn subtitle(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setSubtitle:)]
    pub fn set_subtitle(&mut self, val: &ns::String);

    #[objc::msg_send(titleVisibility)]
    pub fn title_visibility(&self) -> ns::WindowTitleVisibility;

    #[objc::msg_send(setTitleVisibility:)]
    pub fn set_title_visibility(&mut self, val: ns::WindowTitleVisibility);

    #[objc::msg_send(titlebarAppearsTransparent)]
    pub fn titlebar_appears_transparent(&self) -> bool;

    #[objc::msg_send(setTitlebarAppearsTransparent:)]
    pub fn set_titlebar_appears_transparent(&mut self, val: bool);

    #[objc::msg_send(toolbarStyle)]
    pub fn toolbar_style(&self) -> ns::WindowToolbarStyle;

    #[objc::msg_send(setToolbarStyle:)]
    pub fn set_toolbar_style(&mut self, val: ns::WindowToolbarStyle);

    #[objc::msg_send(titlebarAccessoryViewControllers)]
    pub fn titlebar_accessory_vcs(&self) -> arc::R<ns::Array<ns::TitlebarAccessoryViewController>>;

    #[objc::msg_send(setTitlebarAccessoryViewControllers:)]
    pub fn set_titlebar_accessory_vcs(
        &mut self,
        val: &ns::Array<ns::TitlebarAccessoryViewController>,
    );

    #[objc::msg_send(addTitlebarAccessoryViewController:)]
    pub fn add_titlebar_accessory_vc(&mut self, val: &ns::TitlebarAccessoryViewController);

    #[objc::msg_send(insertTitlebarAccessoryViewController:atIndex:)]
    pub fn insert_titlebar_accessory_vc_at(
        &mut self,
        val: &ns::TitlebarAccessoryViewController,
        index: isize,
    );

    #[objc::msg_send(removeTitlebarAccessoryViewControllerAtIndex:)]
    pub fn remove_titlebar_accessory_vc_at(&mut self, index: isize);

    #[objc::msg_send(occlusionState)]
    pub fn occlusion_state(&self) -> ns::WindowOcclusionState;

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> ns::Rect;

    #[objc::msg_send(setFrame:display:)]
    pub fn set_frame_display(&mut self, val: ns::Rect, display: bool);

    #[objc::msg_send(setFrame:display:animate:)]
    pub fn set_frame_display_animate(&mut self, val: ns::Rect, display: bool, animate: bool);

    #[objc::msg_send(display)]
    pub fn display(&mut self);

    #[objc::msg_send(displayIfNeeded)]
    pub fn display_if_needed(&mut self);

    #[objc::msg_send(addChildWindow:ordered:)]
    pub fn add_child_window(&mut self, window: &ns::Window, ordered: ns::WindowOrderingMode);

    #[objc::msg_send(removeChildWindow:)]
    pub fn remove_child_window(&mut self, window: &ns::Window);

    #[objc::msg_send(collectionBehavior)]
    pub fn collection_behavior(&self) -> CollectionBehavior;

    #[objc::msg_send(setCollectionBehavior:)]
    pub fn set_collection_behavior(&mut self, val: CollectionBehavior);

    #[objc::msg_send(isOnActiveSpace)]
    pub fn is_on_active_space(&self) -> bool;

    #[objc::msg_send(toggleFullScreen:)]
    pub fn toggle_full_screen(&self, sender: Option<&ns::Id>);

    #[objc::msg_send(windowController)]
    pub fn window_controller(&self) -> Option<arc::R<ns::WindowController>>;

    #[objc::msg_send(setWindowController:)]
    pub fn set_window_controller(&mut self, val: Option<&ns::WindowController>);

    #[objc::msg_send(isExcludedFromWindowsMenu)]
    pub fn is_excluded_from_windows_menu(&self) -> bool;

    #[objc::msg_send(setExcludedFromWindowsMenu:)]
    pub fn set_excluded_from_windows_menu(&mut self, val: bool);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyWindowDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: WindowDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(windowNumber)]
    pub fn window_number(&self) -> ns::Integer;

    #[objc::msg_send(styleMask)]
    pub fn style_mask(&self) -> ns::WindowStyleMask;

    #[objc::msg_send(setStyleMask:)]
    pub unsafe fn set_style_mask_throws(&mut self, val: ns::WindowStyleMask);

    pub fn set_style_mask(&mut self, val: ns::WindowStyleMask) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.set_style_mask_throws(val) })
    }

    #[objc::msg_send(hasShadow)]
    pub fn has_shadow(&self) -> bool;

    #[objc::msg_send(setHasShadow:)]
    pub fn set_has_shadow(&mut self, val: bool);

    #[objc::msg_send(opaque)]
    pub fn opaque(&self) -> bool;

    #[objc::msg_send(setOpaque:)]
    pub fn set_opaque(&mut self, val: bool);

    #[objc::msg_send(level)]
    pub fn level(&self) -> ns::WindowLevel;

    #[objc::msg_send(setLevel:)]
    pub fn set_level(&mut self, val: ns::WindowLevel);

    #[objc::msg_send(close)]
    pub fn close(&self);

    #[objc::msg_send(hidesOnDeactivate)]
    pub fn hides_on_decative(&self) -> bool;

    #[objc::msg_send(setHidesOnDeactivate:)]
    pub fn set_hides_on_decative(&mut self, val: bool);

    #[objc::msg_send(isMovable)]
    pub fn is_movable(&self) -> bool;

    #[objc::msg_send(setMovable:)]
    pub fn set_movable(&mut self, val: bool);

    #[objc::msg_send(isMovableByWindowBackground)]
    pub fn is_movable_by_window_background(&self) -> bool;

    #[objc::msg_send(setMovableByWindowBackground:)]
    pub fn set_movable_by_window_background(&mut self, val: bool);

    #[objc::msg_send(center)]
    pub fn center(&mut self);

    #[objc::msg_send(dockTile)]
    pub fn dock_tile(&self) -> arc::R<ns::DockTile>;

    #[objc::msg_send(isVisible)]
    pub fn is_visible(&self) -> bool;

    #[objc::msg_send(isKeyWindow)]
    pub fn is_key_window(&self) -> bool;

    #[objc::msg_send(isMainWindow)]
    pub fn is_main_window(&self) -> bool;

    #[objc::msg_send(canBecomeKeyWindow)]
    pub fn can_become_key_window(&self) -> bool;

    #[objc::msg_send(canBecomeMainWindow)]
    pub fn can_become_main_window(&self) -> bool;

    #[objc::msg_send(makeKeyWindow)]
    pub fn make_key_window(&self);

    #[objc::msg_send(makeMainWindow)]
    pub fn make_main_window(&self);

    #[objc::msg_send(becomeKeyWindow)]
    pub fn become_key_window(&self);

    #[objc::msg_send(resignKeyWindow)]
    pub fn resign_key_window(&self);

    #[objc::msg_send(becomeMainWindow)]
    pub fn become_main_window(&self);

    #[objc::msg_send(resignMainWindow)]
    pub fn resign_main_window(&self);

    #[objc::msg_send(backingScaleFactor)]
    pub fn backing_scale_factor(&self) -> cg::Float;

    #[objc::msg_send(isMiniaturized)]
    pub fn is_miniaturized(&self) -> bool;
}

/// ns::ViewController Support
impl Window {
    #[objc::msg_send(contentViewController)]
    pub fn content_vc(&self) -> Option<arc::R<ns::ViewController>>;

    #[objc::msg_send(setContentViewController:)]
    pub fn set_content_vc(&mut self, val: Option<&ns::ViewController>);

    #[objc::msg_send(windowWithContentViewController:)]
    pub fn with_content_vc(vc: &ns::ViewController) -> arc::R<Self>;
}

/// Toolbar Support
impl Window {
    #[objc::msg_send(toolbar)]
    pub fn toolbar(&self) -> Option<arc::R<ns::Toolbar>>;

    #[objc::msg_send(setToolbar:)]
    pub fn set_toolbar(&mut self, val: Option<&ns::Toolbar>);

    #[objc::msg_send(toggleToolbarShown:)]
    pub fn toggle_toolbar_shown(&mut self, sender: Option<&ns::Id>);
}

#[objc::protocol(NSWindowDelegate)]
pub trait WindowDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(windowShouldClose:)]
    fn window_should_close(&mut self, sender: &ns::Window) -> bool;

    #[objc::optional]
    #[objc::msg_send(windowWillResize:toSize:)]
    fn window_will_resize(&mut self, sender: &ns::Window, to_size: cg::Size) -> cg::Size;
}

define_obj_type!(
    pub AnyWindowDelegate(ns::Id)
);

impl WindowDelegate for AnyWindowDelegate {}

impl ns::AnimatablePropContainer for Window {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_WINDOW: &'static objc::Class<Window>;
}

pub mod notifications {
    use crate::ns;

    #[doc(alias = "NSWindowDidChangeOcclusionStateNotification")]
    pub fn did_change_occlusion_state() -> &'static ns::NotificationName {
        unsafe { NSWindowDidChangeOcclusionStateNotification }
    }

    unsafe extern "C" {
        static NSWindowDidChangeOcclusionStateNotification: &'static ns::NotificationName;
    }
}
