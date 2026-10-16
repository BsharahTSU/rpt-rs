//! Drawing shapes end at rendered section instances, not at a sum of design heights.
use crate::Formatter;
use rpt_model::{DrawingShape, Twips};
use rpt_pages::DrawOp;

#[derive(Clone)]
pub(crate) struct SpanShape {
    end: String,
    bottom: i32,
    extend: bool,
    op_index: usize,
    prototype: DrawOp,
}
fn set_bottom(op: &mut DrawOp, bottom: i32) {
    match op {
        DrawOp::Rect(r) => r.bounds.height = Twips((bottom - r.bounds.top.0).max(0)),
        DrawOp::Line(l) => l.to.y = Twips(bottom.max(l.from.y.0)),
        _ => {}
    }
}
impl Formatter<'_> {
    pub(crate) fn register_span(&mut self, section: &str, shape: &DrawingShape, op_index: usize) {
        let flat: Vec<_> = self
            .report
            .report_definition
            .areas
            .iter()
            .flat_map(|a| &a.sections)
            .collect();
        let Some(start) = flat.iter().position(|s| s.name == section) else {
            return;
        };
        let end = usize::from(shape.end_section_index);
        if end <= start || end >= flat.len() {
            return;
        }
        let prototype = self.cur.ops[op_index].clone();
        // A repeated group header opens the next fragment of the same frame.
        self.shape_spans.retain(|s| {
            let same = s.prototype.source().and_then(|r| r.object_name.as_ref())
                == prototype.source().and_then(|r| r.object_name.as_ref());
            if same {
                if let Some(DrawOp::Rect(r)) = self.cur.ops.get_mut(s.op_index) {
                    if r.bounds.height.0 == 0 {
                        r.stroke = None;
                        r.fill = None;
                    }
                }
            }
            !same
        });
        self.shape_spans.push(SpanShape {
            end: flat[end].name.clone(),
            bottom: shape.bottom.0,
            extend: shape.extend_to_bottom_of_section,
            op_index,
            prototype,
        });
    }
    pub(crate) fn end_section_spans(&mut self, name: &str, top: i32, height: i32, design: i32) {
        self.shape_spans.retain(|s| {
            if s.end != name {
                return true;
            }
            let offset = if s.extend {
                height
            } else {
                s.bottom + (height - design).max(0)
            };
            if let Some(op) = self.cur.ops.get_mut(s.op_index) {
                set_bottom(op, top + offset);
            }
            false
        });
    }
    pub(crate) fn finish_spanning_page(&mut self) {
        for s in &mut self.shape_spans {
            if let Some(op) = self.cur.ops.get_mut(s.op_index) {
                set_bottom(op, self.body_bottom);
            }
        }
    }
    pub(crate) fn continue_spanning_page(&mut self) {
        for s in &mut self.shape_spans {
            let mut op = s.prototype.clone();
            match &mut op {
                DrawOp::Rect(r) => {
                    r.bounds.top = Twips(self.cursor_y);
                    r.bounds.height = Twips(0);
                }
                DrawOp::Line(l) => {
                    l.from.y = Twips(self.cursor_y);
                    l.to.y = Twips(self.cursor_y);
                }
                _ => {}
            }
            s.op_index = self.cur.ops.len();
            self.cur.ops.push(op);
        }
    }
}
