#![allow(non_upper_case_globals)]
#![cfg_attr(not(have_qt), allow(unused))]
use const_field_offset::FieldOffsets;
use core::pin::Pin;
#[cfg(have_qt)]
use cpp::cpp;
use sixtyfps_corelib::abi::datastructures::{Item, ItemConsts, ItemVTable};
use sixtyfps_corelib::graphics::{HighLevelRenderingPrimitive, Rect, RenderingVariable, Resource};
use sixtyfps_corelib::input::{MouseEvent, MouseEventType};
use sixtyfps_corelib::item_rendering::CachedRenderingData;
use sixtyfps_corelib::layout::LayoutInfo;
#[cfg(feature = "rtti")]
use sixtyfps_corelib::rtti::*;
use sixtyfps_corelib::{ItemVTable_static, Property, SharedArray, SharedString, Signal};
use sixtyfps_corelib_macros::*;

#[cfg(have_qt)]
mod qttypes;

#[cfg(have_qt)]
fn to_resource(image: qttypes::QImage) -> Resource {
    let size = image.size();
    Resource::EmbeddedRgbaImage {
        width: size.width,
        height: size.height,
        data: SharedArray::from(image.data()),
    }
}

#[cfg(have_qt)]
cpp! {{
    #include <QtWidgets/QApplication>
    #include <QtWidgets/QStyle>
    #include <QtWidgets/QStyleOption>
    #include <QtWidgets/QStyleFactory>
    #include <QtGui/QPainter>

    void ensure_initialized()
    {
        static int argc  = 1;
        static char argv[] = "sixtyfps";
        static char *argv2[] = { argv };
        static QApplication app(argc, argv2);
    }
}}

