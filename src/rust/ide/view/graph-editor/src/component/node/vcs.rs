//! Functionality related to visualising the version control system status of a node.

use crate::component::node as node;
use crate::prelude::*;

use enso_frp as frp;
use ensogl::application::Application;
use ensogl::data::color;
use ensogl::display::shape::*;
use ensogl::display;
use ensogl::gui::component::ShapeView;



// ==============
// === Status ===
// ==============

/// The version control system status of a node.
#[derive(Debug,Copy,Clone)]
pub enum Status {
    /// The node has not been modified.
    Unchanged,
    /// The node was added.
    Added,
    /// The node was edited.
    Edited,
}

impl Status {
    fn get_highlight_color_from_style(self, style:&StyleWatch) -> color::Lcha {
        match self {
            Status::Unchanged => style.get_color(ensogl_theme::graph_editor::node::vcs::unchanged),
            Status::Added     => style.get_color(ensogl_theme::graph_editor::node::vcs::added),
            Status::Edited    => style.get_color(ensogl_theme::graph_editor::node::vcs::edited),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Unchanged
    }
}



// =======================
// === Indicator Shape ===
// =======================

/// Shape used in the status indicator.
mod status_indicator_shape {
    use super::*;

    const INDICATOR_WIDTH_OUTER : f32 = 15.0;
    const INDICATOR_WIDTH_INNER : f32 = 10.0;

    ensogl::define_shape_system! {
        (style:Style,color_rgba:Vector4<f32>) {
            let width  = Var::<Pixels>::from("input_size.x");
            let height = Var::<Pixels>::from("input_size.y");
            let width  = width  - node::PADDING.px() * 2.0;
            let height = height - node::PADDING.px() * 2.0;
            let radius = node::RADIUS.px();

            let base = Rect((&width,&height)).corners_radius(&radius);
            let outer = base.grow(INDICATOR_WIDTH_OUTER.px());
            let inner = base.grow(INDICATOR_WIDTH_INNER.px());

            (outer-inner).fill(color_rgba).into()
        }
    }
}



// ==============================
// === Status Indicator Model ===
// ==============================

/// Internal data of `StatusIndicator`.
#[derive(Clone,CloneRef,Debug)]
struct StatusIndicatorModel {
    shape : ShapeView<status_indicator_shape::Shape>,
    root  : display::object::Instance,
}

impl StatusIndicatorModel {
    fn new(logger: &Logger, app:&Application) -> Self {
        let scene  = app.display.scene();
        let shape = ShapeView::<status_indicator_shape::Shape>::new(&logger,scene);
        let root = display::object::Instance::new(&logger);
        root.add_child(&shape);
        StatusIndicatorModel{shape, root}
    }

    fn hide(&self) {
        self.shape.unset_parent();
    }

    fn show(&self) {
        self.root.add_child(&self.shape);
    }
}

impl display::Object for StatusIndicatorModel {
    fn display_object(&self) -> &display::object::Instance {
        &self.root
    }
}

// ========================
// === Status Indicator ===
// ========================

ensogl::define_endpoints! {
    Input {
        set_status     (Status),
        set_size       (Vector2),
        set_visibility (bool),
    }
    Output {
        status (Status),
    }
}

#[derive(Clone,CloneRef,Debug)]
#[allow(missing_docs)]
pub struct StatusIndicator {
        model : Rc<StatusIndicatorModel>,
    pub frp   : Frp,
}

impl StatusIndicator {
    /// Constructor.
    pub fn new(app:&Application) -> Self {
        let logger = Logger::new("status_indicator");
        let model  = Rc::new(StatusIndicatorModel::new(&logger,app));
        let frp    = Frp::new();
        Self {frp,model}.init_frp(app)
    }

    fn init_frp(self, app:&Application) -> Self {
        let frp             = &self.frp;
        let model           = &self.model;
        let network         = &frp.network;
        let indicator_color = color::Animation::new(network);

        // FIXME : StyleWatch is unsuitable here, as it was designed as an internal tool for shape
        // system (#795)
        let styles = StyleWatch::new(&app.display.scene().style_sheet);

        frp::extend! { network
            frp.source.status <+ frp.input.set_status;

            status_color <- frp.set_status.map(f!([styles](status) {
                status.get_highlight_color_from_style(&styles)
            }));
            indicator_color.target <+ status_color;

            eval indicator_color.value ((c)
                model.shape.shape.color_rgba.set(color::Rgba::from(c).into())
            );

            eval frp.input.set_size ((size)
                model.shape.shape.sprite.size.set(*size);
            );

            eval frp.input.set_visibility ([model](visible)
                if *visible { model.show() } else { model.hide() }
            );
        };

        frp.set_status.emit(Status::Unchanged);
        frp.set_visibility.emit(false);
        self
    }


}

impl display::Object for StatusIndicator {
    fn display_object(&self) -> &display::object::Instance {
        &self.model.display_object()
    }
}

impl Deref for StatusIndicator {
    type Target = Frp;
    fn deref(&self) -> &Self::Target {
        &self.frp
    }
}
