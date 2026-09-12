//! Helpers that wrap a [`crate::system::FrameApp`] or [`crate::system::InputApp`]
//! in a [`crate::ui::Desktop`] so it can run as the application desktop.

/// Implements `Deref`/`DerefMut` to [`crate::ui::Desktop`] and empty default
/// impls of the usual control and event traits for a desktop wrapper.
///
/// Used by [`crate::system::FrameAppBuilder`] and [`crate::system::InputAppBuilder`].
/// `$wrapper` is a generic struct with a `base: Desktop` field. `$bound` is the
/// trait implemented by the user's app type (`FrameApp` or `InputApp`).
///
/// The empty trait impls satisfy the desktop control surface (buttons, menus,
/// app bar, and similar) with default no-op handlers. The wrapper still
/// implements paint, input, resize, and timer logic separately.
///
/// # Examples
///
/// ```rust, ignore
/// impl_app_desktop_methods!(AppDesktop, FrameApp);
/// ```
#[macro_export]
macro_rules! impl_app_desktop_methods {
    ($wrapper:ident, $bound:path) => {
        impl<T> std::ops::Deref for $wrapper<T>
        where
            T: $bound,
        {
            type Target = Desktop;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl<T> std::ops::DerefMut for $wrapper<T>
        where
            T: $bound,
        {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
        $crate::impl_app_desktop_methods!(@empty $wrapper, $bound;
            Control,
            DesktopControl,
            OnWindowRegistered,
            OnDefaultAction,
            OnFocus,
            OnExpand,
            OnSiblingSelected,
            OnThemeChanged,
            ButtonEvents,
            CheckBoxEvents,
            WindowEvents,
            GenericCommandBarEvents,
            GenericMenuEvents,
            ToolBarEvents,
            ColorPickerEvents,
            ThreeStateBoxEvents,
            RadioBoxEvents,
            PasswordEvents,
            KeySelectorEvents,
            TextFieldEvents,
            MarkdownComposerEvents,
            CustomEvents,
            GenericSelectorEvents,
            ComboBoxEvents,
            GenericDropDownListEvents,
            GenericNumericSelectorEvents,
            DatePickerEvents,
            ListBoxEvents,
            GenericListViewEvents,
            ToggleButtonEvents,
            PathFinderEvents,
            GenericTreeViewEvents,
            MarkdownEvents,
            GenericBackgroundTaskEvents,
            AccordionEvents,
            TabEvents,
            CharPickerEvents,
            GenericGraphViewEvents,
            AppBarEvents,
            TimePickerEvents,
            RichTextFieldEvents,
            GenericBufferViewEvents,
            HyperLinkEvents,
            GenericHSliderEvents,
        );
    };
    (@empty $wrapper:ident, $bound:path; $($trait_name:path),* $(,)?) => {
        $(
            impl<T> $trait_name for $wrapper<T> where T: $bound {}
        )*
    };
}

pub(crate) use impl_app_desktop_methods;