#[repr(C)]
#[derive(FieldOffsets, Default, BuiltinItem)]
#[pin]
pub struct QtStyleButton {
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub width: Property<f32>,
    pub height: Property<f32>,
    pub text: Property<SharedString>,
    pub pressed: Property<bool>,
    pub clicked: Signal<()>,
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for QtStyleButton {
    fn geometry(self: Pin<&Self>) -> Rect {
        euclid::rect(
            Self::FIELD_OFFSETS.x.apply_pin(self).get(),
            Self::FIELD_OFFSETS.y.apply_pin(self).get(),
            Self::FIELD_OFFSETS.width.apply_pin(self).get(),
            Self::FIELD_OFFSETS.height.apply_pin(self).get(),
        )
    }
    fn rendering_primitive(self: Pin<&Self>) -> HighLevelRenderingPrimitive {
        #[cfg(have_qt)]
        {
            let down: bool = Self::FIELD_OFFSETS.pressed.apply_pin(self).get();
            let text: qttypes::QString =
                Self::FIELD_OFFSETS.text.apply_pin(self).get().as_str().into();
            let size: qttypes::QSize = qttypes::QSize {
                width: Self::FIELD_OFFSETS.width.apply_pin(self).get() as _,
                height: Self::FIELD_OFFSETS.height.apply_pin(self).get() as _,
            };

            let img = cpp!(unsafe [
                text as "QString",
                size as "QSize",
                down as "bool"
            ] -> qttypes::QImage as "QImage" {
                ensure_initialized();
                QImage img(size, QImage::Format_ARGB32);
                img.fill(Qt::transparent);
                QPainter p(&img);
                QStyleOptionButton option;
                option.text = std::move(text);
                option.rect = QRect(img.rect());
                if (down)
                    option.state |= QStyle::State_Sunken;
                qApp->style()->drawControl(QStyle::CE_PushButton, &option, &p, nullptr);
                return img;
            });
            return HighLevelRenderingPrimitive::Image {
                x: Self::FIELD_OFFSETS.x.apply_pin(self).get(),
                y: Self::FIELD_OFFSETS.y.apply_pin(self).get(),
                source: to_resource(img),
            };
        }
        #[cfg(not(have_qt))]
        HighLevelRenderingPrimitive::NoContents
    }

    fn rendering_variables(self: Pin<&Self>) -> SharedArray<RenderingVariable> {
        SharedArray::from(&[])
    }

    fn layouting_info(self: Pin<&Self>) -> LayoutInfo {
        #[cfg(have_qt)]
        {
            let text: qttypes::QString =
                Self::FIELD_OFFSETS.text.apply_pin(self).get().as_str().into();
            let size = cpp!(unsafe [
                text as "QString"
            ] -> qttypes::QSize as "QSize" {
                ensure_initialized();
                QStyleOptionButton option;
                option.rect = option.fontMetrics.boundingRect(text);
                option.text = std::move(text);
                return qApp->style()->sizeFromContents(QStyle::CT_PushButton, &option, option.rect.size(), nullptr);
            });
            LayoutInfo {
                min_width: size.width as f32,
                min_height: size.height as f32,
                ..LayoutInfo::default()
            }
        }
        #[cfg(not(have_qt))]
        LayoutInfo::default()
    }

    fn input_event(self: Pin<&Self>, event: MouseEvent) {
        Self::FIELD_OFFSETS.pressed.apply_pin(self).set(match event.what {
            MouseEventType::MousePressed => true,
            MouseEventType::MouseReleased => false,
            MouseEventType::MouseMoved => return,
        });
        if matches!(event.what, MouseEventType::MouseReleased) {
            Self::FIELD_OFFSETS.clicked.apply_pin(self).emit(())
        }
    }
}

impl ItemConsts for QtStyleButton {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

ItemVTable_static! { #[no_mangle] pub static QtStyleButtonVTable for QtStyleButton }

#[repr(C)]
#[derive(FieldOffsets, Default, BuiltinItem)]
#[pin]
pub struct QtStyleCheckBox {
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub width: Property<f32>,
    pub height: Property<f32>,
    pub toggled: Signal<()>,
    pub text: Property<SharedString>,
    pub checked: Property<bool>,
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for QtStyleCheckBox {
    fn geometry(self: Pin<&Self>) -> Rect {
        euclid::rect(
            Self::FIELD_OFFSETS.x.apply_pin(self).get(),
            Self::FIELD_OFFSETS.y.apply_pin(self).get(),
            Self::FIELD_OFFSETS.width.apply_pin(self).get(),
            Self::FIELD_OFFSETS.height.apply_pin(self).get(),
        )
    }
    fn rendering_primitive(self: Pin<&Self>) -> HighLevelRenderingPrimitive {
        #[cfg(have_qt)]
        {
            let checked: bool = Self::FIELD_OFFSETS.checked.apply_pin(self).get();
            let text: qttypes::QString =
                Self::FIELD_OFFSETS.text.apply_pin(self).get().as_str().into();
            let size: qttypes::QSize = qttypes::QSize {
                width: Self::FIELD_OFFSETS.width.apply_pin(self).get() as _,
                height: Self::FIELD_OFFSETS.height.apply_pin(self).get() as _,
            };

            let img = cpp!(unsafe [
                text as "QString",
                size as "QSize",
                checked as "bool"
            ] -> qttypes::QImage as "QImage" {
                ensure_initialized();
                QImage img(size, QImage::Format_ARGB32);
                img.fill(Qt::transparent);
                QPainter p(&img);
                QStyleOptionButton option;
                option.text = std::move(text);
                option.rect = QRect(img.rect());
                option.state |= checked ? QStyle::State_On : QStyle::State_Off;
                qApp->style()->drawControl(QStyle::CE_CheckBox, &option, &p, nullptr);
                return img;
            });
            return HighLevelRenderingPrimitive::Image {
                x: Self::FIELD_OFFSETS.x.apply_pin(self).get(),
                y: Self::FIELD_OFFSETS.y.apply_pin(self).get(),
                source: to_resource(img),
            };
        }
        #[cfg(not(have_qt))]
        HighLevelRenderingPrimitive::NoContents
    }

    fn rendering_variables(self: Pin<&Self>) -> SharedArray<RenderingVariable> {
        SharedArray::from(&[])
    }

    fn layouting_info(self: Pin<&Self>) -> LayoutInfo {
        #[cfg(have_qt)]
        {
            let text: qttypes::QString =
                Self::FIELD_OFFSETS.text.apply_pin(self).get().as_str().into();
            let size = cpp!(unsafe [
                text as "QString"
            ] -> qttypes::QSize as "QSize" {
                ensure_initialized();
                QStyleOptionButton option;
                option.rect = option.fontMetrics.boundingRect(text);
                option.text = std::move(text);
                return qApp->style()->sizeFromContents(QStyle::CT_PushButton, &option, option.rect.size(), nullptr);
            });
            LayoutInfo {
                min_width: size.width as f32,
                min_height: size.height as f32,
                ..LayoutInfo::default()
            }
        }
        #[cfg(not(have_qt))]
        LayoutInfo::default()
    }

    fn input_event(self: Pin<&Self>, event: MouseEvent) {
        if matches!(event.what, MouseEventType::MouseReleased) {
            Self::FIELD_OFFSETS
                .checked
                .apply_pin(self)
                .set(!Self::FIELD_OFFSETS.checked.apply_pin(self).get());
            Self::FIELD_OFFSETS.toggled.apply_pin(self).emit(())
        }
    }
}

impl ItemConsts for QtStyleCheckBox {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

ItemVTable_static! { #[no_mangle] pub static QtStyleCheckBoxVTable for QtStyleCheckBox }
