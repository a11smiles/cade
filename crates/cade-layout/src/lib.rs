//! Auto layout for frames, on Taffy.
//!
//! A frame with auto layout is a flex (or grid) container. This crate turns
//! CADE's layout properties into Taffy styles and returns child rectangles.
//! Constraints and responsive breakpoints are CANVAS backlog items.

use taffy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizing {
    Fixed(f32),
    /// Fill the container's free space ("W fill" in the properties strip).
    Fill,
    Hug,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutoLayout {
    pub direction: Direction,
    pub gap: f32,
    pub padding: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Child {
    pub width: Sizing,
    pub height: Sizing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum LayoutError {
    #[error("taffy: {0}")]
    Taffy(String),
}

fn dim(s: Sizing) -> Dimension {
    match s {
        Sizing::Fixed(v) => length(v),
        Sizing::Fill | Sizing::Hug => auto(),
    }
}

/// Lay out `children` inside a frame of `width` × `height`.
pub fn layout(frame: &AutoLayout, width: f32, height: f32, children: &[Child]) -> Result<Vec<Rect>, LayoutError> {
    let mut t: TaffyTree<()> = TaffyTree::new();
    let te = |e: taffy::TaffyError| LayoutError::Taffy(e.to_string());
    let kids: Vec<NodeId> = children
        .iter()
        .map(|c| {
            let grow_main = match frame.direction {
                Direction::Row => c.width == Sizing::Fill,
                Direction::Column => c.height == Sizing::Fill,
            };
            let stretch_cross = match frame.direction {
                Direction::Row => c.height == Sizing::Fill,
                Direction::Column => c.width == Sizing::Fill,
            };
            t.new_leaf(Style {
                size: Size {
                    width: dim(c.width),
                    height: dim(c.height),
                },
                flex_grow: if grow_main { 1.0 } else { 0.0 },
                align_self: if stretch_cross {
                    Some(AlignSelf::STRETCH)
                } else {
                    Some(AlignSelf::FLEX_START)
                },
                ..Default::default()
            })
            .map_err(te)
        })
        .collect::<Result<_, _>>()?;
    let root = t
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: match frame.direction {
                    Direction::Row => FlexDirection::Row,
                    Direction::Column => FlexDirection::Column,
                },
                gap: Size {
                    width: length(frame.gap),
                    height: length(frame.gap),
                },
                padding: taffy::Rect {
                    left: length(frame.padding),
                    right: length(frame.padding),
                    top: length(frame.padding),
                    bottom: length(frame.padding),
                },
                size: Size {
                    width: length(width),
                    height: length(height),
                },
                ..Default::default()
            },
            &kids,
        )
        .map_err(te)?;
    t.compute_layout(root, Size::MAX_CONTENT).map_err(te)?;
    kids.iter()
        .map(|k| {
            let l = t.layout(*k).map_err(te)?;
            Ok(Rect {
                x: l.location.x,
                y: l.location.y,
                w: l.size.width,
                h: l.size.height,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_with_fill_button() {
        let frame = AutoLayout {
            direction: Direction::Column,
            gap: 8.0,
            padding: 16.0,
        };
        let r = layout(
            &frame,
            300.0,
            200.0,
            &[
                Child {
                    width: Sizing::Fill,
                    height: Sizing::Fixed(46.0),
                },
                Child {
                    width: Sizing::Fixed(100.0),
                    height: Sizing::Fixed(20.0),
                },
            ],
        )
        .unwrap();
        assert_eq!(
            r[0],
            Rect {
                x: 16.0,
                y: 16.0,
                w: 268.0,
                h: 46.0
            }
        );
        assert_eq!(r[1].y, 16.0 + 46.0 + 8.0);
    }
}
